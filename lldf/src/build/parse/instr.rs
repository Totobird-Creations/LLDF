use crate::build::codegen::{ Codeblock, Value };
use crate::build::FunctionBuildContext;
use super::{ handle_call, handle_operand, name_to_local };

use std::error::Error;

use llvm_ir::Instruction;
use llvm_ir::instruction::{ Add, Alloca, Call, FAdd, FDiv, FMul, FNeg, FRem, FSub, Mul, SDiv, SExt, SIToFP, SRem, Sub, UDiv, UIToFP, URem, ZExt, IntToPtr, PtrToInt, Load };


pub fn handle_instr(fctx : &mut FunctionBuildContext, instr : &Instruction) -> Result<(), Box<dyn Error>> {
    if (instr.is_atomic()) { return Err("Atomic instructions are unsupported".into()); }
    match (instr) {
        Instruction::Add(Add { .. }) | Instruction::FAdd(FAdd { .. }) => todo!(),
        Instruction::Sub(Sub { .. }) | Instruction::FSub(FSub { .. }) => todo!(),
        Instruction::Mul(Mul { .. }) | Instruction::FMul(FMul { .. }) => todo!(),
        Instruction::UDiv(UDiv { .. }) | Instruction::SDiv(SDiv { .. }) => todo!(),
        Instruction::FDiv(FDiv { .. }) => todo!(),
        Instruction::URem(URem { .. }) | Instruction::SRem(SRem { .. }) | Instruction::FRem(FRem { .. }) => todo!(),
        Instruction::FNeg(FNeg { .. }) => todo!(),
        Instruction::And(_) => todo!(),
        Instruction::Or(_) => todo!(),
        Instruction::Xor(_) => todo!(),
        Instruction::ExtractValue(_) => todo!(),
        Instruction::InsertValue(_) => todo!(),
        Instruction::Alloca(Alloca { .. }) => { /* no-op */ Ok(()) },
        Instruction::Load(Load { dest, address, .. }) => {
            let params = vec![ Value::line_variable(name_to_local(dest)), handle_operand(fctx, address)? ];
            fctx.current_line.blocks.push(Codeblock::action("set_var", "=", params, Vec::new()));
            Ok(())
        },
        Instruction::Store(_) => todo!(),
        Instruction::GetElementPtr(_) => todo!(),
        Instruction::Trunc(_) => todo!(),
        Instruction::ZExt(ZExt { .. }) | Instruction::SExt(SExt { .. }) => { /* no-op */ Ok(()) },
        Instruction::FPTrunc(_) => todo!(),
        Instruction::FPExt(_) => todo!(),
        Instruction::FPToUI(_) => todo!(),
        Instruction::FPToSI(_) => todo!(),
        Instruction::UIToFP(UIToFP { .. }) | Instruction::SIToFP(SIToFP { .. }) => todo!(),
        Instruction::PtrToInt(PtrToInt { .. }) | Instruction::IntToPtr(IntToPtr { .. }) => todo!(),
        Instruction::ICmp(_) => todo!(),
        Instruction::FCmp(_) => todo!(),
        Instruction::Phi(_) => todo!(),
        Instruction::Select(_) => todo!(),
        Instruction::Freeze(_) => todo!(),
        Instruction::Call(call @ Call { function, .. }) => {
            let Some(function) = function.clone().right() else { return Err("Inline-assembly is unsupported".into()) };
            handle_call(fctx, &function, call)
        },

        Instruction::Shl(_) | Instruction::LShr(_) | Instruction::AShr(_)                              => Err("Bit shifting instructions are unsupported"              .into()),
        Instruction::ExtractElement(_) | Instruction::InsertElement(_) | Instruction::ShuffleVector(_) => Err("SIMD vector instructions are unsupported"               .into()),
        Instruction::Fence(_) | Instruction::CmpXchg(_) | Instruction::AtomicRMW(_)                    => Err("Atomic instructions are unsupported"                    .into()),
        Instruction::BitCast(_)                                                                        => Err("Bit cast instructions are unsupported"                  .into()),
        Instruction::AddrSpaceCast(_)                                                                  => Err("Address space cast instructions are unsupported"        .into()),
        Instruction::VAArg(_)                                                                          => Err("Variadic arguments instructions are unsupported"        .into()),
        Instruction::LandingPad(_) | Instruction::CatchPad(_) | Instruction::CleanupPad(_)             => Err("Exception handling instructions are unsupported"        .into())
    }
}
