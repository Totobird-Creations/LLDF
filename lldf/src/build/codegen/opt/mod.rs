mod constant_propagation;
pub use constant_propagation::*;
// TODO: constant_conditional
mod dead_selection;
pub use dead_selection::*;
mod duplicate_selection;
pub use duplicate_selection::*;
mod dead_assignment;
pub use dead_assignment::*;
mod dead_conditional;
pub use dead_conditional::*;


use super::*;
use crate::build::parse::ParsedFunction;


pub fn optimise(mut functions : Vec<&mut ParsedFunction>) -> () {
    let mut did_nothing = 0;
    while (did_nothing < 2) {
        let mut did_something = false;

        for _ in 0..functions.len() {
            let function = functions.remove(0);
            let line     = &mut function.line;

            did_something |= constant_propagation(line, &functions);
            did_something |= dead_selection(line);
            did_something |= duplicate_selection(line);
            did_something |= dead_assignment(line, &functions);
            did_something |= dead_conditional(line);

            functions.push(function);
        }

        did_nothing = if (did_something) { 0 } else { did_nothing + 1 };
    }
}
