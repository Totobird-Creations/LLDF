mod module;
pub use module::*;
mod instr;
pub use instr::*;
mod oper;
pub use oper::*;

use std::error::Error;

use llvm_ir::Name;

use super::codegen::{CodeValue, VariableScope};


#[derive(Clone, Debug)]
pub enum Value {

    // Constants
    ConstInt(u64),
    ConstFloat(f64),
    ConstString(String),

    Local(Name),
    Global(Name)

}
impl Value {


    pub fn to_codevalue(&self, module : &ParsedModule) -> Result<CodeValue, Box<dyn Error>> { match (self) {

        Self::ConstInt    (value) => Ok(CodeValue::Number(value.to_string())),
        Self::ConstFloat  (value) => Ok(CodeValue::Number(value.to_string())),
        Self::ConstString (value) => Ok(CodeValue::String(value.clone())),

        Self::Local(name) => Ok(CodeValue::Variable { name : name_to_local(name), scope : VariableScope::Line }),

        Self::Global(name) => {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {

                Global::UserFunction { name } => Ok(CodeValue::String(name.clone())),

                Global::Constant(value) => value.to_codevalue(module),

                Global::NoopFunction |
                Global::ActionFunction { .. } |
                Global::ActionPtrFunction { .. } |
                Global::GamevalueFunction { .. }
                    => unreachable!(),

            }
        }

    } }


    pub fn to_ptr_accessor_part_strings(&self, module : &ParsedModule) -> Result<(String, String), Box<dyn Error>> { match (self) {

        Self::ConstInt(_) | Self::ConstFloat(_) | Self::ConstString(_)
            => { Err("Can not use constant as pointer accessor".into()) },

        Self::Local(name) => {
            let name = name_to_local(name);
            Ok((format!("%index({},1)", name), format!("%index({},2)", name)))
        },

        Self::Global(name) => {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {

                Global::Constant(value) => value.to_ptr_accessor_part_strings(module),

                Global::NoopFunction |
                Global::UserFunction { .. } |
                Global::ActionFunction { .. } |
                Global::ActionPtrFunction { .. } |
                Global::GamevalueFunction { .. }
                    => unreachable!(),

            }
        }

    } }


    pub fn to_ptr_accessor_string(&self, module : &ParsedModule) -> Result<String, Box<dyn Error>> { match (self) {

        Self::ConstInt(_) | Self::ConstFloat(_) | Self::ConstString(_)
            => { Err("Can not use constant as pointer accessor".into()) },

        Self::Local(name) => {
            let name = name_to_local(name);
            Ok(format!("%index({},1):%index({},2)", name, name))
        },

        Self::Global(name) => {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {

                Global::UserFunction { name } => Ok(name.clone()),

                Global::Constant(value) => value.to_ptr_accessor_string(module),

                Global::NoopFunction |
                Global::ActionFunction { .. } |
                Global::ActionPtrFunction { .. } |
                Global::GamevalueFunction { .. }
                    => unreachable!(),

            }
        }

    } }


    pub fn to_ptr_accessor_codevalue(&self, module : &ParsedModule) -> Result<CodeValue, Box<dyn Error>> {
        Ok(CodeValue::Variable { name : self.to_ptr_accessor_string(module)?, scope : VariableScope::Unsaved })
    }


}


pub fn name_to_local(name : &Name) -> String { match (name) {
    Name::Name   (name   ) => format!("local.name.{}", name),
    Name::Number (number ) => format!("local.number.{}", number),
} }
pub fn name_to_global(name : &Name) -> String { match (name) {
    Name::Name   (name   ) => format!("global.name.{}", name),
    Name::Number (number ) => format!("global.number.{}", number),
} }
