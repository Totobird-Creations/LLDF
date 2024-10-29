use crate::build::codegen::{ self, CodeLine };
use super::*;

use std::collections::HashMap;

use decomp::{ cfa::CFAPrim, cfg::ControlFlowGraph, cfr::{ CFRGroup, CFRGroups } };
use inflector::Inflector;
use llvm_ir::{ Function, Module };



pub struct ParsedModule<'l> {
    pub module    : &'l Module,
    pub globals   : HashMap<Name, Global>,
    pub functions : HashMap<String, ParsedFunction<'l>>
}
#[derive(Debug)]
pub enum Global {
    NoopFunction,
    UserFunction {
        name : String
    },
    ActionFunction {
        codeblock : String,
        action    : String,
        tags      : Vec<(String, String)>
    },
    ActionPtrFunction {
        getter_codeblock : String,
        getter_action    : String,
        getter_tags      : Vec<(String, String)>,
        setter_codeblock : String,
        setter_action    : String,
        setter_tags      : Vec<(String, String)>
    },
    GamevalueFunction {
        kind : String,
        target : String
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
        functions : HashMap::new()
    };

    // Collect user defined functions.
    for function in &module.functions {
        parsed.globals.insert(Name::Name(Box::new(function.name.clone())), Global::UserFunction { name : function.name.clone() });
    }

    // Collect externally linked functions.
    for function in &module.func_declarations {

        if (function.name == "llvm.lifetime.start.p0" || function.name == "llvm.lifetime.end.p0") {
            parsed.globals.insert(Name::Name(Box::new(function.name.clone())), Global::NoopFunction);
            continue;
        }

        let mut parts = function.name.split("__");
        match (parts.next()) {

            Some("DF_ACTION") => {
                if let (Some(executor), None) = (parts.next(), parts.next()) {
                    let mut executor_parts = executor.split("_");
                    if let (Some(codeblock), Some(action)) = (executor_parts.next(), executor_parts.next()) {
                        let codeblock = linked_name_to_codeblock (codeblock );
                        let action    = linked_name_to_action    (action    );
                        parsed.globals.insert(Name::Name(Box::new(function.name.clone())), Global::ActionFunction {
                            codeblock, action,
                            tags : collect_actiontag_parts(executor_parts)
                        });
                        continue;
                    }
                }
            },

            Some("DF_ACTIONPTR") => {
                if let (Some(getter), Some(setter), None) = (parts.next(), parts.next(), parts.next()) {
                    let mut getter_parts = getter.split("_");
                    let mut setter_parts = setter.split("_");
                    if let (Some(getter_codeblock), Some(getter_action), Some(setter_codeblock), Some(setter_action))
                        = (getter_parts.next(), getter_parts.next(), setter_parts.next(), setter_parts.next())
                    {
                        let getter_codeblock = linked_name_to_codeblock (getter_codeblock );
                        let getter_action    = linked_name_to_action    (getter_action    );
                        let setter_codeblock = linked_name_to_codeblock (setter_codeblock );
                        let setter_action    = linked_name_to_action    (setter_action    );
                        parsed.globals.insert(Name::Name(Box::new(function.name.clone())), Global::ActionPtrFunction {
                            getter_codeblock, getter_action,
                            getter_tags : collect_actiontag_parts(getter_parts),
                            setter_codeblock, setter_action,
                            setter_tags : collect_actiontag_parts(setter_parts)
                        });
                        continue;
                    }
                }
            },

            Some("DF_GAMEVALUE") => {
                if let (Some(getter), None) = (parts.next(), parts.next()) {
                    let mut getter_parts = getter.split("_");
                    if let (Some(kind), Some(target)) = (getter_parts.next(), getter_parts.next())
                    {
                        let kind   = linked_name_to_gamevalue_kind   (kind   );
                        let target = linked_name_to_gamevalue_target (target );
                        parsed.globals.insert(Name::Name(Box::new(function.name.clone())), Global::GamevalueFunction { kind, target });
                        continue;
                    }
                }
            }

            _ => { }
        }

        //else if (function.name.starts_with("DF_GAMEVALUE_")) {
        //    let mut parts = function.name.split("_").skip(2);
        //    let Some(kind   ) = parts.next() else { return Err(format!("Externally linked function {} missing required data for game value", function.name).into()) };
        //    let Some(target ) = parts.next() else { return Err(format!("Externally linked function {} missing required data for game value", function.name).into()) };
        //    let kind   = fix_title_case(kind.to_title_case());
        //    let target = target.to_class_case();
        //    parsed.globals.insert(Name::Name(Box::new(function.name.clone())), Global::GamevalueFunction { kind, target });
        //}

        return Err(format!("Unrecognised externally linked function {}", function.name).into());
    }

    // Collect global variables.
    //for GlobalVariable { name, is_constant, initializer, .. } in &module.global_vars {
    //    println!("{name} {is_constant}");
    //}

    println!();
    for function in &module.functions {
        let mut parsed_function = parse_function(&parsed, function)?;
        println!("{}", function.name);
        codegen::opt::dead_selections(&mut parsed_function.line);
        for block in &parsed_function.line.blocks {
            println!("  {:?}", block);
        }
        println!();
        parsed.functions.insert(function.name.clone(), parsed_function);
    }

    Ok(parsed)
}

fn linked_name_to_codeblock(codeblock : &str) -> String {
    let codeblock = codeblock.to_snake_case();
    match (codeblock.as_str()) {
        "select_object" => String::from("select_obj" ),
        "set_variable"  => String::from("set_var"    ),
        _               => codeblock
    }
}
fn linked_name_to_action(action : &str) -> String {
    action.to_title_case().replace(" ", "")
}
fn linked_name_to_actiontag_kind(actiontag_kind : &str) -> String {
    actiontag_kind.to_title_case()
}
fn linked_name_to_actiontag_value(actiontag_value : &str) -> String {
    actiontag_value.to_sentence_case()
}
fn collect_actiontag_parts<'l>(actiontag_parts : impl Iterator<Item = &'l str>) -> Vec<(String, String)> {
    actiontag_parts.array_chunks::<2>()
        .map(|[kind, value]| (linked_name_to_actiontag_kind(kind), linked_name_to_actiontag_value(value)))
        .collect::<Vec<_>>()
}
fn linked_name_to_gamevalue_kind(gamevalue_kind : &str) -> String {
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
fn linked_name_to_gamevalue_target(gamevalue_kind : &str) -> String {
    gamevalue_kind.to_title_case()
}




pub struct ParsedFunction<'l> {
    pub function  : &'l Function,
    pub locals    : HashMap<Name, Value>,
    pub line      : CodeLine,
    pub next_temp : usize
}
impl ParsedFunction<'_> {
    pub fn create_temp_var(&mut self) -> Name {
        let temp_var = self.next_temp;
        self.next_temp += 1;
        Name::Name(Box::new(format!("local.temp.{}", temp_var)))
    }
}

pub fn parse_function<'l>(module : &ParsedModule, function : &'l Function) -> Result<ParsedFunction<'l>, Box<dyn Error>> {
    let mut parsed = ParsedFunction {
        function,
        locals    : HashMap::new(),
        line      : CodeLine::new(),
        next_temp : 0
    };

    let Some(cfr) = CFAPrim::find_all(ControlFlowGraph::new(function)).map(|cfa| CFRGroups::new(&cfa)).flatten() else {
        return Err(format!("Failed to recover control flow primitives of function `{}`.", function.name).into())
    };

    parse_cfr_groups(module, &mut parsed, cfr)?;

    Ok(parsed)
}




pub fn parse_cfr_groups(module : &ParsedModule, parsed : &mut ParsedFunction, groups : CFRGroups) -> Result<(), Box<dyn Error>> {
    for group in groups.groups {
        parse_cfr_group(module, parsed, group)?;
    }
    Ok(())
}

pub fn parse_cfr_group(module : &ParsedModule, parsed : &mut ParsedFunction, group : CFRGroup) -> Result<(), Box<dyn Error>> { match (group) {
    CFRGroup::Block(name) => parse_block(module, parsed, name),
    CFRGroup::PreconditionLoop { cond, body } => todo!("precondition-loop {} {}", cond, body),
    CFRGroup::PostconditionLoop { cond } => todo!("postcondition-loop {}", cond),
    CFRGroup::OnewayConditional { cond, body } => todo!("oneway-conditional {} {}", cond, body),
    CFRGroup::OnewayReturnConditional { cond, body } => todo!("oneway-return-conditional {} {}", cond, body),
    CFRGroup::TwowayConditional { cond, body_true, body_false } => todo!("twoway-conditional {} {} {}", cond, body_true, body_false),
} }




pub fn parse_block(module : &ParsedModule, parsed : &mut ParsedFunction, block : Name) -> Result<(), Box<dyn Error>> {
    let block = parsed.function.basic_blocks.iter().find(|bb| bb.name == block).unwrap();
    for instr in &block.instrs {
        parse_instr(module, parsed, instr)?;
    }
    // TODO: Terminator? This might not be needed if it's part of CFR groups.
    Ok(())
}
