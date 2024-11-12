use super::*;
use crate::build::codegen::{ BracketKind, BracketSide, CodeLine, Codeblock };

use std::collections::HashMap;
use std::borrow::Cow;

use decomp::{ cfa::CFAPrim, cfg::ControlFlowGraph, cfr::{ CFRGroup, CFRGroups } };
use inflector::Inflector;
use llvm_ir::{ Function, Module };
use llvm_ir::module::GlobalVariable;



pub struct ParsedModule<'l> {
    pub module        : &'l Module,
    pub globals       : HashMap<Name, Global>,
    pub init_function : Option<ParsedFunction<'l>>,
    pub functions     : HashMap<String, ParsedFunction<'l>>
}
#[derive(Debug)]
pub enum Global {
    NoopFunction,
    Assert(AssertHandler),
    UserFunction {
        name : String
    },
    ActionFunction {
        codeblock : String,
        action    : String,
        tags      : Vec<ActionFunctionTag>
    },
    BracketFunction {
        kind : BracketKind,
        side : BracketSide
    },
    ElseFunction,
    TempVarFunction,
    GamevalueFunction {
        kind   : String,
        target : String
    },
    SoundFunction {
        id : String
    },
    PotionFunction {
        id : String
    },
    ItemFunction {
        id : String
    },
    Constant(Value)
}
#[derive(Debug)]
pub enum AssertHandler {
    ConstantStrToString
}
#[derive(Debug)]
pub enum ActionFunctionTag {
    Value(CodeValue),
    Dynamic {
        kind          : String,
        default_value : String
    }
}

pub fn parse_module(module : &Module) -> Result<ParsedModule, Box<dyn Error>> {
    // Sanity checks
    if let Some(target_triple) = &module.target_triple {
        if (target_triple != "wasm32-unknown-unknown") {
            eprintln!("warning: Target triple of module {:?} is not {:?}. Got {:?}", module.name, "wasm32-unknown-unknown", target_triple);
        }
    } else {
        eprintln!("warning: Target triple of module {:?} is not known.", module.name);
    }

    let mut parsed = ParsedModule {
        module,
        globals       : HashMap::new(),
        init_function : None,
        functions     : HashMap::new()
    };

    // Collect user defined functions.
    for module_function in &module.functions {
        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::UserFunction { name : module_function.name.clone() });
    }

    // Collect externally linked functions.
    for module_function in &module.func_declarations {

        if (
            module_function.name == "llvm.lifetime.start.p0"
            || module_function.name == "llvm.lifetime.end.p0"
            || module_function.name == "llvm.assume"
        ) {
            parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::NoopFunction);
            continue;
        }

        let mut parts = module_function.name.split("__");
        match (parts.next()) {

            Some("DF_TRANSMUTE") => {
                parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::NoopFunction);
                continue;
            },

            Some("DF_ASSERT") => {
                if let (Some(handler), None) = (parts.next(), parts.next()) {
                    match (handler) {

                        "ConstantStrToString" => {
                            parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::Assert(AssertHandler::ConstantStrToString));
                            continue;
                        }

                        _ => { }
                    }
                }
            },

            Some("DF_ACTION") => {
                if let (Some(executor), None) = (parts.next(), parts.next()) {
                    let mut executor_parts = executor.split("_");
                    if let (Some(codeblock), Some(action)) = (executor_parts.next(), executor_parts.next()) {
                        let codeblock = linked_name_to_codeblock (codeblock );
                        let action    = linked_name_to_action    (action    );
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::ActionFunction {
                            codeblock, action,
                            tags : collect_actiontag_parts(executor_parts)
                        });
                        continue;
                    }
                }
            },

            Some("DF_BRACKET") => {
                if let (Some(typ), None) = (parts.next(), parts.next()) {
                    let mut typ_parts = typ.split("_");
                    if let (Some(kind), Some(side), None) = (typ_parts.next(), typ_parts.next(), typ_parts.next()) {
                        let kind = BracketKind::from(kind)?;
                        let side = BracketSide::from(side)?;
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::BracketFunction {
                            kind, side
                        });
                        continue;
                    }
                }
            },

            Some("DF_ELSE") => {
                if let (None) = (parts.next()) {
                    parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::ElseFunction);
                    continue;
                }
            },

            Some("DF_TEMPVAR") => {
                if let (None) = (parts.next()) {
                    parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::TempVarFunction);
                    continue;
                }
            },

            Some("DF_GAMEVALUE") => {
                if let (Some(getter), None) = (parts.next(), parts.next()) {
                    let mut getter_parts = getter.split("_");
                    if let (Some(kind), Some(target), None) = (getter_parts.next(), getter_parts.next(), getter_parts.next()) {
                        let kind   = linked_name_to_gamevalue_kind   (kind   );
                        let target = linked_name_to_gamevalue_target (target );
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::GamevalueFunction { kind, target });
                        continue;
                    }
                }
            },

            Some("DF_SOUND") => {
                if let (Some(sound), None) = (parts.next(), parts.next()) {
                    let id = linked_name_to_sound_id(sound);
                    parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::SoundFunction { id });
                    continue;
                }
            },

            Some("DF_POTION") => {
                if let (Some(potion), None) = (parts.next(), parts.next()) {
                    let mut potion_parts = potion.split("_");
                    if let (Some(id), None) = (potion_parts.next(), potion_parts.next()) {
                        let id = linked_name_to_potion_id(id);
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::PotionFunction { id });
                        continue;
                    }
                }
            },

            Some("DF_ITEM") => {
                if let (Some(item), None) = (parts.next(), parts.next()) {
                    let mut item_parts = item.split("_");
                    if let (Some(id), None) = (item_parts.next(), item_parts.next()) {
                        let id = linked_name_to_item_id(id);
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::ItemFunction { id });
                        continue;
                    }
                }
            },

            _ => { }
        }

        return Err(format!("Unrecognised externally linked function {}", module_function.name).into());
    }

    // Collect global variables.
    let mut init_function = ParsedFunction::new(None);
    for GlobalVariable { name, /*is_constant,*/ initializer : init, .. } in &module.global_vars {
        //let mut is_constant = *is_constant;
        let var = CodeValue::Variable {
            name  : format!("{}:0", name_to_global(name)),
            scope : VariableScope::Unsaved
        };
        if let Some(init) = init {
            // Handle special cases like strings.
            let value = if let Some(value) = handle_special_const(&init) {
                value
            } else { // Otherwise just create a global.
                todo!()
            };
            //let params = vec![ var.clone(), match (value.to_codevalue(&parsed, &mut init_function)) { Ok(v) => v, Err(e) => { return Err(e) } } ];
            //init_function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
            //parsed.globals.insert(name.clone(), Global::Constant(Value::ConstPtr(name.clone())));
            parsed.globals.insert(name.clone(), Global::Constant(value));
        } else {
            todo!()
            //parsed.globals.insert(name.clone(), Global::Constant(Value::CodeValue(var)));
        }
    }
    parsed.init_function = Some(init_function);

    for module_function in &module.functions {
        let parsed_function = parse_function(&parsed, module_function)?;
        parsed.functions.insert(module_function.name.clone(), parsed_function);
    }

    Ok(parsed)
}

pub fn linked_name_to_codeblock(codeblock : &str) -> String {
    let codeblock = codeblock.to_snake_case();
    match (codeblock.as_str()) {
        "select_object" => String::from("select_obj" ),
        "set_variable"  => String::from("set_var"    ),
        "if_variable"   => String::from("if_var"     ),
        _               => codeblock
    }
}
pub fn linked_name_to_action(action : &str) -> String {
    linked_name_to_actiontag_kind(&names_to_symbols(action)).replace(" ", "")
}
pub fn linked_name_to_actiontag_kind(actiontag_kind : &str) -> String {
    let mut out = String::with_capacity(actiontag_kind.len() * 2);
    let mut last_ch = ' ';
    for (i, ch) in actiontag_kind.chars().rev().enumerate() {
        out.push(ch);
        if (ch.is_uppercase() && (! last_ch.is_uppercase()) && (i + 1 != actiontag_kind.len())) { out.push(' '); }
        last_ch = ch;
    }
    out.chars().rev().collect::<String>().split(" ").enumerate().map(|(i, word)|
        if (i == 0) { Cow::Borrowed(word) } 
        else {
            let word_lc = word.to_lowercase();
            match (word_lc.as_str()) {
                "a"    | "and" | "as"   | "at"   | "but"  | "by"   | "for" | "from" | "if" | "in"   |
                "into" | "nor" | "of"   | "off"  | "on"   | "once" | "or"  | "over" | "so" | "than" |
                "that" | "to"  | "upon" | "when" | "with" | "yet"  => Cow::Owned(word_lc),
                _ => Cow::Borrowed(word)
            }
        }
    ).intersperse(Cow::Borrowed(" ")).collect::<String>()
}
pub fn linked_name_to_actiontag_value(actiontag_value : &str) -> String {
    linked_name_to_gamevalue_kind(actiontag_value).to_sentence_case()
}
pub fn collect_actiontag_parts<'l>(actiontag_parts : impl Iterator<Item = &'l str>) -> Vec<ActionFunctionTag> {
    actiontag_parts.array_chunks::<2>()
        .map(|[kind, value]| {
            let kind  = linked_name_to_actiontag_kind(kind);
            let value = linked_name_to_actiontag_value(value);
            if (value.starts_with("Dynamic")) { ActionFunctionTag::Dynamic {
                kind,
                default_value : value[8..].to_sentence_case()
            } }
            else { ActionFunctionTag::Value(CodeValue::Actiontag {
                kind,
                value,
                variable : None
            }) }
    } ).collect::<Vec<_>>()
}
pub fn linked_name_to_gamevalue_kind(gamevalue_kind : &str) -> String {
    let string = gamevalue_kind.to_title_case();
    let mut out = string.split(" ").intersperse(" ")
        .map_windows::<_, _, 3>(|[a, b, c]| if (*b == " " && a.chars().last().unwrap().is_uppercase() && c.chars().next().unwrap().is_uppercase()) { None } else { Some(*b) })
        .filter_map(|b| b)
        .collect::<String>();
    let mut split = string.split(" ");
    if let Some(part) = split.next() {
        out.insert_str(0, part);
    }
    if let Some(part) = split.last() {
        out.push_str(part);
    }
    out
}
pub fn linked_name_to_gamevalue_target(gamevalue_kind : &str) -> String {
    linked_name_to_actiontag_kind(gamevalue_kind)
}
pub fn linked_name_to_sound_id(sound_id : &str) -> String {
    sound_id.split("_").map(|part| linked_name_to_item_id(part)).intersperse(".".to_string()).collect::<String>()
}
pub fn linked_name_to_potion_id(potion_id : &str) -> String {
    linked_name_to_actiontag_kind(&names_to_symbols(&potion_id))
}
pub fn linked_name_to_item_id(item_id : &str) -> String {
    item_id.to_snake_case()
}
pub fn names_to_symbols(from : &str) -> String {
    // Yes, I know this sucks. No, I'm not going to find something better.
    from.replace("Specialcharspace"            , " ")
        .replace("Specialcharplus"             , "+")
        .replace("Specialcharminus"            , "-")
        .replace("Specialcharslash"            , "/")
        .replace("Specialcharpercent"          , "%")
        .replace("Specialcharexclamation"      , "!")
        .replace("Specialcharequals"           , "=")
        .replace("Specialcharleftbracket"      , "[")
        .replace("Specialcharrightbracket"     , "]")
        .replace("Specialcharleftangle"        , "<")
        .replace("Specialcharrightangle"       , ">")
        .replace("Specialcharleftparenthesis"  , "(")
        .replace("Specialcharrightparenthesis" , ")")
        .replace("Specialcharapostrophe"       , "'")
        .replace("Specialcharcomma"            , ",")
        .replace("Specialcharpipe"             , "|")
        .replace("Specialcharampersand"        , "&")
        .replace("Specialchartilde"            , "~")
        .replace("Specialcharcaret"            , "^")
        .replace("Specialcharcolon"            , ":")
        .replace("Specialcharperiod"           , ".")
}




pub struct ParsedFunction<'l> {
    pub function    : Option<&'l Function>,
    pub locals      : HashMap<Name, Value>,
    pub line        : CodeLine,
    pub next_temp   : usize,
    pub needs_frame : bool
}
impl<'l> ParsedFunction<'l> {
    pub fn new(function : Option<&'l Function>) -> Self { Self {
        function,
        locals      : HashMap::new(),
        line        : CodeLine::new(),
        next_temp   : 0,
        needs_frame : false
    } }
    pub fn create_temp_var_name(&mut self) -> String {
        let temp_var = self.next_temp;
        self.next_temp += 1;
        format!("local.temp.{}", temp_var)
    }
}

pub fn parse_function<'l>(module : &ParsedModule, function : &'l Function) -> Result<ParsedFunction<'l>, Box<dyn Error>> {
    let mut parsed = ParsedFunction::new(Some(function));

    for param in &function.parameters {
        parsed.locals.insert(param.name.clone(), Value::Local(name_to_local(&param.name)));
    }


    // Do the CFR magic.
    let Some(cfr) = CFAPrim::find_all(ControlFlowGraph::new(function)).map(|cfa| CFRGroups::new(&cfa)).flatten() else {
        return Err(format!("Failed to recover control flow primitives of function `{}`.", function.name).into())
    };

    // Actually parse the function.
    parse_cfr_groups(module, &mut parsed, &cfr)?;

    if (parsed.needs_frame) {

        // Create a new 'frame' which can be used to track allocated variables which need to be destroyed on function exit.
        let frame = CodeValue::Variable { name : "lldf.alloca.frame".to_string(), scope : VariableScope::Unsaved };
        parsed.line.blocks.insert(0, Codeblock::action("set_var", "+=", vec![ frame.clone() ], vec![]));
        let params = vec![
            CodeValue::Variable { name : "lldf.alloca.frame.current".to_string(), scope : VariableScope::Line },
            frame
        ];
        parsed.line.blocks.insert(1, Codeblock::action("set_var", "=", params, vec![]));

        // Add the code which destroys any allocated variables from this 'frame'.
        let params = vec![
            CodeValue::String("mem.#%var(lldf.alloca.frame.current)".to_string())
        ];
        let tags = vec![
            CodeValue::Actiontag { kind : "Match Requirement".to_string(), value : "Any part of name".to_string(), variable : None },
            CodeValue::Actiontag { kind : "Ignore Case".to_string(), value : "False".to_string(), variable : None }
        ];
        let block = Codeblock::action("set_var", "PurgeVars", params, tags);
        parsed.line.blocks.push(block); // TODO: Add after every Return or ReturnNTimes codeblock.

    }

    Ok(parsed)
}




pub fn parse_cfr_groups(module : &ParsedModule, function : &mut ParsedFunction, groups : &CFRGroups) -> Result<(), Box<dyn Error>> {
    for group in &groups.groups {
        parse_cfr_group(module, function, group)?;
    }
    Ok(())
}

pub fn parse_cfr_group(module : &ParsedModule, function : &mut ParsedFunction, group : &CFRGroup) -> Result<(), Box<dyn Error>> { match (group) {

    CFRGroup::Block(name) => parse_block(module, function, name),

    CFRGroup::PreconditionLoop { cond, body } => todo!("precondition-loop {} {}", cond, body),

    CFRGroup::PostconditionLoop { cond } => todo!("postcondition-loop {}", cond),

    CFRGroup::OnewayConditional { cond, body } => todo!("oneway-conditional {} {}", cond, body),

    CFRGroup::OnewayReturnConditional { cond, body } => todo!("oneway-return-conditional {} {}", cond, body),

    CFRGroup::TwowayConditional { cond, body_true, body_false } => todo!("twoway-conditional {} {} {}", cond, body_true, body_false),

} }




pub fn parse_block(module : &ParsedModule, function : &mut ParsedFunction, block : &Name) -> Result<(), Box<dyn Error>> {
    let block = function.function.unwrap().basic_blocks.iter().find(|bb| &bb.name == block).unwrap();
    for instr in &block.instrs {
        parse_instr(module, function, instr)?;
    }
    // TODO: Terminator? This might not be needed if it's part of CFR groups.
    Ok(())
}
