use super::*;

use std::mem::transmute;

use llvm_ir::operand::*;
use llvm_ir::constant::*;




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

    Constant::Null(_) => Ok(Value::Null),

    Constant::AggregateZero(_) => todo!(),

    Constant::Struct { .. } => todo!(),

    Constant::Array { elements, .. } => { // TODO: Handle >26 element lists.
        let temp_var = function.create_temp_var_name();
        let mut params = Vec::with_capacity(elements.len() + 1);
        params.push(CodeValue::Variable { name : temp_var.clone(), scope : VariableScope::Local });
        for element in elements {
            params.push(parse_const(module, function, element)?.to_codevalue(module, function)?);
        }
        function.line.blocks.push(Codeblock::action("set_var", "CreateList", params, vec![ ]));
        Ok(Value::Local(temp_var))
    },

    Constant::Undef(_) => Ok(Value::Null),

    Constant::Poison(_) => Ok(Value::Null),

    Constant::GlobalReference { name, .. } => Ok(Value::Global(name.clone())),

    Constant::Add(Add { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_const(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "+", vec![
            CodeValue::Variable { name : temp_var.clone(), scope: VariableScope::Local },
            operand0, operand1
        ], vec![ ]));
        Ok(Value::Local(temp_var))
    },

    Constant::Sub(Sub { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_const(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "-", vec![
            CodeValue::Variable { name : temp_var.clone(), scope: VariableScope::Local },
            operand0, operand1
        ], vec![ ]));
        Ok(Value::Local(temp_var))
    },

    Constant::Mul(Mul { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_const(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "x", vec![
            CodeValue::Variable { name : temp_var.clone(), scope: VariableScope::Local },
            operand0, operand1
        ], vec![ ]));
        Ok(Value::Local(temp_var))
    },

    Constant::And(And { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_const(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", vec![
            CodeValue::Variable { name : temp_var.clone(), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "&".to_string(), variable : None, block_override : None }
        ]));
        Ok(Value::Local(temp_var))
    },

    Constant::Or(Or { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_const(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", vec![
            CodeValue::Variable { name : temp_var.clone(), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "|".to_string(), variable : None, block_override : None }
        ]));
        Ok(Value::Local(temp_var))
    },

    Constant::Xor(Xor { operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?.to_codevalue(module, function)?;
        let operand1 = parse_const(module, function, operand1)?.to_codevalue(module, function)?;
        function.line.blocks.push(Codeblock::action("set_var", "Bitwise", vec![
            CodeValue::Variable { name : temp_var.clone(), scope: VariableScope::Local },
            operand0, operand1
        ], vec![
            CodeValue::Actiontag { kind : "Operator".to_string(), value : "^".to_string(), variable : None, block_override : None }
        ]));
        Ok(Value::Local(temp_var))
    },

    Constant::Shl(_) => todo!(),

    Constant::LShr(_) => todo!(),

    Constant::AShr(_) => todo!(),

    Constant::GetElementPtr(_) => todo!(),

    Constant::Trunc(Trunc { operand, .. }) => parse_const(module, function, operand),

    Constant::ZExt(ZExt { operand, .. }) => parse_const(module, function, operand),

    Constant::SExt(SExt { operand, .. }) => parse_const(module, function, operand),

    Constant::FPTrunc(FPTrunc { operand, .. }) => parse_const(module, function, operand),

    Constant::FPExt(FPExt { operand, .. }) => parse_const(module, function, operand),

    Constant::FPToUI(_) => todo!(),

    Constant::FPToSI(_) => todo!(),

    Constant::UIToFP(UIToFP { operand, .. }) => parse_const(module, function, operand),

    Constant::SIToFP(SIToFP { operand, .. }) => parse_const(module, function, operand),

    Constant::PtrToInt(PtrToInt { operand, .. }) => parse_const(module, function, operand), // TODO: Make sure this works

    Constant::IntToPtr(IntToPtr { operand, .. }) => parse_const(module, function, operand), // TODO: Make sure this works

    Constant::ICmp(ICmp { predicate, operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        parse_icmp(module, function, &temp_var, *predicate, operand0, operand1)?;
        Ok(Value::Local(temp_var))
    },

    Constant::FCmp(FCmp { predicate, operand0, operand1 }) => {
        let temp_var = function.create_temp_var_name();
        let operand0 = parse_const(module, function, operand0)?;
        let operand1 = parse_const(module, function, operand1)?;
        parse_fcmp(module, function, &temp_var, *predicate, operand0, operand1)?;
        Ok(Value::Local(temp_var))
    },

    Constant::Select(Select { condition, true_value, false_value }) => {
        let temp_var = function.create_temp_var_name();
        let condition   = parse_const(module, function, condition   )?;
        let true_value  = parse_const(module, function, true_value  )?;
        let false_value = parse_const(module, function, false_value )?;
        parse_select(module, function, &temp_var, condition, true_value, false_value)?;
        Ok(Value::Local(temp_var))
    },


    Constant::BlockAddress                                                                                      => Err("Block address operands are unsupported"      .into()),
    Constant::TokenNone                                                                                         => Err("Token operands are unsupported"              .into()),
    Constant::Vector(_) | Constant::ExtractElement(_) | Constant::InsertElement(_) | Constant::ShuffleVector(_) => Err("Vector operands are unsupported"             .into()),
    Constant::BitCast(_)                                                                                        => Err("Bit cast operands are unsupported"           .into()),
    Constant::AddrSpaceCast(_)                                                                                  => Err("Address space cast operands are unsupported" .into()),
} }





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
