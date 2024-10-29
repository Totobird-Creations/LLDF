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
    Constant::Vector(_) => todo!(),
    Constant::Undef(_) => todo!(),
    Constant::Poison(_) => todo!(),
    Constant::BlockAddress => todo!(),
    Constant::GlobalReference { name, .. } => todo!(),
    Constant::TokenNone => todo!(),
    Constant::Add(_) => todo!(),
    Constant::Sub(_) => todo!(),
    Constant::Mul(_) => todo!(),
    Constant::Xor(_) => todo!(),
    Constant::Shl(_) => todo!(),
    Constant::ExtractElement(_) => todo!(),
    Constant::InsertElement(_) => todo!(),
    Constant::ShuffleVector(_) => todo!(),
    Constant::GetElementPtr(_) => todo!(),
    Constant::Trunc(_) => todo!(),
    Constant::PtrToInt(PtrToInt { operand, .. }) => todo!(),
    Constant::IntToPtr(_) => todo!(),
    Constant::BitCast(_) => todo!(),
    Constant::AddrSpaceCast(_) => todo!(),
    Constant::ICmp(_) => todo!(),
    Constant::FCmp(_) => todo!(),
} }
