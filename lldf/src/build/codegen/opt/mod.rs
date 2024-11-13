mod constant_propagation;
pub use constant_propagation::*;
mod dead_selection;
pub use dead_selection::*;
mod dead_assignment;
pub use dead_assignment::*;
mod dead_conditional;
pub use dead_conditional::*;
// TODO: duplicate_selection


use super::*;


pub fn optimise(line : &mut CodeLine) -> () {
    let mut did_nothing = 0;
    while (did_nothing < 2) {
        let mut did_something = false;

        did_something |= constant_propagation(line);
        did_something |= dead_selection(line);
        did_something |= dead_assignment(line);
        did_something |= dead_conditional(line);

        did_nothing = if (did_something) { 0 } else { did_nothing + 1 };
    }
}
