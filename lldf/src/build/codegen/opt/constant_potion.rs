use super::*;
use crate::build::codegen::{ CodeValue, CodeblockBlock };


/// Removes redundant sound operation codeblocks, replacing them with sound constants.
pub fn constant_potion(line : &mut CodeLine) -> bool {
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        let block = &mut line.blocks[i];
        if let Codeblock::Block(CodeblockBlock { block : codeblock, action : Some(action), params, .. }) = block { if (codeblock == "set_var") {

            if (action == "SetPotionDur" && params.len() == 3) {
                if let CodeValue::Number(dur) = &params[2] { if let Ok(dur) = dur.parse() {
                    if let CodeValue::Potion { kind, amp, .. } = &params[1] {
                        let params = vec![
                            params[0].clone(),
                            CodeValue::Potion { kind : kind.clone(), amp : *amp, dur }
                        ];
                        *block = Codeblock::action("set_var", "=", params, vec![ ]);
                        did_something = true;
                    }
                } }
            }

            else if (action == "SetPotionAmp" && params.len() == 3) {
                if let CodeValue::Number(amp) = &params[2] { if let Ok(amp) = amp.parse::<u8>() {
                    if let CodeValue::Potion { kind, dur, .. } = &params[1] {
                        let params = vec![
                            params[0].clone(),
                            CodeValue::Potion { kind : kind.clone(), amp : amp - 1, dur : *dur }
                        ];
                        *block = Codeblock::action("set_var", "=", params, vec![ ]);
                        did_something = true;
                    }
                } }
            }

        } }
    }

    did_something
}
