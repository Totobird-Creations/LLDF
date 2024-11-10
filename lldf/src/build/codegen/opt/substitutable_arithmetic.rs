use super::*;
use crate::build::codegen::{ CodeValue, CodeblockBlock, VariableScope };


const SUBSTITUTION_MAX_COUNT : usize = 4;


/// Removes redundant `+`, `-`, `x`, `/` codeblocks, replacing them with `%math` codes.
/// 
/// **This optimisation requires some guarantees that LLVM provides.**
/// Failure to uphold the guarantees may result in broken codegen.
pub fn substitutable_arithmetic(line : &mut CodeLine) -> bool {
    let mut did_something = false;

    'l : for i in (0..line.blocks.len()).rev() {
        let block = &line.blocks[i];
        if let Codeblock::Block(CodeblockBlock { block, action : Some(action), params, .. }) = block {
            if (block == "set_var" && params.len() == 3) {
                // The destination variable is a line variable.
                if let CodeValue::Variable { name : dest_name, scope : VariableScope::Line } = &params[0] {
                    // The destination variable is not a parameter value.
                    if (! line.blocks.iter().any(|block| block.contains_param(dest_name))) {

                        if let Some(left) = try_param(&params[1]) { let left_int = left.parse::<f64>().ok();
                            if let Some(right) = try_param(&params[2]) { let right_int = right.parse::<f64>().ok();
                                if let Some(value) = (match (action.as_str()) {
                                    "+" => Some(CodeValue::Number(if let (Some(left), Some(right)) = (left_int, right_int) { (left + right).to_string() } else { format!("%math({}+{})", left, right) })),
                                    "-" => Some(CodeValue::Number(if let (Some(left), Some(right)) = (left_int, right_int) { (left - right).to_string() } else { format!("%math({}-{})", left, right) })),
                                    "x" => Some(CodeValue::Number(if let (Some(left), Some(right)) = (left_int, right_int) { (left * right).to_string() } else { format!("%math({}*{})", left, right) })),
                                    "/" => Some(CodeValue::Number(if let (Some(left), Some(right)) = (left_int, right_int) { (left / right).to_string() } else { format!("%math({}/{})", left, right) })),
                                    _   => None
                                }) {
                                    // Count the number of usages of the destination variable, also making sure that each usage can be substituted.
                                    let mut count = 0;
                                    for j in (i + 1)..line.blocks.len() {
                                        let checking_block = &line.blocks[j];
                                        if (! checking_block.can_replace_line_var_with_constant(dest_name)) { break 'l; }
                                        if (checking_block.is_line_var_used(&dest_name)) { count += 1; }
                                    }
                                    if (count <= SUBSTITUTION_MAX_COUNT) {

                                        let dest_name = dest_name.clone();
                                        for j in (i + 1)..line.blocks.len() {
                                            line.blocks[j].replace_line_var_with_constant(&dest_name, &value);
                                        }
                                        line.blocks.remove(i);
                                        did_something = true;

                                    }
                                }
                            }
                        }

                    }
                }
            }
        }
    }

    did_something
}


fn try_param(param : &CodeValue) -> Option<String> { match (param) {
    CodeValue::Number   ( value                             ) => Some(value.clone()),
    CodeValue::Variable { name, scope : VariableScope::Line } => Some(format!("%var({})", name)),
    _ => None
} }
