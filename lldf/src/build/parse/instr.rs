use crate::build::codegen::{ CodeValue, Codeblock, VariableScope };

use super::*;

use llvm_ir::instruction::*;



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

    Instruction::ExtractElement(_) => todo!(),

    Instruction::InsertElement(_) => todo!(),

    Instruction::ShuffleVector(_) => todo!(),

    Instruction::ExtractValue(_) => todo!(),

    Instruction::InsertValue(_) => todo!(),

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

    Instruction::Store(Store { address, value, .. }) => {
        let address = parse_oper(module, function, address)?;
        let value   = parse_oper(module, function, value)?;
        let params  = vec![
            address.to_ptr_accessor_codevalue(module)?,
            value.to_codevalue(module)?,
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        Ok(())
    },

    Instruction::GetElementPtr(GetElementPtr { address, indices, dest, .. }) => {
        if (indices.len() != 1) { return Err("Multi-index GEP instructions are unsupported".into()); }
        let address  = parse_oper(module, function, address)?;
        let index    = parse_oper(module, function, &indices[0])?;
        let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Line };
        let accessor = address.to_ptr_accessor_part_strings(module)?;
        let params = vec![
            temp_var.clone(),
            CodeValue::Number(accessor.1),
            index.to_codevalue(module)?
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

    Instruction::Trunc(_) => todo!(),

    Instruction::ZExt(_) => todo!(),

    Instruction::SExt(_) => todo!(),

    Instruction::FPTrunc(_) => todo!(),

    Instruction::FPExt(_) => todo!(),

    Instruction::FPToUI(_) => todo!(),

    Instruction::FPToSI(_) => todo!(),

    Instruction::UIToFP(_) => todo!(),

    Instruction::SIToFP(_) => todo!(),

    Instruction::PtrToInt(_) => todo!(),

    Instruction::IntToPtr(_) => todo!(),

    Instruction::ICmp(_) => todo!(),

    Instruction::FCmp(_) => todo!(),

    Instruction::Phi(_) => todo!(),

    Instruction::Select(_) => todo!(),

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
                            parse_oper(module, function, &arguments[0].0)?.to_codevalue(module)?
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
                        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
                    }
                    Ok(())
                },

                Global::UserFunction { name } => { // TODO: return
                    let mut params = Vec::with_capacity(arguments.len());
                    for (arg, _) in arguments {
                        params.push(parse_oper(module, function, arg)?.to_codevalue(module)?);
                    }
                    function.line.blocks.push(Codeblock::call_func(name, params));
                    Ok(())
                },

                Global::ActionFunction { codeblock, action, tags } => {
                    let mut params = Vec::with_capacity(arguments.len() + 1);
                    if let Some(dest) = dest {
                        params.push(CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Line });
                        function.locals.insert(dest.clone(), Value::Local(dest.clone()));
                    }
                    for (arg, _) in arguments {
                        params.push(parse_oper(module, function, arg)?.to_codevalue(module)?);
                    }
                    function.line.blocks.push(Codeblock::action(codeblock, action, params, tags.clone()));
                    Ok(())
                },

                Global::ActionPtrFunction { getter_codeblock, getter_action, getter_tags, setter_codeblock, setter_action, setter_tags } => todo!(),

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

                Global::Constant(value) => todo!(),

            }
        } else {
            function.line.blocks.push(Codeblock::call_func(format!("%var({})", calling.to_ptr_accessor_string(module)?), vec![])); // TODO: params, return, actiontag, 
            Ok(())
        }
    },

    Instruction::VAArg(_) => todo!(),

    Instruction::Shl(_) | Instruction::LShr(_) | Instruction::AShr(_)                  => Err("Bit shift instructions are unsupported"          .into()),
    Instruction::Fence(_) | Instruction::CmpXchg(_) | Instruction::AtomicRMW(_)        => Err("Atomic instructions are unsupported"             .into()),
    Instruction::BitCast(_)                                                            => Err("Bit cast instructions are unsupported"           .into()),
    Instruction::AddrSpaceCast(_)                                                      => Err("Address space cast instructions are unsupported" .into()),
    Instruction::Freeze(_)                                                             => Err("Propagation freeze instructions are unsupported" .into()),
    Instruction::LandingPad(_) | Instruction::CatchPad(_) | Instruction::CleanupPad(_) => Err("Exception handling instructions are unsupported" .into()),
} }
