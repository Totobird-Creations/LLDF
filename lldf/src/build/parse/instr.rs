use crate::build::codegen::{ CodeValue, Codeblock, VariableScope };

use super::*;

use llvm_ir::instruction::*;
use llvm_ir::Operand;



pub fn parse_instr(module : &ParsedModule, function : &mut ParsedFunction, instr : &Instruction) -> Result<(), Box<dyn Error>> { match (instr) {

    Instruction::Add(_) => todo!(),

    Instruction::Sub(_) => todo!(),

    Instruction::Mul(_) => todo!(),

    Instruction::UDiv(_) => todo!(),

    Instruction::SDiv(_) => todo!(),

    Instruction::URem(_) => todo!(),

    Instruction::SRem(_) => todo!(),

    Instruction::And(_) => todo!(),

    Instruction::Or(_) => todo!(),

    Instruction::Xor(_) => todo!(),

    Instruction::FAdd(_) => todo!(),

    Instruction::FSub(_) => todo!(),

    Instruction::FMul(_) => todo!(),

    Instruction::FDiv(_) => todo!(),

    Instruction::FRem(_) => todo!(),

    Instruction::FNeg(_) => todo!(),

    Instruction::ExtractValue(_) => todo!(),

    Instruction::InsertValue(_) => todo!(),

    // Sets `dest` to a 2-element list with the first element being a unique `unsaved` variable name, and the second being 0.
    // The unique name is tied to the function it was called in, and can be discarded on return.
    Instruction::Alloca(Alloca { dest, .. }) => {
        function.needs_frame = true;
        let counter = CodeValue::Variable { name : "lldf.alloca.counter".to_string(), scope : VariableScope::Unsaved };
        function.line.blocks.push(Codeblock::action("set_var", "+=", vec![ counter.clone() ], vec![]));
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Line },
            CodeValue::String("mem.#%var(lldf.alloca.frame.current).%var(lldf.alloca.counter)".to_string()),
            CodeValue::Number("0".to_string())
        ];
        function.line.blocks.push(Codeblock::action("set_var", "CreateList", params, vec![]));
        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
        Ok(())
    },

    // Gets the value pointed to by a pointer (An `unsaved` variable with name `%var(%index(address,1):%index(address,2))`) and sets `dest` to it.
    Instruction::Load(Load { address, dest, .. }) => {
        let dest_var = CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Line };
        let address = parse_oper(module, function, address)?;
        let params = vec![
            dest_var,
            address.to_ptr_accessor_codevalue(module)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
        Ok(())
    },

    // Sets the value pointed to by a pointer (An `unsaved` variable with name `%var(%index(address,1):%index(address,2))`) to a value.
    Instruction::Store(Store { address, value, .. }) => {
        let address = parse_oper(module, function, address)?;
        let value   = parse_oper(module, function, value)?;
        let params  = vec![
            address.to_ptr_accessor_codevalue(module)?,
            value.to_codevalue(module, function)?,
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        Ok(())
    },

    // Shift the second element of the pointer (represented as a 2-element list).
    Instruction::GetElementPtr(GetElementPtr { address, indices, dest, .. }) => {
        if (indices.len() != 1) { return Err("Multi-index GEP instructions are unsupported".into()); }
        let address  = parse_oper(module, function, address)?;
        let index    = parse_oper(module, function, &indices[0])?;
        let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Line };
        let accessor = address.to_ptr_accessor_part_strings(module)?;
        let params = vec![
            temp_var.clone(),
            CodeValue::Number(accessor.1),
            index.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "+", params, vec![ ]));
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Line },
            CodeValue::String(accessor.0),
            temp_var
        ];
        function.line.blocks.push(Codeblock::action("set_var", "CreateList", params, vec![]));
        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
        Ok(())
    },

    Instruction::Trunc(Trunc { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::ZExt(_) => todo!(),

    Instruction::SExt(_) => todo!(),

    Instruction::FPTrunc(_) => todo!(),

    Instruction::FPExt(_) => todo!(),

    Instruction::FPToUI(_) => todo!(),

    Instruction::FPToSI(_) => todo!(),

    Instruction::UIToFP(_) => todo!(),

    Instruction::SIToFP(_) => todo!(),

    Instruction::PtrToInt(PtrToInt { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure PtrToInt works

    Instruction::IntToPtr(_) => todo!(),

    Instruction::ICmp(_) => todo!(),

    Instruction::FCmp(_) => todo!(),

    Instruction::Phi(_) => todo!(),

    // If `condition` is 0 (false), set `dest` to `false_value`, else `true_value`. 
    Instruction::Select(Select { condition, true_value, false_value, dest, .. }) => {
        let params = vec![
            parse_oper(module, function, condition)?.to_codevalue(module, function)?,
            CodeValue::Number("0".to_string())
        ];
        function.line.blocks.push(Codeblock::action("if_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Line },
            parse_oper(module, function, false_value)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        function.line.blocks.push(Codeblock::elses());
        function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Line },
            parse_oper(module, function, true_value)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
        Ok(())
    },

    // Call a function, or the function behind a pointer.
    // This is also used for manually adding codeblocks.
    Instruction::Call(Call { function : calling, arguments, dest, .. }) => {
        let Some(calling) = calling.as_ref().right() else { return Err("Inline assembly is unsupported".into()) };
        let calling = parse_oper(module, function, calling)?;
        if let Value::Global(name) = &calling {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {

                Global::NoopFunction => {
                    if let Some(dest) = dest {
                        if (arguments.len() != 1) { return Err("Noop function with destination local must have one argument".into()); }
                        let params = vec![
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Line },
                            parse_oper(module, function, &arguments[0].0)?.to_codevalue(module, function)?
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
                        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
                    }
                    Ok(())
                },

                Global::UserFunction { name } => { // TODO: return
                    let mut params = Vec::with_capacity(arguments.len());
                    for (arg, _) in arguments {
                        let mut value = parse_oper(module, function, arg)?.to_codevalue(module, function)?;
                        if let CodeValue::Variable { .. } = value { } else {
                            let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Line };
                            let params = vec![
                                temp_var.clone(),
                                value
                            ];
                            function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
                            value = temp_var;
                        }
                        params.push(value);
                    }
                    function.line.blocks.push(Codeblock::call_func(name, params));
                    Ok(())
                },

                Global::ActionFunction { codeblock, action, tags } => {

                    let mut final_tags  = Vec::with_capacity(tags.len());
                    let mut skip_params = 0;
                    for tag in tags { match (tag) {
                        ActionFunctionTag::Value(value) => { final_tags.push(value.clone()); },
                        ActionFunctionTag::Dynamic { kind, default_value } => {
                            let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Line };
                            let params = vec![
                                temp_var.clone(),
                                parse_oper(module, function, &arguments[0].0)?.to_codevalue(module, function)?
                            ];
                            function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
                            final_tags.push(CodeValue::Actiontag { kind : kind.clone(), value : default_value.clone(), variable : Some(Box::new(temp_var)) });
                            skip_params += 1;
                        },
                    } }

                    let mut final_params = Vec::with_capacity(arguments.len() + 1);
                    if let Some(dest) = dest {
                        final_params.push(CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Line });
                        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
                    }
                    for (arg, _) in arguments.iter().skip(skip_params) {
                        final_params.push(parse_oper(module, function, arg)?.to_codevalue(module, function)?);
                    }

                    function.line.blocks.push(Codeblock::action(codeblock, action, final_params, final_tags));
                    Ok(())
                },

                Global::ActionPtrFunction { .. } => todo!(),

                Global::GamevalueFunction { kind, target } => {
                    if let Some(dest) = dest {
                        let params = vec![
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Line },
                            CodeValue::Gamevalue { kind : kind.clone(), target : target.clone() }
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
                    }
                    Ok(())
                },

                Global::Constant(_) => todo!(),

            }
        } else {
            function.line.blocks.push(Codeblock::call_func(format!("%var({})", calling.to_ptr_accessor_string(module)?), vec![])); // TODO: params, return, actiontag, 
            Ok(())
        }
    },


    Instruction::ExtractElement(_) | Instruction::InsertElement(_) | Instruction::ShuffleVector(_) => Err("Vector instructions are unsupported"             .into()),
    Instruction::VAArg(_)                                                                          => Err("Variadic argument instructions are unsupported"  .into()),
    Instruction::Shl(_) | Instruction::LShr(_) | Instruction::AShr(_)                              => Err("Bit shift instructions are unsupported"          .into()),
    Instruction::Fence(_) | Instruction::CmpXchg(_) | Instruction::AtomicRMW(_)                    => Err("Atomic instructions are unsupported"             .into()),
    Instruction::BitCast(_)                                                                        => Err("Bit cast instructions are unsupported"           .into()),
    Instruction::AddrSpaceCast(_)                                                                  => Err("Address space cast instructions are unsupported" .into()),
    Instruction::Freeze(_)                                                                         => Err("Propagation freeze instructions are unsupported" .into()),
    Instruction::LandingPad(_) | Instruction::CatchPad(_) | Instruction::CleanupPad(_)             => Err("Exception handling instructions are unsupported" .into()),
} }


fn handle_passthru(module : &ParsedModule, function : &mut ParsedFunction, operand : &Operand, dest : &Name) -> Result<(), Box<dyn Error>> {
    let params = vec![
        CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Line },
        parse_oper(module, function, operand)?.to_codevalue(module, function)?
    ];
    function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
    function.locals.insert(dest.clone(), Value::Local(dest.clone()));
    Ok(())
}
