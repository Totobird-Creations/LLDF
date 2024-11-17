mod module;
pub use module::*;
mod function;
pub use function::*;
mod instr;
pub use instr::*;
mod oper;
pub use oper::*;
mod common;
pub use common::*;

use std::error::Error;

use llvm_ir::Name;
use llvm_ir::{ FPPredicate, IntPredicate };

use super::codegen::{ Codeblock, CodeValue, SoundKind, VariableScope };


#[derive(Clone, Debug)]
pub enum Value {

    // Constants
    Null,
    ConstString(String),
    ConstInt(i64),
    ConstFloat(f64),

    Local(String),
    Global(String),
    GlobalRef(Name)

}
impl Value {


    pub fn to_codevalue(&self, module : &ParsedModule, function : &mut ParsedFunction) -> Result<CodeValue, Box<dyn Error>> { match (self) {

        Self::Null                 => Ok(CodeValue::Number("0".to_string())),
        Self::ConstString (value ) => Ok(CodeValue::String(value.clone())),
        Self::ConstInt    (value ) => Ok(CodeValue::Number(value.to_string())),
        Self::ConstFloat  (value ) => Ok(CodeValue::Number(value.to_string())),

        Self::Local(name) => Ok(CodeValue::Variable { name : name.clone(), scope : VariableScope::Local }),
        Self::Global(name) => Ok(CodeValue::Variable { name : format!("{}:0", name), scope : VariableScope::Unsaved }),

        Self::GlobalRef(name) => {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {

                Global::UserFunction { name } => Ok(CodeValue::String(name.clone())),

                Global::Constant(value) => value.to_codevalue(module, function),

                Global::NoopFunction |
                Global::Assert(_) |
                Global::ActionFunction { .. } |
                Global::BracketFunction { .. } |
                Global::ElseFunction |
                Global::TempVarFunction |
                Global::GamevalueFunction { .. } |
                Global::SoundFunction { .. } |
                Global::ParticleFunction { .. } |
                Global::PotionFunction { .. } |
                Global::ItemFunction { .. }
                    => unreachable!(),

            }
        }

    } }


    pub fn to_ptr_accessor_part_strings(&self, module : &ParsedModule) -> Result<(String, String), Box<dyn Error>> { match (self) {

        Self::Null | Self::ConstString(_) | Self::ConstInt(_) | Self::ConstFloat(_)
            => { Err("Can not use constant as pointer accessor".into()) },

        Self::Local(name) => Ok((format!("%index({},1)", name), format!("%index({},2)", name))),
        Self::Global(name) => Ok((name.clone(), "0".to_string())),

        Self::GlobalRef(name) => {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {

                Global::Constant(value) => value.to_ptr_accessor_part_strings(module),

                Global::NoopFunction |
                Global::Assert(_) |
                Global::UserFunction { .. } |
                Global::ActionFunction { .. } |
                Global::BracketFunction { .. } |
                Global::ElseFunction |
                Global::TempVarFunction |
                Global::GamevalueFunction { .. } |
                Global::SoundFunction { .. } |
                Global::ParticleFunction { .. } |
                Global::PotionFunction { .. } |
                Global::ItemFunction { .. }
                    => unreachable!(),

            }
        }

    } }


    pub fn to_ptr_accessor_string(&self, module : &ParsedModule) -> Result<String, Box<dyn Error>> { match (self) {

        Self::Null | Self::ConstString(_) | Self::ConstInt(_) | Self::ConstFloat(_)
            => { Err("Can not use constant as pointer accessor".into()) },

        Self::Local(name) => Ok(format!("%index({},1):%index({},2)", name, name)),
        Self::Global(name) => Ok(format!("{}:0", name)),

        Self::GlobalRef(name) => {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {

                Global::UserFunction { name } => Ok(name.clone()),

                Global::Constant(value) => value.to_ptr_accessor_string(module),

                Global::NoopFunction |
                Global::Assert(_) |
                Global::ActionFunction { .. } |
                Global::BracketFunction { .. } |
                Global::ElseFunction |
                Global::TempVarFunction |
                Global::GamevalueFunction { .. } |
                Global::SoundFunction { .. } |
                Global::ParticleFunction { .. } |
                Global::PotionFunction { .. } |
                Global::ItemFunction { .. }
                    => unreachable!(),

            }
        }

    } }


    pub fn to_ptr_accessor_codevalue(&self, module : &ParsedModule) -> Result<CodeValue, Box<dyn Error>> {
        Ok(CodeValue::Variable { name : self.to_ptr_accessor_string(module)?, scope : VariableScope::Unsaved })
    }


}


pub fn name_to_string(name : &Name) -> String { match (name) {
    Name::Name   (name   ) => format!("name.{}", name),
    Name::Number (number ) => format!("number.{}", number),
} }
pub fn name_to_local(name : &Name) -> String { format!("local.#%var({}).{}", CALL, name_to_string(name)) }
pub fn name_to_global(name : &Name) -> String { format!("global.{}", name_to_string(name)) }
