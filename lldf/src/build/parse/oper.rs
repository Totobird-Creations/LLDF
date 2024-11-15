use super::*;

use std::mem::transmute;

use llvm_ir::operand::*;
use llvm_ir::constant::*;
use llvm_ir::Type;




pub fn parse_oper(module : &ParsedModule, function : &mut ParsedFunction, oper : &Operand) -> Result<Value, Box<dyn Error>> { match (oper) {
    Operand::LocalOperand { name, .. } => Ok(Value::Local(name_to_local(name))),
    Operand::ConstantOperand(cor)      => parse_const(module, function, cor),
    Operand::MetadataOperand           => Err("Metadata operands are unsupported".into()),
} }




pub fn parse_const(module : &ParsedModule, function : &mut ParsedFunction, cor : &Constant) -> Result<Value, Box<dyn Error>> { match (cor) {

    Constant::Int { value, .. } => Ok(Value::ConstInt(unsafe{ transmute(*value) })),

    Constant::Float(value) => match (value) {
        Float::Half          => Err("Half floats are unsupported"   .into()),
        Float::BFloat        => Err("BFloat floats are unsupported" .into()),
        Float::Single(value) => Ok(Value::ConstFloat(*value as f64)),
        Float::Double(value) => Ok(Value::ConstFloat(*value)),
        Float::Quadruple     => Err("Quadruple floats are unsupported" .into()),
        Float::X86_FP80      => Err("X86_FP80 floats are unsupported"  .into()),
        Float::PPC_FP128     => Err("PPC_FP128 floats are unsupported" .into()),
    },

    Constant::Null(_) | Constant::Undef(_) | Constant::Poison(_) => Ok(Value::Null),

    Constant::AggregateZero(typ) => handle_aggregate_type(module, function, typ),

    Constant::Struct { values, .. } => {
        let elements = values.iter().map(|element| parse_const(module, function, element)?.to_codevalue(module, function)).collect::<Result<Vec<_>, _>>()?;
        handle_aggregate(function, &elements)
    },

    Constant::Array { elements, .. } => {
        let elements = elements.iter().map(|element| parse_const(module, function, element)?.to_codevalue(module, function)).collect::<Result<Vec<_>, _>>()?;
        handle_aggregate(function, &elements)
    },

    Constant::GlobalReference { name, .. } => Ok(Value::Global(name.clone())),

    Constant::Add(Add { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_arithmetic(module, function, &temp_var, operand0, operand1, "+")?;
        Ok(Value::Local(temp_var))
    },

    Constant::Sub(Sub { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_arithmetic(module, function, &temp_var, operand0, operand1, "-")?;
        Ok(Value::Local(temp_var))
    },

    Constant::Mul(Mul { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_arithmetic(module, function, &temp_var, operand0, operand1, "x")?;
        Ok(Value::Local(temp_var))
    },

    Constant::And(And { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_bitwise(module, function, &temp_var, operand0, operand1, "&")?;
        Ok(Value::Local(temp_var))
    },

    Constant::Or(Or { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_bitwise(module, function, &temp_var, operand0, operand1, "|")?;
        Ok(Value::Local(temp_var))
    },

    Constant::Xor(Xor { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_bitwise(module, function, &temp_var, operand0, operand1, "^")?;
        Ok(Value::Local(temp_var))
    },

    Constant::Shl(Shl { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_bitwise(module, function, &temp_var, operand0, operand1, "<<")?;
        Ok(Value::Local(temp_var))
    },

    Constant::LShr(LShr { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_bitwise(module, function, &temp_var, operand0, operand1, ">>>")?;
        Ok(Value::Local(temp_var))
    },

    Constant::AShr(AShr { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_bitwise(module, function, &temp_var, operand0, operand1, ">>")?;
        Ok(Value::Local(temp_var))
    },

    Constant::GetElementPtr(GetElementPtr { address, indices, .. }) => {
        if (indices.len() != 1) { return Err("Multi-index GEP instructions are unsupported".into()); }
        let temp_var = function.create_temp_var_name();
        let address = parse_const(module, function, address)?;
        let index   = parse_const(module, function, &indices[0])?;
        handle_gep(module, function, &temp_var, address, index)?;
        Ok(Value::Local(temp_var))
    },

    Constant::Trunc(Trunc { operand, .. }) |
    Constant::ZExt(ZExt { operand, .. }) |
    Constant::SExt(SExt { operand, .. }) |
    Constant::FPTrunc(FPTrunc { operand, .. }) |
    Constant::FPExt(FPExt { operand, .. }) |
    Constant::UIToFP(UIToFP { operand, .. }) |
    Constant::SIToFP(SIToFP { operand, .. }) |
    Constant::BitCast(BitCast { operand, .. })
        => parse_const(module, function, operand),

    Constant::FPToUI(FPToUI { operand, .. }) | Constant::FPToSI(FPToSI { operand, .. }) => {
        let temp_var = function.create_temp_var_name();
        let operand = parse_const(module, function, operand)?;
        handle_trunc(module, function, &temp_var, operand)?;
        Ok(Value::Local(temp_var))
    },

    Constant::PtrToInt(PtrToInt { operand, .. }) |
    Constant::IntToPtr(IntToPtr { operand, .. })
        => parse_const(module, function, operand),

    Constant::ICmp(ICmp { predicate, operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_icmp(module, function, &temp_var, *predicate, operand0, operand1)?;
        Ok(Value::Local(temp_var))
    },

    Constant::FCmp(FCmp { predicate, operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        handle_fcmp(module, function, &temp_var, *predicate, operand0, operand1)?;
        Ok(Value::Local(temp_var))
    },

    Constant::Select(Select { condition, true_value, false_value }) => {
        let temp_var = function.create_temp_var_name();
        let condition   = parse_const(module, function, condition   )?;
        let true_value  = parse_const(module, function, true_value  )?;
        let false_value = parse_const(module, function, false_value )?;
        handle_select(module, function, &temp_var, condition, true_value, false_value)?;
        Ok(Value::Local(temp_var))
    },


    Constant::BlockAddress                                                                                      => Err("Block address operands are unsupported"      .into()),
    Constant::TokenNone                                                                                         => Err("Token operands are unsupported"              .into()),
    Constant::Vector(_) | Constant::ExtractElement(_) | Constant::InsertElement(_) | Constant::ShuffleVector(_) => Err("Vector operands are unsupported"             .into()),
    Constant::AddrSpaceCast(_)                                                                                  => Err("Address space cast operands are unsupported" .into()),
} }


fn handle_aggregate(function : &mut ParsedFunction, elements : &Vec<CodeValue>) -> Result<Value, Box<dyn Error>> {
    let temp_var = function.create_temp_var_name();
    let (chunks, remainder) = elements.as_chunks::<26>();
    for (i, chunk) in chunks.iter().enumerate() {
        let mut params = Vec::with_capacity(27);
        params.push(CodeValue::Variable { name : temp_var.clone(), scope : VariableScope::Local });
        params.extend_from_slice(chunk);
        function.line.blocks.push(Codeblock::action("set_var", if (i == 0) { "CreateList" } else { "AppendValue" }, params, vec![ ]));
    }
    if (remainder.len() > 0) {
        let mut params = Vec::with_capacity(27);
        params.push(CodeValue::Variable { name : temp_var.clone(), scope : VariableScope::Local });
        params.extend_from_slice(remainder);
        function.line.blocks.push(Codeblock::action("set_var", "AppendValue", params, vec![ ]));
    }
    Ok(Value::Local(temp_var))
}



fn handle_aggregate_type(module : &ParsedModule, function : &mut ParsedFunction, typ : &Type) -> Result<Value, Box<dyn Error>> {
    match (typ) {
        Type::VoidType | Type::IntegerType { .. } => Ok(Value::ConstInt(0)),
        Type::PointerType { .. }                  => handle_aggregate(function, &vec![ CodeValue::String("".to_string()), CodeValue::Number("0".to_string()) ]),
        Type::FPType(_)                                => Ok(Value::ConstFloat(0.0)),
        Type::FuncType { .. }                          => Ok(Value::ConstString("".to_string())),
        Type::ArrayType { num_elements, element_type } => {
            let element = handle_aggregate_type(module, function, element_type)?.to_codevalue(module, function)?;
            handle_aggregate(function, &vec![element; *num_elements])
        },
        Type::StructType { element_types, .. } => {
            let elements = element_types.iter().map(|typ| handle_aggregate_type(module, function, typ)?.to_codevalue(module, function)).collect::<Result<Vec<_>, _>>()?;
            handle_aggregate(function, &elements)
        },
        Type::NamedStructType { name } => handle_aggregate_type(module, function, &*module.types.named_struct(name)),
        Type::VectorType { .. } => Err("Vector types are unsupported".into()),
        Type::X86_MMXType       => Err("X86_MMX types are unsupported".into()),
        Type::X86_AMXType       => Err("X86_AMX types are unsupported".into()),
        Type::MetadataType      => Err("Metadata types are unsupported".into()),
        Type::LabelType         => Err("Label types are unsupported".into()),
        Type::TokenType         => Err("Token types are unsupported".into()),
        Type::TargetExtType     => Err("TargetExt types are unsupported".into())
    }
}




pub fn handle_special_const(value : &ConstantRef) -> Option<Value> {

    // Strings
    'string_failed : loop {
        if let Constant::Struct { values, is_packed : true, .. } = &**value {

            if (values.len() == 1) {
                if let Constant::Array { elements, .. } = &*values[0] {
                    let mut bytes = Vec::with_capacity(elements.len());
                    for element in elements { if let Constant::Int { bits : 8, value } = &**element {
                        bytes.push(*value as u8);
                    } else { break 'string_failed; } }
                    if let Ok(string) = String::from_utf8(bytes) {
                        return Some(Value::ConstString(string));
                    }
                }
            }

            else if (values.len() == 2) {
                if let Constant::Array { .. } = &*values[1] {
                    if let Constant::GlobalReference { name, .. } = &*values[0] {
                        return Some(Value::Global(name.clone()));
                    }
                }
            }

        }
        break 'string_failed;
    }

    None
}
