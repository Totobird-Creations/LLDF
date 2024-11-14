use super::*;


/// Removes any `SET VARIABLE`-like codeblocks, where the destination variable is not used.
pub fn dead_assignment(line : &mut CodeLine, other_functions : &Vec<&mut ParsedFunction>) -> bool {
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        let block = &line.blocks[i];
        if let (true, Some(dest_name)) = block.setvar_like_line() {
            // Check that the destination variable is not used in another block.
            if (other_functions.iter().all(|other_function| ! other_function.line.blocks.iter().any(|block| block.is_line_var_used(dest_name)))) {

                // Check that the variable is never read from.
                let mut can_remove = true;
                for j in 0..line.blocks.len() {
                    let other_block = &line.blocks[j];
                    if (other_block.is_line_var_used(dest_name)) { can_remove = false; break; }
                }
                if (can_remove) {

                    // Remove the current codeblock,
                    line.blocks.remove(i);

                    // Allow the optimiser to try another pass.
                    did_something = true;
                }

            }
        }
    }

    did_something
}
