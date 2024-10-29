mod module;
pub use module::*;
mod instr;
pub use instr::*;
mod oper;
pub use oper::*;

use std::error::Error;

use llvm_ir::Name;


#[derive(Clone, Debug)]
pub enum Value {

}
