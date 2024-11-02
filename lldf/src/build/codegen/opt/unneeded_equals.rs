use super::*;
use crate::build::codegen::{ CodeValue, CodeblockBlock, VariableScope };


/// Removes `SetVariable =` and distributes the value to later usages, if ALL of:
/// - The destination variable is a line variable.
/// - The destination variable is not a parameter value.
/// - The value is NOT a game value.
/// AND, any one of:
/// - The value is a constant.
/// - The value is another line variable, and that line variable is never used again after.
/// - The destination variable is used only in the codeblock immediately following this one, UNLESS the immediate following block is a call_func codeblock and the value is not a variable.
pub fn unneeded_equals(line : &mut CodeLine) -> () {
    return;
    for i in (0..line.blocks.len()).rev() {
        let block = &line.blocks[i];
        if let Codeblock::Block(CodeblockBlock { block, action : Some(action), params, .. }) = block {
            if (block == "set_var" && action == "=" && params.len() == 2) {
                // The destination variable is a line variable.
                if let CodeValue::Variable { name : dest_name, scope : VariableScope::Line } = &params[0] {
                    // The destination variable is not a parameter value.
                    if (! line.blocks.iter().any(|block| block.contains_param(dest_name))) {
                        let value = &params[1];
                        // The value is NOT a game value.
                        if let CodeValue::Gamevalue { .. } = value {} else { if (

                            // The value is a constant.
                            value.is_constant()
                            ||
                            // The value is another line variable, and that line variable is never used again after.
                            (if let CodeValue::Variable { name, scope : VariableScope::Line } = value {
                                ! ((i + 1)..line.blocks.len()).any(|j| line.blocks[j].contains_line_var(&name))
                            } else { false })
                            ||
                            // The destination variable is used only in the codeblock immediately following this one.
                            (
                                (! ((i + 2)..line.blocks.len()).any(|j| line.blocks[j].contains_line_var(dest_name)))
                                // UNLESS the immediate following block is a call_func codeblock and the value is not a variable
                                && (if let Some(block) = line.blocks.get(i + 1) { (! block.is_call_func()) || (! value.is_variable()) } else { false })
                            )

                        ) {
                            let name  = dest_name.clone();
                            let value = value.clone();
                            for j in (i + 1)..line.blocks.len() {
                                line.blocks[j].replace_line_var(&name, &value);
                            }
                            line.blocks.remove(i);
                        }
                    }
                } }
            }
        }
    }
}
