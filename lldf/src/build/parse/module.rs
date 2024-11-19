use super::*;
use crate::build::codegen::{ BracketKind, BracketSide, CodeblockBlock };

use std::collections::HashMap;
use std::borrow::Cow;

use inflector::Inflector;
use llvm_ir::Module;
use llvm_ir::module::GlobalVariable;
use llvm_ir::types::Types;



pub struct ParsedModule<'l> {
    pub module    : &'l Module,
    pub globals   : HashMap<Name, Global>,
    pub types     : &'l Types,
    pub functions : Vec<ParsedFunction>
}
#[derive(Debug)]
pub enum Global {
    NoopFunction,
    OpaqueTransmuteFunction,
    Assert(AssertHandler),
    UserFunction {
        name : String
    },
    ActionFunction {
        codeblock : String,
        action    : String,
        tags      : Vec<ActionFunctionTag>
    },
    ProcessFunction(ProcessHandler),
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
    ParticleFunction {
        id               : String,
        motion           : bool,
        motion_variation : bool,
        colour           : bool,
        colour_variation : bool,
        opacity          : bool,
        material         : bool,
        size             : bool,
        size_variation   : bool,
        roll             : bool,
        fade_colour      : bool
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
    /// Fixes empty strings being optimised away.
    ConstantStrToString,
    /// Fixes `f64` being represented as `u64` under certain conditions.
    NoOptF64
}
#[derive(Debug)]
pub enum ProcessHandler {
    Spawn,
    Join
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
        globals   : HashMap::new(),
        types     : &module.types,
        functions : Vec::new()
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
            || module_function.name == "llvm.experimental.noalias.scope.decl"
        ) {
            parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::NoopFunction);
            continue;
        }

        let mut parts = module_function.name.split("__");
        match (parts.next()) {

            Some("DF_TRANSMUTE") => {
                parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::OpaqueTransmuteFunction);
                continue;
            },

            Some("DF_ASSERT") => {
                if let (Some(handler), None) = (parts.next(), parts.next()) {
                    match (handler) {

                        "ConstantStrToString" => {
                            parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::Assert(AssertHandler::ConstantStrToString));
                            continue;
                        },

                        "NoOptF64" => {
                            parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::Assert(AssertHandler::NoOptF64));
                            continue;
                        },

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

            Some("DF_PROCESS") => {
                if let (Some(handler), None) = (parts.next(), parts.next()) { match (handler) {
                    "Spawn" => {
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::ProcessFunction(ProcessHandler::Spawn));
                        continue;
                    },
                    "Join" => {
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::ProcessFunction(ProcessHandler::Join));
                        continue;
                    },
                    _ => { }
                } }
            }

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

            Some("DF_PARTICLE") => {
                if let (Some(particle), None) = (parts.next(), parts.next()) {
                    let mut particle_parts = particle.split("_");
                    if let Some(id) = particle_parts.next() {
                        let     id               = linked_name_to_particle_id(id);
                        let mut motion           = false;
                        let mut motion_variation = false;
                        let mut colour           = false;
                        let mut colour_variation = false;
                        let mut opacity          = false;
                        let mut material         = false;
                        let mut size             = false;
                        let mut size_variation   = false;
                        let mut roll             = false;
                        let mut fade_colour      = false;
                        for part in particle_parts { match (part) {
                            "Motion"          => { motion           = true; },
                            "MotionVariation" => { motion_variation = true; },
                            "Color"           => { colour           = true; },
                            "ColorVariation"  => { colour_variation = true; },
                            "Opacity"         => { opacity          = true; },
                            "Material"        => { material         = true; },
                            "Size"            => { size             = true; },
                            "SizeVariation"   => { size_variation   = true; },
                            "Roll"            => { roll             = true; },
                            "FadeColor"       => { fade_colour      = true; },
                            _ => { return Err(format!("Unknown particle field {:?}", part).into()) }
                        } }
                        parsed.globals.insert(Name::Name(Box::new(module_function.name.clone())), Global::ParticleFunction { id,
                            motion, motion_variation,
                            colour, colour_variation,
                            opacity,
                            material,
                            size, size_variation,
                            roll,
                            fade_colour
                        });
                        continue;
                    }
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
    let mut init_function = ParsedFunction::new();
    for GlobalVariable { name, initializer : init, .. } in &module.global_vars {
        let var = CodeValue::Variable {
            name  : format!("{}:0", name_to_global(name)),
            scope : VariableScope::Unsaved
        };
        if let Some(init) = init {
            // Handle special cases like strings.
            let value = if let Some(value) = handle_special_const(&init) { value }
            else { // Otherwise just create a global.
                let value = parse_const(&parsed, &mut init_function, init)?.to_codevalue(&parsed, &mut init_function)?;
                init_function.line.blocks.push(Codeblock::action("set_var", "=", vec![ var, value ], vec![ ]));
                Value::Global(name_to_global(name))
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

    for module_function in &module.functions {
        parse_function(&mut parsed, module_function)?;
    }

    // Init special event handler.
    let init_var = CodeValue::Variable { name : "lldf.init".to_string(), scope : VariableScope::Unsaved };
    init_function.line.blocks.splice(0..0, [
        Codeblock::ifs("if_var", "=", true, vec![ init_var.clone(), CodeValue::Number("1".to_string()) ], vec![ ]),
        Codeblock::OPEN_COND_BRACKET,
        Codeblock::action("set_var", "=", vec![ init_var, CodeValue::Number("1".to_string()) ], vec![ ])
    ].into_iter());
    init_function.line.blocks.push(Codeblock::call_func("DF_EVENT__LLDF_PlotStart:0", vec![ ]));
    init_function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
    if let Some(player_join_function) = parsed.functions.iter_mut().find(|function| if let Some(Codeblock::Block(CodeblockBlock { block, action : Some(action), .. })) = function.line.blocks.get(0) && block == "event" && action == "Join" { true } else { false }) {
        player_join_function.line.blocks.splice(1..1, init_function.line.blocks);
    } else {
        init_function.line.blocks.insert(0, Codeblock::event("event", "Join"));
        parsed.functions.push(init_function);
    }

    // Process spawn handler.
    let mut process_spawn_function = ParsedFunction::new();
    process_spawn_function.line.blocks.push(Codeblock::process("lldf.spawn", true));
    process_spawn_function.line.blocks.push(Codeblock::call_func("%var(lldf.noopt.spawn)", vec![ ]));
    process_spawn_function.line.blocks.push(Codeblock::action("set_var", "PurgeVars", vec![
        CodeValue::String("lldf.pid_running.%var(lldf.noopt.pid)".to_string())
    ], vec![
        CodeValue::Actiontag { kind : "Match Requirement".to_string(), value : "Any part of name".to_string(), variable : None, block_override : None },
        CodeValue::Actiontag { kind : "Ignore Case".to_string(), value : "False".to_string(), variable : None, block_override : None }
    ]));
    parsed.functions.push(process_spawn_function);

    Ok(parsed)
}

pub fn propercase(string : &str, standard_decapitalisation : bool) -> String {
    let mut out = String::with_capacity(string.len() * 2);
    let mut last_ch = 'A';
    for (i, ch) in string.chars().rev().enumerate() {
        out.push(ch);
        if (ch.is_uppercase() && (! last_ch.is_uppercase()) && (i + 1 != string.len())) { out.push(' '); }
        last_ch = ch;
    }
    let out = out.chars().rev().collect::<String>();
    if (! standard_decapitalisation) { return out; }
    out.split(" ").enumerate().map(|(i, word)|
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
    propercase(&names_to_symbols(action), false).replace(" ", "")
}
pub fn linked_name_to_actiontag_kind(actiontag_kind : &str) -> String {
    if (actiontag_kind == "ItemsToRemove") { return String::from("Items to remove"); } // FIXME: Report this because bad capitalisation
    propercase(actiontag_kind, true)
}
pub fn linked_name_to_actiontag_value(actiontag_kind : &str, actiontag_value : &str) -> String {
    let lowercase = propercase(actiontag_value, true).split(" ").map(|word| {
        if (word.to_uppercase() == word) { Cow::Borrowed(word) }
        else { Cow::Owned(if (actiontag_kind == "Items to remove") { word.to_title_case() } else { word.to_lowercase() }) }
    }).intersperse(Cow::Borrowed(" ")).collect::<String>();
    let mut chars = lowercase.chars();
    match (chars.next()) {
        None => String::new(),
        Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str()
    }
}
pub fn collect_actiontag_parts<'l>(actiontag_parts : impl Iterator<Item = &'l str>) -> Vec<ActionFunctionTag> {
    actiontag_parts.array_chunks::<2>()
        .map(|[kind, value]| {
            let kind  = linked_name_to_actiontag_kind(kind);
            let value = linked_name_to_actiontag_value(&kind, value);
            if (value.starts_with("Dynamic")) { ActionFunctionTag::Dynamic {
                kind,
                default_value : value[8..].to_sentence_case()
            } }
            else if (value.starts_with("Specialcase")) { ActionFunctionTag::Value(CodeValue::Actiontag {
                kind,
                value : match (&value[12..]) {
                    "uppercase"  => "UPPERCASE".to_string(),
                    "lowercase"  => "lowercase".to_string(),
                    "propercase" => "Proper Case".to_string(),
                    "invertcase" => "iNVERT cASE".to_string(),
                    "randomcase" => "RAnDoM cASe".to_string(),
                    casing => casing.to_string()
                },
                variable        : None,
                block_override : None
            } ) }
            else { ActionFunctionTag::Value(CodeValue::Actiontag {
                kind,
                value,
                variable       : None,
                block_override : None
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
    propercase(gamevalue_kind, true)
}
pub fn linked_name_to_sound_id(sound_id : &str) -> String {
    propercase(sound_id, true)
}
pub fn linked_name_to_particle_id(sound_id : &str) -> String {
    propercase(sound_id, true)
}
pub fn linked_name_to_potion_id(potion_id : &str) -> String {
    propercase(&names_to_symbols(&potion_id), true)
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
