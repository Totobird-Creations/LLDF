use crate::build::codegen::{ CodeValue, VariableScope };

use super::*;


/// Removes any `SetVariable =` codeblocks which reassign variables that will never be used again.
pub fn dead_equals(line : &mut CodeLine) -> () {
    let mut i = 0;
    while (i < line.blocks.len()) {
        let curr = &line.blocks[i];
        if let Codeblock::Block(curr) = curr { if (curr.block == "set_var" && curr.action.as_ref().is_some_and(|a| a == "=")) { 
            if let Some(CodeValue::Variable { name : dest, scope : VariableScope::Line }) = curr.params.get(0) {
                if let Some(CodeValue::Variable { name : src, scope : VariableScope::Line }) = curr.params.get(1) {
                    if let None = curr.params.get(2) {
                        let mut success = true;
                        // Scan code blocks. Make sure `src` is never used again.
                        for j in (i + 1)..line.blocks.len() {
                            if (line.blocks[j].contains_line_var(src)) {
                                success = false;
                                break;
                            }
                        }
                        if (success) {
                            let dest = dest.clone();
                            let src  = src.clone();
                            // Scan code blocks, and replace every instance of  `dest` with `src`.
                            for j in (i + 1)..line.blocks.len() {
                                line.blocks[j].replace_line_var(&dest, &src);
                            }
                            // Destroy this code block.
                            line.blocks.remove(i);
                            continue;
                        }
                    }
                }
            }
        } }
        i += 1;
    }
}
