use crate::build::codegen::{ Codeblock, Value };
use crate::build::{ FunctionBuildContext, BCDeclaration };
use super::{ handle_operand, name_to_global, name_to_local };

use std::error::Error;

use llvm_ir::{ Constant, Operand };
use llvm_ir::instruction::Call;


pub fn handle_call(fctx : &mut FunctionBuildContext, op : &Operand, call : &Call) -> Result<(), Box<dyn Error>> { match (op) {
    Operand::LocalOperand { name, ty : _ } => {
        fctx.current_line.blocks.push(Codeblock::call_func(format!("%var({})", name_to_local(name)), vec![ /* TODO */ ]));
        Ok(())
    },
    Operand::ConstantOperand(co) => { handle_call_constant(fctx, co, call) },
    Operand::MetadataOperand => { Err("Metadata operands are not supported".into()) }
} }


fn handle_call_constant(fctx : &mut FunctionBuildContext, co : &Constant, call : &Call) -> Result<(), Box<dyn Error>> { match (co) {

    Constant::GlobalReference { name, .. } => {
        if let Some(decl) = fctx.ctx.declarations.get(name) {
            match (decl) {
                BCDeclaration::NoOp => { }

                BCDeclaration::Action { codeblock, kind, tags } => {
                    let mut args = call.arguments.iter();
                    let mut final_tags = Vec::with_capacity(tags.len());
                    for tag in tags {
                        let Some((op, _)) = args.next() else { return Err("Action call didn't receive enough parameters for tags".into()) };
                        let out = handle_operand(fctx, op)?.as_actiontag()?;
                        final_tags.push(Value::Actiontag {
                            kind     : tag.clone(),
                            value    : out.0, // TODO
                            variable : out.1.map(|v| Box::new(v))
                        })
                    }
                    let mut params = Vec::with_capacity(call.arguments.len() - final_tags.len());
                    for (op, _) in args {
                        params.push(handle_operand(fctx, op)?);
                    }
                    fctx.current_line.blocks.push(Codeblock::action(codeblock, kind, params, final_tags));
                },

                BCDeclaration::Gamevalue { kind, target } => { if let Some(dest) = &call.dest {
                    let var    = Value::line_variable(name_to_local(dest));
                    let gv     = Value::Gamevalue { kind : kind.clone(), target : target.clone() };
                    let params = vec![ var.clone(), gv ];
                    fctx.current_line.blocks.push(Codeblock::action("set_var", "=", params, Vec::new()));
                } },

                BCDeclaration::Function => {
                    //fctx.current_line.blocks.push(Codeblock::call_func(name, vec![ /* TODO */ ]))
                    // TODO
                },

                BCDeclaration::Constant { .. } => {
                    // TODO
                },

                BCDeclaration::Static => {
                    fctx.current_line.blocks.push(Codeblock::call_func(format!("%var({})", name_to_global(name)), vec![ /* TODO */ ]))
                }

            }
            Ok(())
        } else { Err(format!("Unknown declaration {:?}", name).into()) }
    },

    Constant::Int { .. }                                                        => Err("Integer can not be used as a function"                                       .into()),
    Constant::Float(_)                                                          => Err("Float can not be used as a function"                                         .into()),
    Constant::Null(_)                                                           => Err("Null can not be used as a function"                                          .into()),
    Constant::AggregateZero(_)                                                  => Err("Aggregate can not be used as a function"                                     .into()),
    Constant::Struct { .. }                                                     => Err("Struct can not be used as a function"                                        .into()),
    Constant::Array { .. }                                                      => Err("Array can not be used as a function"                                         .into()),
    Constant::Vector(_) | Constant::ShuffleVector(_)                            => Err("SIMD vector can not be used as a function"                                   .into()),
    Constant::Undef(_)                                                          => Err("Undefined can not be used as a function"                                     .into()),
    Constant::Poison(_)                                                         => Err("Poison can not be used as a function"                                        .into()),
    Constant::BlockAddress                                                      => Err("Block address can not be used as a function"                                 .into()),
    Constant::Add(_) | Constant::Sub(_) | Constant::Mul(_) | Constant::Trunc(_) => Err("Numeric operations will never return a value that can be used as a function" .into()),
    Constant::Xor(_) | Constant::Shl(_)                                         => Err("Bit operations will never return a value that can be used as a function"     .into()),
    Constant::TokenNone                                                         => Err("Token can not be used as a function"                                         .into()),
    Constant::ExtractElement(_) | Constant::InsertElement(_)                    => Err("SIMD vector element will never be a value that can be used as a function"    .into()),
    Constant::GetElementPtr(_) => todo!(),
    Constant::PtrToInt(_) => todo!(),
    Constant::IntToPtr(_) => todo!(),
    Constant::BitCast(_) => todo!(),
    Constant::AddrSpaceCast(_) => todo!(),
    Constant::ICmp(_) => todo!(),
    Constant::FCmp(_) => todo!(),
} }
