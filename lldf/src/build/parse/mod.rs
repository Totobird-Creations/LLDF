mod instr;
pub use instr::handle_instr;
mod call;
pub use call::handle_call;
mod operand;
pub use operand::{ handle_operand, handle_constant };



use llvm_ir::Name;


pub fn name_to_local(name : &Name) -> String { match (name) {
    Name::Name   (name   ) => format!("local.name.{}",   name   ),
    Name::Number (number ) => format!("local.number.{}", number )
} }

pub fn name_to_global(name : &Name) -> String { match (name) {
    Name::Name   (name   ) => format!("global.name.{}",   name   ),
    Name::Number (number ) => format!("global.number.{}", number )
} }


/*
pub fn handle_operand(operand : &Operand) -> Result<(), Box<dyn Error>> { match (operand) {
    Operand::LocalOperand { .. } => todo!(),
    Operand::ConstantOperand(co) => handle_constant(co),
    Operand::MetadataOperand => todo!(),
} }


pub fn handle_constant(constant : &Constant) -> Result<(), Box<dyn Error>> { match (&*constant) {
    Constant::Int { .. } => todo!(),
    Constant::Float(_) => todo!(),
    Constant::Null(_) => todo!(),
    Constant::AggregateZero(_) => todo!(),
    Constant::Struct { .. } => todo!(),
    Constant::Array { .. } => todo!(),
    Constant::Undef(_) => todo!(),
    Constant::Poison(_) => todo!(),
    Constant::GlobalReference { .. } => todo!(),
    Constant::Add(_) => todo!(),
    Constant::Sub(_) => todo!(),
    Constant::Mul(_) => todo!(),
    Constant::Xor(_) => todo!(),
    Constant::Shl(_) => todo!(),
    Constant::GetElementPtr(_) => todo!(),
    Constant::Trunc(_) => todo!(),
    Constant::ICmp(_) => todo!(),
    Constant::FCmp(_) => todo!(),


    Constant::Vector(_) | Constant::ExtractElement(_) | Constant::InsertElement(_) | Constant::ShuffleVector(_) => { Err("SIMD vector constants are unsupported"                .into()) },
    Constant::BlockAddress                                                                                      => { Err("Block address constants are unsupported"              .into()) },
    Constant::TokenNone                                                                                         => { Err("Token constants are unsupported"                      .into()) },
    Constant::PtrToInt(_) | Constant::IntToPtr(_)                                                               => { Err("Pointer-integer conversion constants are unsupported" .into()) }
    Constant::BitCast(_)                                                                                        => { Err("Bit cast constants are unsupported"                   .into()) },
    Constant::AddrSpaceCast(_)                                                                                  => { Err("Address space cast constants are unsupported"         .into()) },
} }
*/