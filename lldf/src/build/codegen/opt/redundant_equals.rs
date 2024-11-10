use super::*;
use crate::build::codegen::{ CodeValue, CodeblockBlock, VariableScope };


/// Removes `SetVariable =` and distributes the value to later usages.
/// 
/// **This optimisation requires some guarantees that LLVM provides.**
/// Failure to uphold the guarantees may result in broken codegen.
pub fn redundant_equals(line : &mut CodeLine) -> bool { // TODO: Add a check to handle crossing a loop boundary.
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        let block = &line.blocks[i];
        if let Codeblock::Block(CodeblockBlock { block, action : Some(action), params, .. }) = block {
            if (block == "set_var" && action == "=" && params.len() == 2) {
                // The destination variable is a line variable.
                if let CodeValue::Variable { name : dest_name, scope : VariableScope::Line } = &params[0] {
                    // The destination variable is not a parameter value.
                    if (! line.blocks.iter().any(|block| block.contains_param(dest_name))) {
                        let value = &params[1];

                        // The value is a line variable.
                        if let CodeValue::Variable { name, scope : VariableScope::Line } = value {
                            let dest_name = dest_name.clone();
                            let name      = name.clone();
                            for j in (i + 1)..line.blocks.len() {
                                line.blocks[j].replace_line_var(&dest_name, &name);
                            }
                            line.blocks.remove(i);
                            did_something = true;
                            continue;
                        }

                        // The value is a constant
                        // AND is not used in `%index` or `%entry` codes.
                        if (
                            value.is_constant()
                            && ((i + 1)..line.blocks.len()).all(|j| line.blocks[j].can_replace_line_var_with_constant(dest_name))
                        ) {
                            let dest_name = dest_name.clone();
                            let value     = value.clone();
                            for j in (i + 1)..line.blocks.len() {
                                line.blocks[j].replace_line_var_with_constant(&dest_name, &value);
                            }
                            line.blocks.remove(i);
                            did_something = true;
                            continue;
                        }

                        // The destination variable is only used in the block immediately following this block.
                        if (line.blocks.get(i + 1).is_none_or(|block| ! block.is_call_func())) {
                            if ((i + 2)..line.blocks.len()).all(|j| ! line.blocks[j].is_line_var_used(dest_name)) {
                                let dest_name = dest_name.clone();
                                let value     = value.clone();
                                if let Some(block) = line.blocks.get_mut(i + 1) {
                                    block.replace_line_var_with_constant(&dest_name, &value);
                                }
                                line.blocks.remove(i);
                                did_something = true;
                                continue;
                            }
                        }

                    }
                }
            }
        }
    }

    did_something
}
