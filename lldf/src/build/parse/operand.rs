use crate::build::{BCDeclaration, FunctionBuildContext};
use crate::build::codegen::{ Codeblock, Value };
use super::{name_to_global, name_to_local};

use std::error::Error;

use llvm_ir::{ Constant, Operand };
use llvm_ir::constant::{ Add, Float, IntToPtr, Mul, PtrToInt, Sub };


pub fn handle_operand(fctx : &mut FunctionBuildContext, op : &Operand) -> Result<Value, Box<dyn Error>> { match (op) {

    Operand::LocalOperand { name, .. } => {
        let name = name_to_local(name);
        Ok(fctx.locals.get(&name).map_or_else(|| Value::line_variable(name), |l| l.clone()))
    },

    Operand::ConstantOperand(co) => { handle_constant(fctx, co) }

    Operand::MetadataOperand => { Err("Metadata operands are not supported".into()) }

} }


pub fn handle_constant(fctx : &mut FunctionBuildContext, co : &Constant) -> Result<Value, Box<dyn Error>> { match (co) {
    Constant::Int    { value, .. } => Ok(Value::Int(*value as i64)), // TODO: check signedness
    Constant::Float  ( float     ) => match (float) {
        Float::Half          => Err("Half type floats are unsupported"   .into()),
        Float::BFloat        => Err("BFloat type floats are unsupported" .into()),
        Float::Single(value) => Ok(Value::Float(*value as f64)),
        Float::Double(value) => Ok(Value::Float(*value)),
        Float::Quadruple     => Err("Quadruple type floats are unsupported" .into()),
        Float::X86_FP80      => Err("X86_FP80 type floats are unsupported"  .into()),
        Float::PPC_FP128     => Err("PPC_FP128 type floats are unsupported" .into()),
    }
    Constant::Null          ( _          ) => Ok(Value::Int(0)),
    Constant::AggregateZero ( _          ) => todo!(),
    Constant::Struct        { values, .. } => {
        let     temp_var = fctx.create_temp_var();
        let mut params   = Vec::with_capacity(values.len());
        for value in values {
            params.push(handle_constant(fctx, value)?);
        }
        for (i, chunk) in params.chunks(26).enumerate() {
            let mut final_params = Vec::with_capacity(chunk.len() + 1);
            final_params.push(temp_var.clone());
            final_params.extend_from_slice(chunk);
            fctx.current_line.blocks.push(Codeblock::action(
                "set_var",
                if (i == 0) { "CreateList" } else { "AppendValue" },
                final_params,
                Vec::new()
            ));
        }
        Ok(temp_var)
    },
    Constant::Array { elements, .. } => {
        let     temp_var = fctx.create_temp_var();
        let mut params   = Vec::with_capacity(elements.len());
        for element in elements {
            params.push(handle_constant(fctx, element)?);
        }
        for (i, chunk) in params.chunks(26).enumerate() {
            let mut final_params = Vec::with_capacity(chunk.len() + 1);
            final_params.push(temp_var.clone());
            final_params.extend_from_slice(chunk);
            fctx.current_line.blocks.push(Codeblock::action(
                "set_var",
                if (i == 0) { "CreateList" } else { "AppendValue" },
                final_params,
                Vec::new()
            ));
        }
        Ok(temp_var)
    },
    Constant::Undef           ( _        ) => Ok(Value::Int(0)),
    Constant::Poison          ( _        ) => Ok(Value::Int(0)),
    Constant::GlobalReference { name, .. } => {
        if let Some(decl) = fctx.ctx.declarations.get(name) {
            match (decl) {
                BCDeclaration::NoOp => Err("Operator found a no-op".into()),

                BCDeclaration::Action { .. } => Err("Operator found an action call".into()),

                BCDeclaration::Gamevalue { kind, target } => Ok(Value::Gamevalue { kind : kind.clone(), target : target.clone() }),

                BCDeclaration::Function => todo!(),

                BCDeclaration::Constant { value } => Ok(value.clone()),

                BCDeclaration::Static => Ok(Value::unsaved_variable(name_to_global(name)))

            }
        } else { Err(format!("Unknown declaration {:?}", name).into()) }
    },
    Constant::Add(Add { operand0, operand1 }) => {
        let temp_var = fctx.create_temp_var();
        let params   = vec![ temp_var.clone(), handle_constant(fctx, operand0)?, handle_constant(fctx, operand1)? ];
        fctx.current_line.blocks.push(Codeblock::action("set_var", "+", params, Vec::new()));
        Ok(temp_var)
    },
    Constant::Sub(Sub { operand0, operand1 }) => {
        let temp_var = fctx.create_temp_var();
        let params   = vec![ temp_var.clone(), handle_constant(fctx, operand0)?, handle_constant(fctx, operand1)? ];
        fctx.current_line.blocks.push(Codeblock::action("set_var", "-", params, Vec::new()));
        Ok(temp_var)
    },
    Constant::Mul(Mul { operand0, operand1 }) => {
        let temp_var = fctx.create_temp_var();
        let params   = vec![ temp_var.clone(), handle_constant(fctx, operand0)?, handle_constant(fctx, operand1)? ];
        fctx.current_line.blocks.push(Codeblock::action("set_var", "x", params, Vec::new()));
        Ok(temp_var)
    },
    Constant::Trunc         (_) => todo!(),
    Constant::Xor           (_) => todo!(),
    Constant::GetElementPtr (_) => todo!(),
    Constant::PtrToInt(PtrToInt { operand, .. }) | Constant::IntToPtr(IntToPtr { operand, .. }) => {
        println!("warning: Pointer-integer conversion operations are highly unstable and may result in unexpected behaviour");
        handle_constant(fctx, operand)
    },
    Constant::ICmp(_) => todo!(),
    Constant::FCmp(_) => todo!(),

    Constant::Vector(_) | Constant::ExtractElement(_) | Constant::InsertElement(_) | Constant::ShuffleVector(_) => Err("SIMD vector operators are unsupported"               .into()),
    Constant::BlockAddress                                                                                      => Err("Block address operators are unsupported"             .into()),
    Constant::TokenNone                                                                                         => Err("Token operators are unsupported"                     .into()),
    Constant::Shl(_)                                                                                            => Err("Bit shifting operators are unsupported"              .into()),
    Constant::BitCast(_)                                                                                        => Err("Bit cast operators are unsupported"                  .into()),
    Constant::AddrSpaceCast(_)                                                                                  => Err("Address space cast operators are unsupported"        .into()),
} }