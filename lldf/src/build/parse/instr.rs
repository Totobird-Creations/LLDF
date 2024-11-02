use crate::build::codegen::{CodeValue, Codeblock};

use super::*;

use llvm_ir::instruction::*;
use llvm_ir::ConstantRef;
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

    Instruction::FAdd(FAdd { operand0, operand1, dest, .. }) => handle_binop(module, function, dest, operand0, operand1, "+"),

    Instruction::FSub(FSub { operand0, operand1, dest, .. }) => handle_binop(module, function, dest, operand0, operand1, "-"),

    Instruction::FMul(FMul { operand0, operand1, dest, .. }) => handle_binop(module, function, dest, operand0, operand1, "x"),

    Instruction::FDiv(FDiv { operand0, operand1, dest, .. }) => handle_binop(module, function, dest, operand0, operand1, "/"), // TODO: error on div by zero

    Instruction::FRem(FRem { operand0, operand1, dest, .. }) => { // TODO: error on div by zero.
        let dest_var = CodeValue::line_variable_name(dest);
        let params = vec![
            dest_var.clone(),
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        let tags   = vec![ CodeValue::Actiontag { kind : "Remainder Mode".to_string(), value : "Remainder".to_string(), variable : None } ];
        function.line.blocks.push(Codeblock::action("set_var", "%", params, tags));
        function.locals.insert(dest.clone(), Value::CodeValue(dest_var));
        Ok(())
    },

    Instruction::FNeg(FNeg { operand, dest, .. }) => handle_binop(module, function, dest, &Operand::ConstantOperand(ConstantRef::new(llvm_ir::Constant::Int { bits : 0, value : 0 })), operand, "+"),

    Instruction::ExtractValue(_) => todo!(),

    Instruction::InsertValue(_) => todo!(),

    Instruction::Alloca(Alloca { dest, .. }) => {
        function.locals.insert(dest.clone(), Value::GetSetPtr {
            getter           : GSPGetter::Local(dest.clone()),
            setter_codeblock : String::from("set_var"),
            setter_action    : String::from("="),
            setter_tags      : Vec::new(),
            params           : vec![ Value::CodeValue(CodeValue::line_variable_name(dest)) ],
        });
        Ok(())
    },

    Instruction::Load(Load { address, dest, .. }) => {
        match (parse_oper(module, function, address)?) {

            Value::GlobalRef(name) => {
                let Some(global) = module.globals.get(&name) else { return Err(format!("Unknown global {}", name).into()) };
                match (global) {

                    Global::NoopFunction => Err("Can not load from a no-op function".into()),

                    Global::UserFunction { .. } => Err("Can not load from a function".into()),

                    Global::ActionFunction { .. } => Err("Can not load from an action function".into()),

                    Global::ActionPtrFunction { .. } => Err("Can not load from an action-pointer function".into()),

                    Global::GamevalueFunction { .. } => Err("Can not load from a gamevalue function".into()),

                    Global::Constant(value) => {
                        let params = vec![ CodeValue::line_variable_name(dest), value.to_codevalue(module, function)? ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                        function.locals.insert(dest.clone(), value.clone());
                        Ok(())
                    },

                }
            },

            Value::GetSetPtr { getter, params, .. } => {
                let dest_value = getter.to_codevalue(module, function, &params)?;
                function.locals.insert(dest.clone(), Value::CodeValue(dest_value));
                Ok(())
            },

            Value::IntPtr(_) => todo!(),

            Value::Local(name) => {
                let Some(value) = function.locals.get(&name) else { return Err("".into()) };
                let value = value.clone();
                let params = vec![ CodeValue::line_variable_name(dest), value.to_codevalue(module, function)? ];
                function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                function.locals.insert(dest.clone(), value);
                Ok(())
            },

            Value::CodeValue(_) => Err("Can not load from a code value".into())

        }
    },

    Instruction::Store(Store { address, value, .. }) => {
        let     value   = parse_oper(module, function, value   )?;
        let mut address = parse_oper(module, function, address )?;
        handle_store(module, function, &mut address, &value)
    },

    Instruction::GetElementPtr(GetElementPtr { address, indices, dest, .. }) => {
        if (indices.len() != 1) { return Err("Multi-index GEP instructions are unsupported".into()); }
        let params = vec![
            parse_oper(module, function, address)?,
            parse_oper(module, function, &indices[0])?
        ];
        let value = Value::GetSetPtr {
            getter : GSPGetter::Codeblock {
                codeblock : "set_var".to_string(),
                action    : "GetListValue".to_string(),
                tags      : vec![ ]
            },
            setter_codeblock : "set_var".to_string(),
            setter_action    : "SetListValue".to_string(),
            setter_tags      : vec![ ],
            params
        };
        function.locals.insert(dest.clone(), value);
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
        match (parse_oper(module, function, calling)?) {

            Value::GlobalRef(global) => {
                let Some(global) = module.globals.get(&global) else { return Err(format!("Unknown global {}", global).into()) };
                match (global) {

                    Global::NoopFunction => Ok(()),

                    Global::UserFunction { name } => {
                        let block = Codeblock::call_func(name, vec![]); // TODO: params & return value
                        function.line.blocks.push(block);
                        Ok(())
                    },

                    Global::ActionFunction { codeblock, action, tags } => {
                        let mut params = Vec::with_capacity(arguments.len());
                        for (arg, _) in arguments {
                            params.push(parse_oper(module, function, arg)?.to_codevalue(module, function)?);
                        }
                        let block  = Codeblock::action(codeblock, action, params, tags.clone());
                        function.line.blocks.push(block);
                        Ok(())
                    },

                    Global::ActionPtrFunction { getter_codeblock, getter_action, getter_tags, setter_codeblock, setter_action, setter_tags } => {
                        let Some(dest) = dest else { return Err("Action pointer function return value must be assigned to a local".into()) };
                        let mut params = Vec::with_capacity(arguments.len());
                        for (arg, _) in arguments {
                            params.push(parse_oper(module, function, arg)?);
                        }
                        let value = Value::GetSetPtr {
                            getter : GSPGetter::Codeblock {
                                codeblock : getter_codeblock .clone(),
                                action    : getter_action    .clone(),
                                tags      : getter_tags      .clone()
                            },
                            setter_codeblock : setter_codeblock .clone(),
                            setter_action    : setter_action    .clone(),
                            setter_tags      : setter_tags      .clone(),
                            params
                        };
                        function.locals.insert(dest.clone(), value);
                        Ok(())
                    },

                    Global::GamevalueFunction { kind, target } => {
                        let Some(dest) = dest else { return Err("Game value function return value must be assigned to a local".into()) };
                        let dest_var = CodeValue::line_variable_name(dest);
                        let params = vec![ dest_var.clone(), CodeValue::Gamevalue { kind : kind.clone(), target : target.clone() } ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                        function.locals.insert(dest.clone(), Value::CodeValue(dest_var));
                        Ok(())
                    },

                    Global::Constant(_) => todo!()

                }
            },

            Value::GetSetPtr { .. } => { Err("Can not call a pointer".into()) }, // TODO: function-pointer, maybe?

            Value::IntPtr(_) => { Err("Can not call an integer".into()) },

            Value::Local(_) => { Err("Can not call a local".into()) },

            Value::CodeValue(_) => { Err("Can not call a code value".into()) }

        }
    },

    Instruction::Shl(_) | Instruction::LShr(_) | Instruction::AShr(_)                              => Err("Bit shift instructions are unsupported"          .into()),
    Instruction::ExtractElement(_) | Instruction::InsertElement(_) | Instruction::ShuffleVector(_) => Err("Vector instructions are unsupported"             .into()),
    Instruction::Fence(_) | Instruction::CmpXchg(_) | Instruction::AtomicRMW(_)                    => Err("Atomic instructions are unsupported"             .into()),
    Instruction::BitCast(_)                                                                        => Err("Bit cast instructions are unsupported"           .into()),
    Instruction::AddrSpaceCast(_)                                                                  => Err("Address space cast instructions are unsupported" .into()),
    Instruction::Freeze(_)                                                                         => Err("Propagation freeze instructions are unsupported" .into()),
    Instruction::VAArg(_)                                                                          => Err("Variadic argument instructions are unsupported"  .into()),
    Instruction::LandingPad(_) | Instruction::CatchPad(_) | Instruction::CleanupPad(_)             => Err("Exception handling instructions are unsupported" .into()),
} }





fn handle_binop(module : &ParsedModule, function : &mut ParsedFunction, dest : &Name, operand0 : &Operand, operand1 : &Operand, op : &str) -> Result<(), Box<dyn Error>> {
    let dest_var = CodeValue::line_variable_name(dest);
    let params = vec![
        dest_var.clone(),
        parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
        parse_oper(module, function, operand1)?.to_codevalue(module, function)?
    ];
    function.line.blocks.push(Codeblock::action("set_var", op, params, vec![]));
    function.locals.insert(dest.clone(), Value::CodeValue(dest_var));
    Ok(())
}



fn handle_store(module : &ParsedModule, function : &mut ParsedFunction, address : &mut Value, value : &Value) -> Result<(), Box<dyn Error>> {
    match (address) {

        Value::GlobalRef(name) => {
            let params = vec![ CodeValue::unsaved_variable_name(name), value.to_codevalue(module, function)? ];
            function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
            function.locals.insert(name.clone(), value.clone());
            Ok(())
        },

        Value::GetSetPtr { getter, setter_codeblock, setter_action, setter_tags, params, .. } => {
            let mut final_params = Vec::with_capacity(params.len() + 1);
            for param in params {
                final_params.push(param.to_codevalue(module, function)?);
            }
            final_params.push(value.to_codevalue(module, function)?);
            function.line.blocks.push(Codeblock::action(setter_codeblock.clone(), setter_action.clone(), final_params, setter_tags.clone()));
            *getter = GSPGetter::Codeblock {
                codeblock : String::from("set_var"),
                action    : String::from("="),
                tags      : vec![ ]
            };
            Ok(())
        },

        Value::IntPtr(_) => Err("Can not store to an integer".into()),

        Value::Local(_) => Err("Can not store to a local".into()), // TODO: Pointer?

        Value::CodeValue(_) => Err("Can not store to a code value".into())

    }
}
