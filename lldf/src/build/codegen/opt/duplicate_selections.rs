use super::*;


/// Removes any `SelectObject` codeblocks which is follow an identical `SelectObject` codeblock.
pub fn duplicate_selections(line : &mut CodeLine) -> () {
    let mut prev_selobj = None;
    let mut i = 0;
    while (i < line.blocks.len()) {
        let curr = &line.blocks[i];
        if let Codeblock::Block(curr) = curr { if (curr.block == "select_obj") {

            if let Some(prev_selobj) = &prev_selobj {
                if (curr == prev_selobj) {
                    line.blocks.remove(i);
                    continue;
                }
            }
            prev_selobj = Some(curr.clone());

        } }
        i += 1;
    }
}
