mod dead_selections;
pub use dead_selections::dead_selections;
mod duplicate_selections;
pub use duplicate_selections::duplicate_selections;
mod unneeded_equals;
pub use unneeded_equals::unneeded_equals;


use super::{ Codeblock, CodeLine };


pub fn optimise(line : &mut CodeLine) -> () {
    dead_selections(line);
    duplicate_selections(line);
    unneeded_equals(line);
    dead_selections(line);
}
