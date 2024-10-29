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
    GlobalReference(Name),

    /// A 'faked' pointer with known get and set behaviour.
    SetGetPtr {
        getter_codeblock : String,
        getter_action    : String,
        getter_tags      : Vec<CodeValue>,
        setter_codeblock : String,
        setter_action    : String,
        setter_tags      : Vec<CodeValue>,
        parameters       : Vec<Value>
    },

    /// Direct access to a local value.
    /// Should NEVER be a pointer.
    Local(Name),

    /// A DiamondFire code value.
    CodeValue(CodeValue)

}

impl Value {
    pub fn to_codevalue(&self, module : &ParsedModule, function : &mut ParsedFunction) -> Result<CodeValue, Box<dyn Error>> { match (self) {

        Value::GlobalReference(name) => todo!(),

        Value::SetGetPtr { getter_codeblock, getter_action, getter_tags, parameters, .. } => { // This is a read-from-ptr operation.
            let temp_var = CodeValue::line_variable(function.create_temp_var_name());
            let mut params = Vec::with_capacity(parameters.len() + 1);
            params.push(temp_var.clone());
            for param in parameters {
                params.push(param.to_codevalue(module, function)?);
            }
            function.line.blocks.push(Codeblock::action(getter_codeblock, getter_action, params, getter_tags.clone())); // TODO: params
            Ok(temp_var)
        },

        Value::Local(name) => {
            let Some(value) = function.locals.get(name) else { return Err(format!("Unknown local {}", name).into()) };
            value.clone().to_codevalue(module, function)
        },

        Value::CodeValue(value) => Ok(value.clone())

    } }
}
