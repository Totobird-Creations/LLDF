use super::*;

use llvm_ir::operand::*;
use llvm_ir::constant::*;




pub fn parse_oper(module : &ParsedModule, parsed : &mut ParsedFunction, oper : &Operand) -> Result<Value, Box<dyn Error>> { match (oper) {
    Operand::LocalOperand    { name, .. } => Ok(parsed.locals[name].clone()),
    Operand::ConstantOperand ( cor      ) => parse_const(module, parsed, cor),
    Operand::MetadataOperand              => Err("Metadata operands are unsupported".into()),
} }




pub fn parse_const(module : &ParsedModule, parsed : &mut ParsedFunction, cor : &Constant) -> Result<Value, Box<dyn Error>> { match (cor) {
    Constant::Int { value, .. } => todo!(),
    Constant::Float(_) => todo!(),
    Constant::Null(_) => todo!(),
    Constant::AggregateZero(_) => todo!(),
    Constant::Struct { .. } => todo!(),
    Constant::Array { ..} => todo!(),
    Constant::Undef(_) => todo!(),
    Constant::Poison(_) => todo!(),
    Constant::GlobalReference { name, .. } => todo!(),
    Constant::Add(_) => todo!(),
    Constant::Sub(_) => todo!(),
    Constant::Mul(_) => todo!(),
    Constant::Xor(_) => todo!(),
    Constant::Shl(_) => todo!(),
    Constant::GetElementPtr(_) => todo!(),
    Constant::Trunc(_) => todo!(),
    Constant::PtrToInt(PtrToInt { operand, .. }) => todo!(),
    Constant::IntToPtr(_) => todo!(),
    Constant::ICmp(_) => todo!(),
    Constant::FCmp(_) => todo!(),

    Constant::Vector(_) | Constant::ExtractElement(_) | Constant::InsertElement(_) | Constant::ShuffleVector(_) => Err("Vector operands are unsupported"             .into()),
    Constant::BlockAddress                                                                                      => Err("Block address operands are unsupported"      .into()),
    Constant::TokenNone                                                                                         => Err("Token operands are unsupported"              .into()),
    Constant::BitCast(_)                                                                                        => Err("Bit cast operands are unsupported"           .into()),
    Constant::AddrSpaceCast(_)                                                                                  => Err("Address space cast operands are unsupported" .into()),
} }
