use super::*;


/// Removes any `SelectObject` codeblocks which is immediately followed by a non-filter `SelectObject` codeblock.
pub fn dead_selections(line : &mut CodeLine) -> () {
    for i in (1..line.blocks.len()).rev() {
        let prev = &line.blocks[i - 1];
        let curr = &line.blocks[i];
        if let Codeblock::Block(prev) = prev { if (prev.block == "select_obj") {
            if let Codeblock::Block(curr) = curr { if (curr.block == "select_obj" && (! curr.action.as_ref().unwrap().contains("Filter"))) {
                line.blocks.remove(i - 1);
            } }
        } }
    }
}
