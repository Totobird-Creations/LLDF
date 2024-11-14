use super::*;
use super::super::codegen::{ CodeLine, Codeblock, ParameterType };

use std::collections::HashMap;
use std::mem;

use llvm_ir::{ BasicBlock, Function, Type };
use llvm_ir::module::Visibility;
use llvm_ir::terminator::*;



pub struct ParsedFunction {
    pub locals      : HashMap<Name, Value>,
    pub line        : CodeLine,
    pub next_temp   : usize,
    pub needs_frame : bool
}
impl ParsedFunction {
    pub fn new() -> Self { Self {
        locals      : HashMap::new(),
        line        : CodeLine::new(),
        next_temp   : 0,
        needs_frame : false
    } }
    pub fn create_temp_var_name(&mut self) -> String {
        let temp_var = self.next_temp;
        self.next_temp += 1;
        format!("local.#$var({}).temp.{}", CALL, temp_var)
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
        root_function.locals.insert(param.name.clone(), Value::Local(name_to_local(&param.name)));
        params.push(CodeValue::Parameter {
            name : name_to_string(&param.name),
            typ : ParameterType::Any, plural : false, optional : false,
            description : Some(format!("{}", param.name)),
            note        : Some(format!("Type: {}", param.ty))
        });
        root_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
            CodeValue::Variable { name : name_to_local(&param.name), scope : VariableScope::Local },
            CodeValue::Variable { name : name_to_string(&param.name), scope : VariableScope::Line }
        ], vec![ ]));
    }
    // TODO: Event header
    root_function.line.blocks.insert(0, Codeblock::function(function.name.clone(), params, function.visibility != Visibility::Default));

    // TODO: Don't use the block loop if there is only one block.
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
    root_function.line.blocks.push(Codeblock::action("player_action", "SendMessage", vec![
        CodeValue::Text(format!("<dark_purple>{}<reset> <light_purple>%var(lldf.block)", function.name)),
    ], vec![ ])); // TODO: Remove later. Intended for debugging.
    root_function.line.blocks.push(Codeblock::call_func(format!("{}.{}.%var({})", function.name, BLOCK_ACTIVE, BLOCK_ACTIVE), vec![
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
    for block in &function.basic_blocks {
        parse_block(module, &mut root_function, function, block)?;
    }

    module.functions.push(root_function);

    Ok(())
}


pub fn parse_block(module : &mut ParsedModule, root_function : &mut ParsedFunction, function : &Function, block : &BasicBlock) -> Result<(), Box<dyn Error>> {

    let mut block_function = ParsedFunction::new();
    mem::swap(&mut block_function.locals, &mut root_function.locals);
    block_function.line.blocks.insert(0, Codeblock::function(format!("{}.{}.{}", function.name, BLOCK_ACTIVE, name_to_string(&block.name)), vec![
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

        llvm_ir::Terminator::Ret(Ret { return_operand, .. }) => {
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

        llvm_ir::Terminator::Br(Br { dest, .. }) => {
            block_function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : BLOCK_ACTIVE.to_string(), scope : VariableScope::Line },
                CodeValue::String(name_to_string(dest))
            ], vec![ ]));
        },

        llvm_ir::Terminator::CondBr(CondBr { condition, true_dest, false_dest, .. }) => {
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

        llvm_ir::Terminator::Switch(_) => todo!(),

        llvm_ir::Terminator::IndirectBr(_) => todo!(),

        llvm_ir::Terminator::Invoke(_) => todo!(),

        llvm_ir::Terminator::Resume(_) => todo!(),

        llvm_ir::Terminator::Unreachable(_) => todo!(),

        llvm_ir::Terminator::CleanupRet(_) => todo!(),

        llvm_ir::Terminator::CatchRet(_) => todo!(),

        llvm_ir::Terminator::CatchSwitch(_) => todo!(),

        llvm_ir::Terminator::CallBr(_) => todo!(),

    }


    mem::swap(&mut block_function.locals, &mut root_function.locals);
    module.functions.push(block_function);

    Ok(())
}
