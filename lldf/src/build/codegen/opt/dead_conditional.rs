use super::*;


/// Removes any empty conditional or else blocks.
pub fn dead_conditional(line : &mut CodeLine) -> bool {
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        if let (
            Some(Codeblock::Block(CodeblockBlock { block, ..  })),
            Some(Codeblock::Bracket { kind : BracketKind::Normal, side : BracketSide::Open }),
            Some(Codeblock::Bracket { kind : BracketKind::Normal, side : BracketSide::Close }),
        ) = (line.blocks.get(i), line.blocks.get(i + 1), line.blocks.get(i + 2)) {

            if (block == "else") {
                line.blocks.remove(i);
                line.blocks.remove(i);
                line.blocks.remove(i);
                did_something = true;
            }

            else if (block == "if_var" || block == "if_player" || block == "if_entity") {
                // If there is an else branch, invert this conditional.
                if let (
                    Some(Codeblock::Block(CodeblockBlock { block, ..  })),
                    Some(Codeblock::Bracket { kind : BracketKind::Normal, side : BracketSide::Open }),
                ) = (line.blocks.get(i + 3), line.blocks.get(i + 4)) { if (block == "else") {
                    if let Codeblock::Block(CodeblockBlock { attr, ..  }) = &mut line.blocks[i] {
                        if let Some(attr_value) = attr && attr_value == "NOT" {
                            *attr = None;
                        } else {
                            *attr = Some(String::from("NOT"));
                        }
                    }
                    line.blocks.remove(i + 1);
                    line.blocks.remove(i + 1);
                    line.blocks.remove(i + 1);
                    did_something = true;
                    continue;
                } }
                // Otherwise, remove it.
                line.blocks.remove(i);
                line.blocks.remove(i);
                line.blocks.remove(i);
                did_something = true;
            }

        }
    }

    did_something
}
