use super::*;


/// Removes any `SelectObject` codeblocks following an identical `SelectObject` codeblock.
pub fn duplicate_selection(line : &mut CodeLine) -> bool {
    let mut did_something = false;

    let mut prev_selobj = None;
    let mut i = 0;

    while (i < line.blocks.len()) {
        let block = &line.blocks[i];
        if let Codeblock::Bracket { side : BracketSide::Close, .. } = block { prev_selobj = None; }
        else if let Codeblock::Block(CodeblockBlock { block : codeblock, ..  }) = block { if (codeblock == "select_obj") {
            if let Some(prev_selobj) = &prev_selobj {
                if (block == prev_selobj) {
                    line.blocks.remove(i);
                    did_something = true;
                    continue;
                }
            }
            prev_selobj = Some(block.clone());
        } }
        i += 1;
    }

    did_something
}
