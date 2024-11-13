use super::*;


/// Removes any `PlayerName` or `EntityName` `Select Object` codeblocks which has a `Selection Target UUIDs` game value as the only parameter.
pub fn redundant_selection(line : &mut CodeLine) -> bool {
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        let block = &line.blocks[i];
        if let Codeblock::Block(block) = block { if (block.block == "select_obj") { if let Some(action) = &block.action { if (action == "PlayerName" || action == "EntityName") {
            if (block.params.len() == 1) { if let CodeValue::Gamevalue { kind, .. } = &block.params[0] { if (kind == "Selection Target UUIDs") {
                line.blocks.remove(i);
                did_something = true;
            } } }
        } } } }
    }

    did_something
}
