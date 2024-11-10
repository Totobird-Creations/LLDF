use super::*;
use crate::build::codegen::{ CodeValue, CodeblockBlock, SoundKind };


/// Removes redundant sound operation codeblocks, replacing them with sound constants.
pub fn constant_sound(line : &mut CodeLine) -> bool {
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        let block = &mut line.blocks[i];
        if let Codeblock::Block(CodeblockBlock { block : codeblock, action : Some(action), params, .. }) = block { if (codeblock == "set_var") {

            if (action == "SetCustomSound" && params.len() == 3) {
                if let CodeValue::String(key) = &params[2] {
                    if let CodeValue::Sound { volume, pitch, .. } = &params[1] {
                        let params = vec![
                            params[0].clone(),
                            CodeValue::Sound { kind : SoundKind::Keyed(key.clone()), volume : *volume, pitch : *pitch }
                        ];
                        *block = Codeblock::action("set_var", "=", params, vec![ ]);
                        did_something = true;
                    }
                }
            }

            else if (action == "SetSoundVolume" && params.len() == 3) {
                if let CodeValue::Number(volume) = &params[2] { if let Ok(volume) = volume.parse() {
                    if let CodeValue::Sound { kind, pitch, .. } = &params[1] {
                        let params = vec![
                            params[0].clone(),
                            CodeValue::Sound { kind : kind.clone(), volume, pitch : *pitch }
                        ];
                        *block = Codeblock::action("set_var", "=", params, vec![ ]);
                        did_something = true;
                    }
                } }
            }

            else if (action == "SetSoundPitch" && params.len() == 3) {
                if let CodeValue::Number(pitch) = &params[2] { if let Ok(pitch) = pitch.parse() {
                    if let CodeValue::Sound { kind, volume, .. } = &params[1] {
                        let params = vec![
                            params[0].clone(),
                            CodeValue::Sound { kind : kind.clone(), volume : *volume, pitch }
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
