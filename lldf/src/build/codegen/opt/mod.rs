mod dead_selections;
pub use dead_selections::dead_selections;
mod duplicate_selections;
pub use duplicate_selections::duplicate_selections;
mod redundant_equals;
pub use redundant_equals::redundant_equals;
mod substitutable_arithmetic;
pub use substitutable_arithmetic::substitutable_arithmetic;
mod substitutable_string;
pub use substitutable_string::substitutable_string;
mod substitutable_text;
pub use substitutable_text::substitutable_text;


use super::{ Codeblock, CodeLine };


pub fn optimise(line : &mut CodeLine) -> () {
    let mut did_something = true;
    while (did_something) {
        did_something = false;
        did_something &= dead_selections(line);
        did_something &= duplicate_selections(line);
        did_something &= redundant_equals(line);
        did_something &= substitutable_arithmetic(line);
        did_something &= substitutable_string(line);
        did_something &= substitutable_text(line);
    }
}
