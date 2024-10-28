use crate::build::codegen::{CodeValue, Codeblock};
use super::*;

use llvm_ir::{function::ParameterAttribute, instruction::*, Operand};




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
    Instruction::Shl(_) => todo!(),
    Instruction::LShr(_) => todo!(),
    Instruction::AShr(_) => todo!(),
    Instruction::FAdd(_) => todo!(),
    Instruction::FSub(_) => todo!(),
    Instruction::FMul(_) => todo!(),
    Instruction::FDiv(_) => todo!(),
    Instruction::FRem(_) => todo!(),
    Instruction::FNeg(_) => todo!(),
    Instruction::ExtractElement(_) => todo!(),
    Instruction::InsertElement(_) => todo!(),
    Instruction::ShuffleVector(_) => todo!(),
    Instruction::ExtractValue(_) => todo!(),
    Instruction::InsertValue(_) => todo!(),

    Instruction::Alloca(Alloca { dest, .. }) => {
        let temp_var = parsed.create_temp_var();
        // TODO: Insert a SetVar codeblock for undefined?
        parsed.locals.insert(dest.clone(), Value::PtrImmediate(Box::new(Value::Local(temp_var))));
        Ok(())
    },

    Instruction::Load(Load { address, dest, .. }) => {
        let address = parse_oper(module, parsed, address)?;
        match (address) {
            Value::PtrImmediate(value) => {
                // TODO: Insert a SetVar codeblock.
                parsed.locals.insert(dest.clone(), *value);
                Ok(())
            },
            Value::PtrInteger(_) => Err("(ptr)Integer can not be loaded as a pointer".into()),
            Value::GlobalRef(name) => todo!(),
            Value::Local    (name ) => Err(format!("Local {} can not be loaded as a pointer", name).into()),
            Value::Constant (_    ) => Err("Constant can not be loaded as a pointer".into())
        }
    },

    Instruction::Store(_) => todo!(),
    Instruction::Fence(_) => todo!(),
    Instruction::CmpXchg(_) => todo!(),
    Instruction::AtomicRMW(_) => todo!(),
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
    Instruction::BitCast(_) => todo!(),
    Instruction::AddrSpaceCast(_) => todo!(),
    Instruction::ICmp(_) => todo!(),
    Instruction::FCmp(_) => todo!(),
    Instruction::Phi(_) => todo!(),
    Instruction::Select(_) => todo!(),
    Instruction::Freeze(_) => todo!(),

    Instruction::Call(Call { function, arguments, dest, .. }) => {
        let Some(function) = function.as_ref().right() else { return Err("Inline assembly is unsupported".into()) };
        let function = parse_oper(module, parsed, function)?;
        handle_call(module, parsed, &function, arguments, dest)
    },

    Instruction::VAArg(_) => todo!(),
    Instruction::LandingPad(_) => todo!(),
    Instruction::CatchPad(_) => todo!(),
    Instruction::CleanupPad(_) => todo!(),
} }




fn handle_call(module : &ParsedModule, parsed : &mut ParsedFunction, function : &Value, arguments : &Vec<(Operand, Vec<ParameterAttribute>)>, dest : &Option<Name>) -> Result<(), Box<dyn Error>> {
    match (function) {

        Value::Local(name) => Err(format!("Local {} can not be called as a function", name).into()),

        Value::PtrImmediate(value) => handle_call(module, parsed, &*value, arguments, dest),

        Value::PtrInteger(value) => Err("(ptr)Integer {} can not be called as a function".into()),

        Value::GlobalRef(name) => {
            match (&module.globals[&name]) {
                Global::NoopFunction => { },
                Global::UserFunction  { name } => {
                    parsed.line.blocks.push(Codeblock::call_func(name, vec![])) // TODO: params and return
                },
                Global::ActionFunction { codeblock, action, tags } => {
                    let mut args    = arguments.iter();
                    let param_count = arguments.len() - tags.len();
                    let mut params  = Vec::with_capacity(param_count);
                    for _ in 0..param_count {
                        let param = parse_oper(module, parsed, &args.next().unwrap().0)?;
                        params.push(param.to_codevalue(module)?);
                    }
                    parsed.line.blocks.push(Codeblock::action(codeblock, action, params, vec![])) // TODO: params and tags
                },
                Global::GamevalueFunction { kind, target } => {
                    let dest = dest.as_ref().unwrap();
                    let params = vec![
                        CodeValue::line_variable(dest),
                        CodeValue::Gamevalue { kind : kind.clone(), target : target.clone() }
                    ];
                    parsed.line.blocks.push(Codeblock::action("set_var", "=", params, vec![]));
                    parsed.locals.insert(dest.clone(), Value::Local(dest.clone()));
                }
            }
            Ok(())
        },

        Value::Constant(_) => Err("Constant can not be called as a function".into())

    }
}
