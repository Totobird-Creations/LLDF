mod module;
pub use module::*;
mod instr;
pub use instr::*;
mod oper;
pub use oper::*;

use super::codegen::CodeValue;

use std::error::Error;

use llvm_ir::Name;


#[derive(Clone, Debug)]
pub enum Value {

    /// Is technically a pointer, but will behave as if it is directly accessing the underlying value.
    /// Used where the target of the pointer is known at compile-time.
    PtrImmediate(Box<Value>),

    PtrInteger(Box<Value>),

    /// A reference to a global.
    GlobalRef(Name),

    /// Direct access to a local variable.
    /// **Can NOT be used as a pointer**
    Local(Name),

    Constant(CodeValue)

}


impl Value {
    pub fn to_codevalue(&self, module : &ParsedModule) -> Result<CodeValue, Box<dyn Error>> { match (self) {
        Self::PtrImmediate (value) => value.to_codevalue(module),
        Self::PtrInteger   (value) => value.to_codevalue(module),
        Self::GlobalRef(name) => {
            match (&module.globals[&name]) {
                Global::NoopFunction             => Err("Function can not be used as a value".into()),
                Global::UserFunction      { .. } => Err("Function can not be used as a value".into()),
                Global::ActionFunction    { .. } => Err("Function can not be used as a value".into()),
                Global::GamevalueFunction { .. } => Err("Function can not be used as a value".into()),
            }
        },
        Self::Local    (name  ) => Ok(CodeValue::line_variable(name)),
        Self::Constant (value ) => Ok(value.clone())
    } }
}
