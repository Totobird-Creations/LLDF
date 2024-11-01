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

    Constant::Int { value, .. } => Ok(Value::CodeValue(CodeValue::Number(*value as f64))), // TODO: Signed vs Unsigned

    Constant::Float(_) => todo!(),
    Constant::Null(_) => todo!(),
    Constant::AggregateZero(_) => todo!(),

    Constant::Struct { values, .. } => parse_aggregate(module, function, values),

    Constant::Array { elements, .. } => parse_aggregate(module, function, elements),

    Constant::Undef(_) => todo!(),
    Constant::Poison(_) => todo!(),

    Constant::GlobalReference { name, .. } => Ok(Value::GlobalRef(name.clone())),

    Constant::Add(_) => todo!(),
    Constant::Sub(_) => todo!(),
    Constant::Mul(_) => todo!(),
    Constant::Xor(_) => todo!(),
    Constant::Shl(_) => todo!(),
    Constant::GetElementPtr(_) => todo!(),
    Constant::Trunc(_) => todo!(),

    Constant::PtrToInt(PtrToInt { operand, .. }) => Ok(Value::IntPtr(Box::new(parse_const(module, function, operand)?))),

    Constant::IntToPtr(_) => todo!(),

    Constant::ICmp(_) => todo!(),
    Constant::FCmp(_) => todo!(),

    Constant::Vector(_) | Constant::ExtractElement(_) | Constant::InsertElement(_) | Constant::ShuffleVector(_) => Err("Vector operands are unsupported"             .into()),
    Constant::BlockAddress                                                                                      => Err("Block address operands are unsupported"      .into()),
    Constant::TokenNone                                                                                         => Err("Token operands are unsupported"              .into()),
    Constant::BitCast(_)                                                                                        => Err("Bit cast operands are unsupported"           .into()),
    Constant::AddrSpaceCast(_)                                                                                  => Err("Address space cast operands are unsupported" .into()),
} }




pub fn parse_aggregate(module : &ParsedModule, function : &mut ParsedFunction, values : &Vec<ConstantRef>) -> Result<Value, Box<dyn Error>> {
    let var = CodeValue::line_variable(function.create_temp_var_name());
    let mut first = true;
    for chunk in values.chunks(26) {
        let mut params = Vec::with_capacity(chunk.len() + 1);
        params.push(var.clone());
        for param in chunk {
            params.push(parse_const(module, function, param)?.to_codevalue(module, function)?);
        }
        function.line.blocks.push(Codeblock::action("set_var",
        if (first) { "CreateList" } else { "AppendValue" },
        params, vec![]
    ));
        if (first) { first = false; }
    }
    Ok(Value::CodeValue(var))
}




pub fn parse_special_const(value : &ConstantRef) -> Option<Value> {

    // Strings
    'string_failed : loop {
        if let Constant::Struct { values, is_packed : true, .. } = &**value {
            if (values.len() == 1) { if let Constant::Array { elements, .. } = &*values[0] {
                let mut bytes = Vec::with_capacity(elements.len());
                for element in elements { if let Constant::Int { bits : 8, value } = &**element {
                    bytes.push(*value as u8);
                } else { break 'string_failed; } }
                if let Ok(string) = String::from_utf8(bytes) {
                    return Some(Value::CodeValue(CodeValue::String(string)));
                }
            }}
        }
        break 'string_failed;
    }

    None
}
