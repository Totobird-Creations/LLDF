use std::mem::transmute;

use super::*;

use llvm_ir::instruction::*;
use llvm_ir::Operand;



pub fn parse_instr(module : &ParsedModule, function : &mut ParsedFunction, instr : &Instruction) -> Result<(), Box<dyn Error>> { match (instr) {

    Instruction::Add(Add { operand0, operand1, dest, .. }) | Instruction::FAdd(FAdd { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "+", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![ ]));
        Ok(())
    },

    Instruction::Sub(Sub { operand0, operand1, dest, .. }) | Instruction::FSub(FSub { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "-", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![ ]));
        Ok(())
    },

    Instruction::Mul(Mul { operand0, operand1, dest, .. }) | Instruction::FMul(FMul { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "x", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![ ]));
        Ok(())
    },

    Instruction::UDiv(_) => todo!(),

    Instruction::SDiv(_) => todo!(),

    Instruction::URem(_) => todo!(),

    Instruction::SRem(_) => todo!(),

    Instruction::And(And { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "&".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::Or(Or { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "|".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::Xor(Xor { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "^".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::FDiv(FDiv { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "/", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Division Mode".to_string(), value : "Default".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::FRem(FRem { operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_oper(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "%", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Remainder Mode".to_string(), value : "Remainder".to_string(), variable : None, block_override : None }
        ]));
        Ok(())
    },

    Instruction::FNeg(FNeg { operand, dest, .. }) => {
        let operand = parse_oper(module, function, operand)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "-", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::Number("0".to_string()), operand
        ], vec![ ]));
        Ok(())
    },

    Instruction::ExtractValue(ExtractValue { aggregate, indices, dest, .. }) => {
        if (indices.len() != 1) { return Err("Multi-index ExtractValue instructions are unsupported".into()); }
        let aggregate = parse_oper(module, function, aggregate)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "GetListValue", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            aggregate,
            CodeValue::Number((indices[0] + 1).to_string())
        ], vec![ ]));
        Ok(())
    },

    Instruction::InsertValue(InsertValue { aggregate, element, indices, dest, .. }) => {
        let dest_var  = CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local };
        let aggregate = parse_oper(module, function, aggregate)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "=", vec![
            dest_var.clone(),
            aggregate
        ], vec![ ]));
        let element = parse_oper(module, function, element)?.to_codevalue(module, function)?;
        for index in indices {
            function.line.blocks.push(Codeblock::action("set_var", "SetListValue", vec![
                dest_var.clone(),
                CodeValue::Number((index + 1).to_string()),
                element.clone()
            ], vec![ ]));
        }
        Ok(())
    },

    // Sets `dest` to a 2-element list with the first element being a unique `unsaved` variable name, and the second being 0.
    // The unique name is tied to the function it was called in, and can be discarded on return.
    Instruction::Alloca(Alloca { dest, .. }) => {
        let counter = CodeValue::Variable { name : ALLOCA.to_string(), scope : VariableScope::Unsaved };
        function.line.blocks.push(Codeblock::action("set_var", "+=", vec![ counter.clone() ], vec![ ]));
        function.line.blocks.push(Codeblock::action("set_var", "CreateList", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::String(format!("mem.#%var(lldf.call).%var({})", ALLOCA)),
            CodeValue::Number("0".to_string())
        ], vec![ ]));
        Ok(())
    },

    // Gets the value pointed to by a pointer (An `unsaved` variable with name `%var(%index(address,1):%index(address,2))`) and sets `dest` to it.
    Instruction::Load(Load { address, dest, .. }) => {
        let dest_var = CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local };
        let address = parse_oper(module, function, address)?.to_ptr_accessor_codevalue(module)?;
        function.line.blocks.push(Codeblock::action("set_var", "=", vec![
            dest_var,
            address
        ], vec![ ]));
        Ok(())
    },

    // Sets the value pointed to by a pointer (An `unsaved` variable with name `%var(%index(address,1):%index(address,2))`) to a value.
    Instruction::Store(Store { address, value, .. }) => {
        let address = parse_oper(module, function, address)?.to_ptr_accessor_codevalue(module)?;
        let value   = parse_oper(module, function, value)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "=", vec![
            address,
            value,
        ], vec![ ]));
        Ok(())
    },

    // Shift the second element of the pointer (represented as a 2-element list).
    Instruction::GetElementPtr(GetElementPtr { address, indices, dest, .. }) => {
        if (indices.len() != 1) { return Err("Multi-index GEP instructions are unsupported".into()); }
        let address  = parse_oper(module, function, address)?;
        let index    = parse_oper(module, function, &indices[0])?.to_codevalue(module, function)?;
        let temp_var = CodeValue::Variable { name : function.create_temp_var_name(), scope : VariableScope::Local };
        let accessor = address.to_ptr_accessor_part_strings(module)?;
        function.line.blocks.push(Codeblock::action("set_var", "+", vec![
            temp_var.clone(),
            CodeValue::Number(accessor.1),
            index
        ], vec![ ]));
        function.line.blocks.push(Codeblock::action("set_var", "CreateList", vec![
            CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
            CodeValue::String(accessor.0),
            temp_var
        ], vec![]));
        Ok(())
    },

    Instruction::Trunc(Trunc { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::ZExt(ZExt { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::SExt(SExt { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::FPTrunc(FPTrunc { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::FPExt(FPExt { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::FPToUI(_) => todo!(),

    Instruction::FPToSI(_) => todo!(),

    Instruction::UIToFP(UIToFP { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::SIToFP(SIToFP { operand, dest, .. }) => handle_passthru(module, function, operand, dest),

    Instruction::PtrToInt(PtrToInt { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure this works

    Instruction::IntToPtr(IntToPtr { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure this works

    Instruction::BitCast(BitCast { operand, dest, .. }) => handle_passthru(module, function, operand, dest), // TODO: Make sure this works

    Instruction::ICmp(ICmp { predicate, operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?;
        let operand1 = parse_oper(module, function, operand1)?;
        parse_icmp(module, function, &name_to_local(dest), *predicate, operand0, operand1)
    },

    Instruction::FCmp(FCmp { predicate, operand0, operand1, dest, .. }) => {
        let operand0 = parse_oper(module, function, operand0)?;
        let operand1 = parse_oper(module, function, operand1)?;
        parse_fcmp(module, function, &name_to_local(dest), *predicate, operand0, operand1)
    },

    Instruction::Phi(Phi { incoming_values, dest, .. }) => {
        for (value, block) in incoming_values {
            let value = parse_oper(module, function, value)?.to_codevalue(module, function)?;
            function.line.blocks.push(Codeblock::action("if_var", "StringMatches", vec![
                CodeValue::Variable { name : BLOCK_PREVIOUS.to_string(), scope : VariableScope::Line },
                CodeValue::String(name_to_string(block))
            ], vec![
                CodeValue::Actiontag { kind : "Ignore Case".to_string(), value : "False".to_string(), variable : None, block_override : None },
                CodeValue::Actiontag { kind : "Regular Expressions".to_string(), value : "Disable".to_string(), variable : None, block_override : None },
            ]));
            function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
            function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : name_to_local(dest), scope: VariableScope::Local },
                value
            ], vec![ ]));
            function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        }
        Ok(())
    },

    // If `condition` is 0 (false), set `dest` to `false_value`, else `true_value`. 
    Instruction::Select(Select { condition, true_value, false_value, dest, .. }) => {
        let condition   = parse_oper(module, function, condition   )?;
        let true_value  = parse_oper(module, function, true_value  )?;
        let false_value = parse_oper(module, function, false_value )?;
        parse_select(module, function, &name_to_local(dest), condition, true_value, false_value)
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
                    },

                    AssertHandler::NoOptF64 => {
                        let value    = parse_oper(module, function, &arguments[0].0)?;
                        let dest_var = CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local };
                        handle_nooptf64(module, function, &value, dest_var)
                    },

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

                Global::ParticleFunction { id,
                    motion, motion_variation,
                    colour, colour_variation,
                    opacity,
                    material,
                    size, size_variation,
                    roll,
                    fade_colour
                } => {
                    if let Some(dest) = dest {
                        let params = vec![
                            CodeValue::Variable { name : name_to_local(dest), scope : VariableScope::Local },
                            CodeValue::Particle {
                                kind             : id.clone(),
                                spread_x         : 0.0,
                                spread_y         : 0.0,
                                amount           : 1,
                                motion           : motion           .then(|| (0.0, 0.0, 0.0)),
                                motion_variation : motion_variation .then(|| 0.0),
                                colour           : colour           .then(|| 0),
                                colour_variation : colour_variation .then(|| 0.0),
                                opacity          : opacity          .then(|| 1.0),
                                material         : material         .then(|| "minecraft:stone".to_string()),
                                size             : size             .then(|| 1.0),
                                size_variation   : size_variation   .then(|| 0.0),
                                roll             : roll             .then(|| 0.0),
                                fade_colour      : fade_colour      .then(|| 0)
                            }
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


fn handle_nooptf64(module : &ParsedModule, function : &mut ParsedFunction, value : &Value, dest_var : CodeValue) -> Result<(), Box<dyn Error>> {
    match (value) {
        Value::Null => {
            function.line.blocks.push(Codeblock::action("set_var", "=", vec![ dest_var.clone(), CodeValue::Number("0".to_string()) ], vec![ ]));
            Ok(())
        },
        Value::ConstInt(value) => {
            let value = unsafe{ transmute::<_, f64>(value) };
            function.line.blocks.push(Codeblock::action("set_var", "=", vec![ dest_var.clone(), CodeValue::Number(value.to_string()) ], vec![ ]));
            Ok(())
        },
        Value::ConstFloat(value) => {
            function.line.blocks.push(Codeblock::action("set_var", "=", vec![ dest_var.clone(), CodeValue::Number(value.to_string()) ], vec![ ]));
            Ok(())
        },
        Value::Global(name) => {
            let global = module.globals.get(&name);
            if let Some(global) = global { match (global) {
                Global::Constant(value) => handle_nooptf64(module, function, value, dest_var),
                _ => Err("Non-constant values can not be handled by NoOptF64".into())
            } }
            else { Err(format!("Unknown global {}", name).into()) }
        },
        Value::Local(_) => Err("Non-constant values can not be handled by NoOptF64".into()),
        Value::ConstString(_) => Err("Strings can not be handled by NoOptF64".into()),
    }
}
