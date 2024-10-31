use crate::build::codegen::{CodeValue, Codeblock};

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
    Instruction::ExtractValue(_) => todo!(),
    Instruction::InsertValue(_) => todo!(),

    Instruction::Alloca(Alloca { dest, .. }) => {
        function.locals.insert(dest.clone(), Value::GetSetPtr {
            getter           : GSPGetter::Local(dest.clone()),
            setter_codeblock : String::from("set_var"),
            setter_action    : String::from("="),
            setter_tags      : Vec::new(),
            parameters       : vec![ Value::CodeValue(CodeValue::line_variable_name(dest)) ],
        });
        Ok(())
    },

    Instruction::Load(Load { address, dest, .. }) => {
        match (parse_oper(module, function, address)?) {

            Value::GlobalRef(name) => {
                let Some(global) = module.globals.get(&name) else { return Err(format!("Unknown global {}", name).into()) };
                match (global) {

                    Global::NoopFunction => Err("Can not load a no-op function".into()),

                    Global::UserFunction { .. } => Err("Can not load a function".into()),

                    Global::ActionFunction { .. } => Err("Can not load an action function".into()),

                    Global::ActionPtrFunction { .. } => Err("Can not load an action-pointer function".into()),

                    Global::GamevalueFunction { .. } => Err("Can not load a gamevalue function".into()),

                    Global::Constant(value) => {
                        let params = vec![ CodeValue::line_variable_name(dest), value.to_codevalue(module, function)? ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                        function.locals.insert(dest.clone(), value.clone());
                        Ok(())
                    },

                }
            },

            Value::GetSetPtr { getter, parameters, .. } => {
                let dest_value = getter.to_codevalue(module, function, &parameters)?;
                function.locals.insert(dest.clone(), Value::CodeValue(dest_value));
                Ok(())
            },

            Value::IntPtr(_) => todo!(),

            Value::Local(_) => todo!(),

            Value::CodeValue(_) => todo!()

        }
    },

    Instruction::Store(Store { address, value, .. }) => {
        let     value   = parse_oper(module, function, value   )?;
        let mut address = parse_oper(module, function, address )?;
        handle_store(module, function, &mut address, &value)
    },

    Instruction::GetElementPtr(_) => todo!(),
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
                        let mut parameters = Vec::with_capacity(arguments.len());
                        for (arg, _) in arguments {
                            parameters.push(parse_oper(module, function, arg)?);
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
                            parameters
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

            Value::IntPtr(_) => todo!(),

            Value::Local(_) => { Err("Can not call a local".into()) },

            Value::CodeValue(_) => { Err("Can not call a code value".into()) } // TODO: function-pointer, maybe?

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





fn handle_store(module : &ParsedModule, function : &mut ParsedFunction, address : &mut Value, value : &Value) -> Result<(), Box<dyn Error>> {
    match (address) {

        Value::GlobalRef(name) => {
            let params = vec![ CodeValue::unsaved_variable_name(name), value.to_codevalue(module, function)? ];
            function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
            function.locals.insert(name.clone(), value.clone());
            Ok(())
        },

        Value::GetSetPtr { getter, setter_codeblock, setter_action, setter_tags, parameters, .. } => {
            let mut params = Vec::with_capacity(parameters.len() + 1);
            for param in parameters {
                params.push(param.to_codevalue(module, function)?);
            }
            params.push(value.to_codevalue(module, function)?);
            function.line.blocks.push(Codeblock::action(setter_codeblock.clone(), setter_action.clone(), params, setter_tags.clone()));
            *getter = GSPGetter::Codeblock {
                codeblock : String::from("set_var"),
                action    : String::from("="),
                tags      : vec![ ]
            };
            Ok(())
        },

        Value::IntPtr(_) => todo!(),

        Value::Local(_) => todo!(),

        Value::CodeValue(_) => todo!()

    }
}
