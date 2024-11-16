use crate::build::codegen::opt::optimise;

use super::*;
use super::super::codegen::{ CodeLine, Codeblock, ParameterType };

use llvm_ir::{ BasicBlock, Function, Type };
use llvm_ir::module::{Linkage, Visibility};
use llvm_ir::terminator::*;



pub struct ParsedFunction {
    pub line      : CodeLine,
    pub next_temp : usize
}
impl ParsedFunction {
    pub fn new() -> Self { Self {
        line      : CodeLine::new(),
        next_temp : 0
    } }
    pub fn create_temp_var_name(&mut self) -> String {
        let temp_var = self.next_temp;
        self.next_temp += 1;
        format!("local.#%var({}).temp.{}", CALL, temp_var)
    }
}


pub const NEXT_CALL      : &'static str = "lldf.call.next";
pub const CALL           : &'static str = "lldf.call";
pub const BLOCK_ACTIVE   : &'static str = "lldf.block.active";
pub const BLOCK_CURRENT  : &'static str = "lldf.block.current";
pub const BLOCK_PREVIOUS : &'static str = "lldf.block.previous";
pub const RETURN         : &'static str = "lldf.return";
pub const ALLOCA         : &'static str = "lldf.alloca";


pub fn parse_function(module : &mut ParsedModule, function : &Function) -> Result<(), Box<dyn Error>> {

    let mut root_function = ParsedFunction::new();

    // Call frame
    let next_call    = CodeValue::Variable { name : NEXT_CALL.to_string(), scope : VariableScope::Unsaved };
    let current_call = CodeValue::Variable { name : CALL.to_string(), scope : VariableScope::Line };
    root_function.line.blocks.push(Codeblock::action("set_var", "+=", vec![
        next_call.clone()
    ], vec![ ]));
    root_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        current_call.clone(),
        next_call
    ], vec![ ]));

    let mut params = Vec::with_capacity(function.parameters.len());

    // Return value
    let does_return = ! matches!(*function.return_type, Type::VoidType);
    if (does_return) {
        params.push(CodeValue::Parameter {
            name : RETURN.to_string(),
            typ  : ParameterType::Variable, plural : false, optional : false,
            description : Some("RETURN".to_string()),
            note        : Some(format!("Type: {}", function.return_type))
        });
    }

    // Parameters    
    for param in &function.parameters {
        params.push(CodeValue::Parameter {
            name : name_to_string(&param.name),
            typ : handle_param_type(module, &*param.ty)?,
            plural : false, optional : false,
            description : Some(format!("{}", param.name)),
            note        : Some(format!("Type: {}", param.ty))
        });
        root_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
            CodeValue::Variable { name : name_to_local(&param.name), scope : VariableScope::Local },
            CodeValue::Variable { name : name_to_string(&param.name), scope : VariableScope::Line }
        ], vec![ ]));
    }

    // Line head
    let visible = function.visibility == Visibility::Default && matches!(function.linkage, Linkage::External | Linkage::ExternalWeak | Linkage::AvailableExternally);
    let mut head_codeblock = Codeblock::function(function.name.clone(), params, ! visible);
    let mut name_parts = function.name.split("__");
    if let (Some(df), Some(event), None) = (name_parts.next(), name_parts.next(), name_parts.next()) { if (df == "DF_EVENT") {
        let mut event_parts = event.split("_");
        if let (Some(codeblock), Some(action), None) = (event_parts.next(), event_parts.next(), event_parts.next()) {
            let codeblock = linked_name_to_codeblock (codeblock );
            let action    = linked_name_to_action    (action    );
            head_codeblock = Codeblock::event(codeblock, action)
        }
    } }
    root_function.line.blocks.insert(0, head_codeblock);

    // The main loop which handles transfers between blocks.
    let active_block   = CodeValue::Variable { name : BLOCK_ACTIVE   .to_string(), scope : VariableScope::Line };
    let current_block  = CodeValue::Variable { name : BLOCK_CURRENT  .to_string(), scope : VariableScope::Line };
    let previous_block = CodeValue::Variable { name : BLOCK_PREVIOUS .to_string(), scope : VariableScope::Line };
    root_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        active_block.clone(),
        CodeValue::String(name_to_string(&function.basic_blocks[0].name))
    ], vec![ ]));
    root_function.line.blocks.push(Codeblock::subaction("repeat", "While", "VarIsType", vec![
        active_block.clone()
    ], vec![
        CodeValue::Actiontag { kind : "Variable Type".to_string(), value : "String".to_string(), variable : None, block_override : Some("if_var".to_string()) }
    ]));
    root_function.line.blocks.push(Codeblock::OPEN_REPEAT_BRACKET);
    root_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        current_block.clone(),
        active_block.clone()
    ], vec![ ]));
    /*root_function.line.blocks.push(Codeblock::action("player_action", "SendMessage", vec![ // Intended for debugging.
        CodeValue::Text(format!("<dark_purple>{}<reset> <light_purple>%var({})", function.name, BLOCK_ACTIVE)),
    ], vec![ ]));*/
    root_function.line.blocks.push(Codeblock::call_func(format!("{}.block.%var({})", function.name, BLOCK_ACTIVE), vec![
        CodeValue::Variable { name : RETURN.to_string(), scope : VariableScope::Line },
        current_call,
        active_block,
        previous_block.clone()
    ]));
    root_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        previous_block,
        current_block
    ], vec![ ]));
    root_function.line.blocks.push(Codeblock::CLOSE_REPEAT_BRACKET);

    // Call frame cleanup
    root_function.line.blocks.push(Codeblock::action("set_var", "PurgeVars", vec![
        CodeValue::String(format!("mem.#%var({}).", CALL)),
        CodeValue::String(format!("local.#%var({}).", CALL))
    ], vec![
        CodeValue::Actiontag { kind : "Match Requirement".to_string(), value : "Any part of name".to_string(), variable : None, block_override : None },
        CodeValue::Actiontag { kind : "Ignore Case".to_string(), value : "False".to_string(), variable : None, block_override : None }
    ]));

    // Blocks
    let mut block_functions = Vec::with_capacity(function.basic_blocks.len());
    for block in &function.basic_blocks {
        block_functions.push(parse_block(module, function, block)?);
    }
    optimise(block_functions.iter_mut().collect::<Vec<_>>());

    module.functions.append(&mut block_functions);
    module.functions.push(root_function);

    Ok(())
}


pub fn parse_block(module : &mut ParsedModule, function : &Function, block : &BasicBlock) -> Result<ParsedFunction, Box<dyn Error>> {

    let mut block_function = ParsedFunction::new();
    block_function.line.blocks.insert(0, Codeblock::function(format!("{}.block.{}", function.name, name_to_string(&block.name)), vec![
        CodeValue::Parameter {
            name : RETURN.to_string(),
            typ  : ParameterType::Variable, plural : false, optional : false,
            description : Some("RETURN".to_string()),
            note        : Some(format!("Type: {}", function.return_type))
        },
        CodeValue::Parameter {
            name : CALL.to_string(), typ : ParameterType::Variable,
            plural : false, optional : false, description : None, note : None
        },
        CodeValue::Parameter {
            name : BLOCK_ACTIVE.to_string(), typ : ParameterType::Variable,
            plural : false, optional : false, description : None, note : None
        },
        CodeValue::Parameter {
            name : BLOCK_PREVIOUS.to_string(), typ : ParameterType::Variable,
            plural : false, optional : false, description : None, note : None
        }
    ], true));

    for instr in &block.instrs {
        parse_instr(module, &mut block_function, instr)?;
    }

    // TODO: Reload the selection after entering the next block.
    block_function.line.blocks.push(Codeblock::action("select_obj", "Reset", vec![ ], vec![ ]));

    match (&block.term) {

        Terminator::Ret(Ret { return_operand, .. }) => {
            if let Some(return_operand) = return_operand {
                let codeblock = Codeblock::action("set_var", "=", vec![
                    CodeValue::Variable { name : RETURN.to_string(), scope : VariableScope::Line },
                    parse_oper(module, &mut block_function, return_operand)?.to_codevalue(module, &mut block_function)?
                ], vec![ ]);
                block_function.line.blocks.push(codeblock);
            }
            block_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : BLOCK_ACTIVE.to_string(), scope : VariableScope::Line },
                CodeValue::Number("0".to_string())
            ], vec![ ]));
        },

        Terminator::Br(Br { dest, .. }) => {
            block_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : BLOCK_ACTIVE.to_string(), scope : VariableScope::Line },
                CodeValue::String(name_to_string(dest))
            ], vec![ ]));
        },

        Terminator::CondBr(CondBr { condition, true_dest, false_dest, .. }) => {
            let codeblock = Codeblock::ifs("if_var", "=", false, vec![
                parse_oper(module, &mut block_function, condition)?.to_codevalue(module, &mut block_function)?,
                CodeValue::Number("0".to_string())
            ], vec![ ]);
            block_function.line.blocks.push(codeblock);
            block_function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
            block_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : BLOCK_ACTIVE.to_string(), scope : VariableScope::Line },
                CodeValue::String(name_to_string(false_dest))
            ], vec![ ]));
            block_function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
            block_function.line.blocks.push(Codeblock::elses());
            block_function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
            block_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : BLOCK_ACTIVE.to_string(), scope : VariableScope::Line },
                CodeValue::String(name_to_string(true_dest))
            ], vec![ ]));
            block_function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        },

        Terminator::Switch(Switch { operand, dests, default_dest, .. }) => {
            let operand = parse_oper(module, &mut block_function, operand)?.to_codevalue(module, &mut block_function)?;
            for (case, dest) in dests {
                let case = parse_const(module, &mut block_function, case)?.to_codevalue(module, &mut block_function)?;
                block_function.line.blocks.push(Codeblock::ifs("if_var", "=", false, vec![
                    operand.clone(),
                    case
                ], vec![ ]));
                block_function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
                block_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                    CodeValue::Variable { name : BLOCK_ACTIVE.to_string(), scope : VariableScope::Line },
                    CodeValue::String(name_to_string(dest))
                ], vec![ ]));
                block_function.line.blocks.push(Codeblock::action("control", "Return", vec![ ], vec![ ]));
                block_function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
            }
            block_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : BLOCK_ACTIVE.to_string(), scope : VariableScope::Line },
                CodeValue::String(name_to_string(default_dest))
            ], vec![ ]));
        },

        Terminator::IndirectBr(_) => { return Err("IndirectBr terminators are unsupported (No BlockAddress)".into()) },

        Terminator::Invoke(_) => { return Err("Invoke terminators are unsupported (No Exceptions)".into()) },

        Terminator::Resume(_) => { return Err("Resume terminators are unsupported".into()) },

        Terminator::Unreachable(_) => { },

        Terminator::CleanupRet(_) => { return Err("CleanupRet terminators are unsupported".into()) },

        Terminator::CatchRet(_) => { return Err("CatchRet terminators are unsupported".into()) },

        Terminator::CatchSwitch(_) => { return Err("CatchSwitch terminators are unsupported".into()) },

        Terminator::CallBr(_) => { return Err("CallBr terminators are unsupported".into()) },

    }

    Ok(block_function)
}



fn handle_param_type(module : &ParsedModule, typ : &Type) -> Result<ParameterType, Box<dyn Error>> {
    match (typ) {
        Type::VoidType | Type::IntegerType { .. } | Type::FPType(_) => Ok(ParameterType::Number),
        Type::PointerType { .. }                                    => Ok(ParameterType::List),
        Type::FuncType { .. }                                       => Ok(ParameterType::String),
        Type::ArrayType { .. } | Type::StructType { .. }            => Ok(ParameterType::List),
        Type::NamedStructType { name }                              => handle_param_type(module, &*module.types.named_struct(name)),
        Type::VectorType { .. } => Err("Vector types are unsupported".into()),
        Type::X86_MMXType       => Err("X86_MMX types are unsupported".into()),
        Type::X86_AMXType       => Err("X86_AMX types are unsupported".into()),
        Type::MetadataType      => Err("Metadata types are unsupported".into()),
        Type::LabelType         => Err("Label types are unsupported".into()),
        Type::TokenType         => Err("Token types are unsupported".into()),
        Type::TargetExtType     => Err("TargetExt types are unsupported".into())
    }
}
