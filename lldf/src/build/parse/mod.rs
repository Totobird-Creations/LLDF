mod module;
pub use module::*;
mod instr;
pub use instr::*;
mod oper;
pub use oper::*;

use std::error::Error;

use llvm_ir::Name;

use super::codegen::{CodeValue, Codeblock};


#[derive(Clone, Debug)]
pub enum Value {

    /// A reference to a global value.
    GlobalRef(Name),

    /// A 'faked' pointer with known get and set behaviour.
    GetSetPtr {
        getter           : GSPGetter,
        setter_codeblock : String,
        setter_action    : String,
        setter_tags      : Vec<CodeValue>,
        parameters       : Vec<Value>
    },

    /// A pointer that was converted to an integer.
    /// Due to DiamondFire restrictions, can not be used as
    ///   an integer, but can be converted back to a pointer.
    IntPtr(Box<Value>),

    /// Direct access to a local value.
    /// Should NEVER be used as a pointer.
    Local(Name),

    /// A DiamondFire code value.
    CodeValue(CodeValue)

}


#[derive(Clone, Debug)]
pub enum GSPGetter {
    Codeblock {
        codeblock : String,
        action    : String,
        tags      : Vec<CodeValue>
    },
    Local(Name)
}


impl Value {
    pub fn to_codevalue(&self, module : &ParsedModule, function : &mut ParsedFunction) -> Result<CodeValue, Box<dyn Error>> { match (self) {

        Value::GlobalRef(name) => {
            let Some(global) = module.globals.get(name) else { return Err(format!("Unknown global {}", name).into()) };
            match (global) {
                
                Global::NoopFunction => Err("No-op function can not be converted to a code value".into()),

                Global::UserFunction { .. } => Err("Function can not be converted to a code value".into()),

                Global::ActionFunction { .. } => Err("Action function can not be converted to a code value".into()),

                Global::ActionPtrFunction { .. } => Err("Action-pointer function can not be converted to a code value".into()),

                Global::GamevalueFunction { .. } => Err("Gamevalue function can not be converted to a code value".into()),

                Global::Constant(value) => value.to_codevalue(module, function),

            }
        },

        Value::GetSetPtr { getter, parameters, .. } => getter.to_codevalue(module, function, parameters),

        Value::IntPtr(value) => value.to_codevalue(module, function),

        Value::Local(name) => {
            let Some(value) = function.locals.get(name) else { return Err(format!("Unknown local {}", name).into()) };
            value.clone().to_codevalue(module, function)
        },

        Value::CodeValue(value) => Ok(value.clone())

    } }
}


impl GSPGetter {
    pub fn to_codevalue(&self, module : &ParsedModule, function : &mut ParsedFunction, parameters : &Vec<Value>) -> Result<CodeValue, Box<dyn Error>> { match (self) {
        // This is a ptr-load operation.

        GSPGetter::Codeblock { codeblock, action, tags } => {
            let dest = CodeValue::line_variable(function.create_temp_var_name());
            let mut params = Vec::with_capacity(parameters.len() + 1);
            params.push(dest.clone());
            for param in parameters {
                params.push(param.to_codevalue(module, function)?);
            }
            function.line.blocks.push(Codeblock::action(codeblock, action, params, tags.clone()));
            Ok(dest)
        },

        GSPGetter::Local(name) => Ok(CodeValue::line_variable_name(name)),

    } }
}
