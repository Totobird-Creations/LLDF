use crate::build::codegen::SoundKind;
use crate::build::codegen::{ CodeValue, Codeblock, VariableScope };

use super::*;

use llvm_ir::instruction::*;
use llvm_ir::Operand;



pub fn parse_instr(module : &ParsedModule, function : &mut ParsedFunction, instr : &Instruction) -> Result<(), Box<dyn Error>> { match (instr) {

    Instruction::Add(Add { operand0, operand1, dest, .. }) | Instruction::FAdd(FAdd { operand0, operand1, dest, .. }) => {
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "+", params, vec![ ]));
        Ok(())
    },

    Instruction::Sub(Sub { operand0, operand1, dest, .. }) | Instruction::FSub(FSub { operand0, operand1, dest, .. }) => {
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "-", params, vec![ ]));
        Ok(())
    },

    Instruction::Mul(Mul { operand0, operand1, dest, .. }) | Instruction::FMul(FMul { operand0, operand1, dest, .. }) => {
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "x", params, vec![ ]));
        Ok(())
    },

    Instruction::UDiv(_) => todo!(),

    Instruction::SDiv(_) => todo!(),

    Instruction::URem(_) => todo!(),

    Instruction::SRem(_) => todo!(),

    Instruction::And(And { operand0, operand1, dest, .. }) => {
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", params, vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "&".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::Or(Or { operand0, operand1, dest, .. }) => {
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", params, vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "|".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::Xor(Xor { operand0, operand1, dest, .. }) => {
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", params, vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "^".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::FDiv(_) => todo!(),

    Instruction::FRem(_) => todo!(),

    Instruction::FNeg(FNeg { operand, dest, .. }) => {
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::Number("0".to_string()),
            parse_oper(module, function, operand)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "-", params, vec![ ]));
        Ok(())
    },

    Instruction::ExtractValue(_) => todo!(),

    Instruction::InsertValue(_) => todo!(),

    // Sets `dest` to a 2-element list with the first element being a unique `unsaved` variable name, and the second being 0.
    // The unique name is tied to the function it was called in, and can be discarded on return.
    Instruction::Alloca(Alloca { dest, .. }) => {
        let counter = CodeValue::Variable { name : ALLOCA.to_string(), scope : VariableScope::Unsaved };
        function.line.blocks.push(Codeblock::action("set_var", "+=", vec![ counter.clone() ], vec![ ]));
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::String(format!("mem.#%var(lldf.call).%var({})", ALLOCA)),
            CodeValue::Number("0".to_string())
        ];
        function.line.blocks.push(Codeblock::action("set_var", "CreateList", params, vec![ ]));
        Ok(())
    },

    // Gets the value pointed to by a pointer (An `unsaved` variable with name `%var(%index(address,1):%index(address,2))`) and sets `dest` to it.
    Instruction::Load(Load { address, dest, .. }) => {
        let dest_var = CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local };
        let address = parse_oper(module, function, address)?;
        let params = vec![
            dest_var,
            address.to_ptr_accessor_codevalue(module)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
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
        let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Local };
        let accessor = address.to_ptr_accessor_part_strings(module)?;
        let params = vec![
            temp_var.clone(),
            CodeValue::Number(accessor.1),
            index.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "+", params, vec![ ]));
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::String(accessor.0),
            temp_var
        ];
        function.line.blocks.push(Codeblock::action("set_var", "CreateList", params, vec![]));
        Ok(())
    },

    Instruction::Trunc(Trunc { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::ZExt(ZExt { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::SExt(SExt { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::FPTrunc(FPTrunc { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::FPExt(FPExt { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::FPToUI(FPToUI { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::FPToSI(FPToSI { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::UIToFP(UIToFP { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure this works with sign.

    Instruction::SIToFP(SIToFP { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure this works with sign.

    Instruction::PtrToInt(PtrToInt { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure this works

    Instruction::IntToPtr(IntToPtr { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure this works

    Instruction::ICmp(ICmp { predicate, operand0, operand1, dest, .. }) => {
        let op = match (predicate) {
            llvm_ir::IntPredicate::EQ => "=",
            llvm_ir::IntPredicate::NE => "!=",
            llvm_ir::IntPredicate::UGT | llvm_ir::IntPredicate::SGT => ">",
            llvm_ir::IntPredicate::UGE | llvm_ir::IntPredicate::SGE => ">=",
            llvm_ir::IntPredicate::ULT | llvm_ir::IntPredicate::SLT => "<",
            llvm_ir::IntPredicate::ULE | llvm_ir::IntPredicate::SLE => "<=",
        };
        let params = vec![
            parse_oper(module, function, operand0)?.to_codevalue(module, function)?,
            parse_oper(module, function, operand1)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("if_var", op, params, vec![ ]));
        function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::Number(String::from("1"))
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        function.line.blocks.push(Codeblock::elses());
        function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::Number(String::from("0"))
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        Ok(())
    },

    Instruction::FCmp(_) => todo!(),

    Instruction::Phi(Phi { incoming_values, dest, .. }) => {
        for (value, block) in incoming_values {
            function.line.blocks.push(Codeblock::action("if_var", "StringMatches", vec![
                CodeValue::Variable { name : BLOCK_PREVIOUS.to_string(), scope : VariableScope::Line },
                CodeValue::String(name_to_string(block))
            ], vec![
                CodeValue::Actiontag { kind : "Ignore Case".to_string(), value : "False".to_string(), variable : None, block_override : None },
                CodeValue::Actiontag { kind : "Regular Expressions".to_string(), value : "Disable".to_string(), variable : None, block_override : None },
            ]));
            function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
            let params = vec![
                CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
                parse_oper(module, function, value)?.to_codevalue(module, function)?
            ];
            function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
            function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        }
        Ok(())
    },

    // If `condition` is 0 (false), set `dest` to `false_value`, else `true_value`. 
    Instruction::Select(Select { condition, true_value, false_value, dest, .. }) => {
        let params = vec![
            parse_oper(module, function, condition)?.to_codevalue(module, function)?,
            CodeValue::Number("0".to_string())
        ];
        function.line.blocks.push(Codeblock::action("if_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, false_value)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        function.line.blocks.push(Codeblock::elses());
        function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
        let params = vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            parse_oper(module, function, true_value)?.to_codevalue(module, function)?
        ];
        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
        function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
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
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local },
                            parse_oper(module, function, &arguments[0].0)?.to_codevalue(module, function)?
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
                    }
                    Ok(())
                },

                Global::Assert(handler) => { if let Some(dest) = dest { match (handler) {

                    AssertHandler::ConstantStrToString => {
                        let value    = parse_oper(module, function, &arguments[0].0)?.to_codevalue(module, function)?;
                        let dest_var = CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local };
                        match (value) {
                            CodeValue::String(value)
                                => {
                                    function.line.blocks.push(Codeblock::action("set_var", "=", vec![ dest_var.clone(), CodeValue::String(value) ], vec![ ]));
                                    Ok(())
                                },
                            CodeValue::Text(_) | CodeValue::Number(_) | CodeValue::Location { .. } | CodeValue::Vector { .. } | CodeValue::Sound { .. } |
                            CodeValue::Particle { .. } | CodeValue::Potion { .. } | CodeValue::Item { .. }
                                => {
                                    function.line.blocks.push(Codeblock::action("set_var", "=", vec![ dest_var.clone(), CodeValue::String("".to_string()) ], vec![ ]));
                                    Ok(())
                                },
                            src_value @ CodeValue::Variable { .. } | src_value @ CodeValue::Gamevalue { .. }
                                => {
                                    function.line.blocks.push(Codeblock::action("set_var", "=", vec![ dest_var.clone(), src_value ], vec![ ]));
                                    function.line.blocks.push(Codeblock::ifs("if_var", "VarIsType", true, vec![ dest_var.clone() ], vec![ CodeValue::Actiontag { kind : "Variable Type".to_string(), value : "String".to_string(), variable : None, block_override : None } ]));
                                    function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
                                    function.line.blocks.push(Codeblock::action("set_var", "=", vec![ dest_var, CodeValue::String(String::new()) ], vec![ ]));
                                    function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
                                    Ok(())
                                },
                            CodeValue::Parameter { .. } | CodeValue::Actiontag { .. } => unreachable!(),
                        }
                    }

                } } else { Ok(()) } },

                Global::UserFunction { name } => {
                    let mut params = Vec::with_capacity(arguments.len() + 1);
                    // Return value
                    if let Some(dest) = dest {
                        params.push(CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local });
                    }
                    // Parameters
                    for (arg, _) in arguments {
                        let mut value = parse_oper(module, function, arg)?.to_codevalue(module, function)?;
                        if let CodeValue::Variable { .. } = value { } else {
                            let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Local };
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
                            let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Local };
                            let params = vec![
                                temp_var.clone(),
                                parse_oper(module, function, &arguments[skip_params].0)?.to_codevalue(module, function)?
                            ];
                            function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
                            final_tags.push(CodeValue::Actiontag { kind : kind.clone(), value : default_value.clone(), variable : Some(Box::new(temp_var)), block_override : None });
                            skip_params += 1;
                        },
                    } }

                    let mut final_params = Vec::with_capacity(arguments.len() + 1);
                    if let Some(dest) = dest {
                        final_params.push(CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local });
                    }
                    for (arg, _) in arguments.iter().skip(skip_params) {
                        final_params.push(parse_oper(module, function, arg)?.to_codevalue(module, function)?);
                    }

                    function.line.blocks.push(Codeblock::action(codeblock, action, final_params, final_tags));
                    Ok(())
                },

                Global::BracketFunction { kind, side } => {
                    function.line.blocks.push(Codeblock::Bracket { kind : kind.clone(), side : side.clone() });
                    Ok(())
                },

                Global::ElseFunction => {
                    function.line.blocks.push(Codeblock::elses());
                    Ok(())
                },

                Global::TempVarFunction => Ok(()), // TODO: Test this

                Global::GamevalueFunction { kind, target } => {
                    if let Some(dest) = dest {
                        let params = vec![
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local },
                            CodeValue::Gamevalue { kind : kind.clone(), target : target.clone() }
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                    }
                    Ok(())
                },

                Global::SoundFunction { id } => {
                    if let Some(dest) = dest {
                        let params = vec![
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local },
                            CodeValue::Sound { kind : SoundKind::Named(id.clone()), volume : 1.0, pitch : 1.0 }
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                    }
                    Ok(())
                },

                Global::PotionFunction { id } => {
                    if let Some(dest) = dest {
                        let params = vec![
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local },
                            CodeValue::Potion { kind : id.clone(), dur : 1000000, amp : 0 }
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                    }
                    Ok(())
                },

                Global::ItemFunction { id } => {
                    if let Some(dest) = dest {
                        let params = vec![
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local },
                            CodeValue::Item { id : id.clone(), count : 1, nbt : "{}".to_string() }
                        ];
                        function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
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
        CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
        parse_oper(module, function, operand)?.to_codevalue(module, function)?
    ];
    function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
    Ok(())
}
