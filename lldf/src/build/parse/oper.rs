use super::*;

use llvm_ir::operand::*;
use llvm_ir::constant::*;




pub fn parse_oper(module : &ParsedModule, function : &mut ParsedFunction, oper : &Operand) -> Result<Value, Box<dyn Error>> { match (oper) {
    Operand::LocalOperand { name, .. } => {
        let Some(value) = function.locals.get(name) else { return Err(format!("Unknown local {}", name).into()) };
        Ok(value.clone())
    },
    Operand::ConstantOperand(cor) => parse_const(module, function, cor),
    Operand::MetadataOperand      => Err("Metadata operands are unsupported".into()),
} }




pub fn parse_const(module : &ParsedModule, function : &mut ParsedFunction, cor : &Constant) -> Result<Value, Box<dyn Error>> { match (cor) {

    Constant::Int { value, .. } => Ok(Value::ConstInt(*value)), // Unsigned vs signed

    Constant::Float(value) => match (value) {
        Float::Half          => Err("Half floats are unsupported"   .into()),
        Float::BFloat        => Err("BFloat floats are unsupported" .into()),
        Float::Single(value) => Ok(Value::ConstFloat(*value as f64)),
        Float::Double(value) => Ok(Value::ConstFloat(*value)),
        Float::Quadruple     => Err("Quadruple floats are unsupported" .into()),
        Float::X86_FP80      => Err("X86_FP80 floats are unsupported"  .into()),
        Float::PPC_FP128     => Err("PPC_FP128 floats are unsupported" .into()),
    },

    Constant::Null(_) => todo!(),

    Constant::AggregateZero(_) => todo!(),

    Constant::Struct { .. } => todo!(),

    Constant::Array { .. } => todo!(),

    Constant::Vector(_) => todo!(),

    Constant::Undef(_) => todo!(),

    Constant::Poison(_) => todo!(),

    Constant::GlobalReference { name, .. } => Ok(Value::Global(name.clone())),

    Constant::Add(_) => todo!(),

    Constant::Sub(_) => todo!(),

    Constant::Mul(_) => todo!(),

    Constant::Xor(_) => todo!(),

    Constant::Shl(_) => todo!(),

    Constant::GetElementPtr(_) => todo!(),

    Constant::Trunc(Trunc { operand, .. }) => parse_const(module, function, operand),

    Constant::PtrToInt(PtrToInt { operand, .. }) => parse_const(module, function, operand),

    Constant::IntToPtr(_) => todo!(),

    Constant::ICmp(_) => todo!(),

    Constant::FCmp(_) => todo!(),


    Constant::ExtractElement(_) | Constant::InsertElement(_) | Constant::ShuffleVector(_) => Err("Vector operands are unsupported"             .into()),
    Constant::BlockAddress                                                                => Err("Block address operands are unsupported"      .into()),
    Constant::TokenNone                                                                   => Err("Token operands are unsupported"              .into()),
    Constant::BitCast(_)                                                                  => Err("Bit cast operands are unsupported"           .into()),
    Constant::AddrSpaceCast(_)                                                            => Err("Address space cast operands are unsupported" .into()),
} }





pub fn handle_special_const(value : &ConstantRef) -> Option<Value> {

    // Strings
    'string_failed : loop {
        if let Constant::Struct { values, is_packed : true, .. } = &**value {
            if (values.len() == 1) { if let Constant::Array { elements, .. } = &*values[0] {
                let mut bytes = Vec::with_capacity(elements.len());
                for element in elements { if let Constant::Int { bits : 8, value } = &**element {
                    bytes.push(*value as u8);
                } else { break 'string_failed; } }
                if let Ok(string) = String::from_utf8(bytes) {
                    return Some(Value::ConstString(string));
                }
            }}
        }
        break 'string_failed;
    }

    None
}
