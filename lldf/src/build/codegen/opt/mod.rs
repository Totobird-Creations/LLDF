mod dead_selections;
pub use dead_selections::dead_selections;
mod duplicate_selections;
pub use duplicate_selections::duplicate_selections;


use super::{ Codeblock, CodeLine };


pub fn optimise(line : &mut CodeLine) -> () {
    dead_selections(line);
    duplicate_selections(line);
    // TODO: Remove set local to constant.
    // TODO: Reassign last usage of local to another local
    dead_selections(line);
}
