use super::*;

use llvm_ir::instruction::*;




pub fn parse_instr(module : &ParsedModule, parsed : &mut ParsedFunction, instr : &Instruction) -> Result<(), Box<dyn Error>> { match (instr) {
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

    Instruction::Alloca(Alloca { dest, .. }) => todo!(),

    Instruction::Load(Load { address, dest, .. }) => todo!(),

    Instruction::Store(_) => todo!(),
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

    Instruction::Call(Call { function, arguments, dest, .. }) => {
        let Some(function) = function.as_ref().right() else { return Err("Inline assembly is unsupported".into()) };
        //let function = parse_oper(module, parsed, function)?;
        todo!()
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
