mod dead_selection;
pub use dead_selection::dead_selection;
mod duplicate_selection;
pub use duplicate_selection::duplicate_selection;
mod redundant_selection;
pub use redundant_selection::redundant_selection;
mod redundant_equals;
pub use redundant_equals::redundant_equals;
mod substitutable_arithmetic;
pub use substitutable_arithmetic::substitutable_arithmetic;
mod substitutable_string;
pub use substitutable_string::substitutable_string;
mod substitutable_text;
pub use substitutable_text::substitutable_text;
mod constant_sound;
pub use constant_sound::constant_sound;
mod constant_potion;
pub use constant_potion::constant_potion;


use super::{ Codeblock, CodeLine, CodeValue };


pub fn optimise(line : &mut CodeLine) -> () {
    let mut did_nothing = 0;
    while (did_nothing < 5) {
        let mut did_something = false;
        did_something &= redundant_selection(line);
        did_something &= dead_selection(line);
        did_something &= duplicate_selection(line);
        did_something &= redundant_equals(line);
        did_something &= substitutable_arithmetic(line);
        did_something &= substitutable_string(line);
        did_something &= substitutable_text(line);
        did_something &= constant_sound(line);
        did_something &= constant_potion(line);
        did_nothing = if (did_something) { 0 } else { did_nothing + 1 };
    }
}
