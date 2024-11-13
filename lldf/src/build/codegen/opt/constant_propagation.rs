use super::*;
use crate::build::codegen::{ CodeValue, CodeblockBlock, VariableScope };
use std::array;


/// Removes constant declarations and distributes the value to later usages.
/// 
/// **This optimisation requires the following guarantees to be upheld:**
/// - Variables may only be assigned ONCE, EXCEPT in `switch`-style statements.
/// Failure to uphold the guarantees may result in broken codegen.
pub fn constant_propagation(line : &mut CodeLine) -> bool {
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        let block = &line.blocks[i];
        if let Codeblock::Block(CodeblockBlock { block, action : Some(action), params, tags, .. }) = block {
            if (block == "set_var") {
                // The destination variable is a line variable.
                if let CodeValue::Variable { name : dest_name, scope : VariableScope::Line } = &params[0] {
                    // The destination variable is not a parameter value.
                    if (! line.blocks.iter().any(|block| block.contains_param(dest_name))) {
                        // Check if this value can be propagated.
                        if let Some(value) = check_value_to_propagate(action, &mut params.iter().skip(1), &tags) {
                            // Check if all codeblocks in the line allow replacing the variable.
                            // Also check that the value is only set once.
                            let mut can_replace = true;
                            let mut write_count = 0usize;
                            for j in 0..line.blocks.len() {
                                let other_block = &line.blocks[j];
                                if (! other_block.can_replace_line_var_with_constant(dest_name)) { can_replace = false; break; }
                                if let (_, Some(other_dest_var)) = other_block.setvar_like_line() {
                                    if (other_dest_var == dest_name) {
                                        write_count += 1;
                                        if (write_count > 1) { can_replace = false; break; }
                                    }
                                }
                            }
                            if (can_replace) {
                                let dest_name = dest_name.clone();

                                // Remove the current codeblock,
                                line.blocks.remove(i);

                                for block in &mut line.blocks {
                                    block.replace_line_var_with_constant(&dest_name, &value);
                                }

                                // Allow the optimiser to try another pass.
                                did_something = true;

                            }
                        }
                    }
                }
            }
        }
    }

    did_something
}


fn check_value_to_propagate<'l>(action : &str, params : &mut impl Iterator<Item = &'l CodeValue>, tags : &Vec<CodeValue>) -> Option<CodeValue> {


    // Variable Setting

    if (action == "=") { match (get_chunk::<2>(params)) {
        [Some(value), None] => { if (value.is_constant()) { return Some(value.clone()); } },
        _ => { }
    } }


    // Numerical Actions

    else if (action == "+") {
        let mut final_value = 0.0;
        for param in params {
            if let CodeValue::Number(value) = param {
                if let Ok(value) = value.parse::<f64>() { final_value += value; }
                else { return None; }
            } else { return None; }
        }
        return Some(CodeValue::Number(final_value.to_string()));
    }

    else if (action == "-") && let Some(CodeValue::Number(base_value)) = params.next() { if let Ok(mut final_value) = base_value.parse::<f64>() {
        for param in params {
            if let CodeValue::Number(value) = param {
                if let Ok(value) = value.parse::<f64>() { final_value -= value; }
                else { return None; }
            } else { return None; }
        }
        return Some(CodeValue::Number(final_value.to_string()));
    } }

    else if (action == "*") && let Some(CodeValue::Number(base_value)) = params.next() { if let Ok(mut final_value) = base_value.parse::<f64>() {
        for param in params {
            if let CodeValue::Number(value) = param {
                if let Ok(value) = value.parse::<f64>() { final_value *= value; }
                else { return None; }
            } else { return None; }
        }
        return Some(CodeValue::Number(final_value.to_string()));
    } }

    // TODO: /

    // TODO: %

    // TODO: Exponent

    // TODO: Root

    // TODO: Logarithm

    else if (action == "ParseNumber") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::String(value)), None] => { if let Ok(value) = value.parse::<f64>() {
            return Some(CodeValue::Number(value.to_string()));
        } }, _ => { }
    } }

    else if (action == "AbsoluteValue") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Number(value)), None] => { if let Ok(value) = value.parse::<f64>() {
            return Some(CodeValue::Number((-value).to_string()));
        } }, _ => { }
    } }

    else if (action == "ClampNumber") { match (get_chunk::<4>(params)) {
        [Some(CodeValue::Number(value)), Some(CodeValue::Number(min)), Some(CodeValue::Number(max)), None] => { if let Ok(value) = value.parse::<f64>() { if let Ok(min) = min.parse::<f64>() { if let Ok(max) = max.parse::<f64>() {
            return Some(CodeValue::Number(value.clamp(min, max).to_string()));
        } } } }, _ => { }
    } }

    // TODO: WrapNum

    // TODO: BounceNum

    else if (action == "Average") {
        let mut final_value = 0.0;
        let mut count       = 0usize;
        for param in params {
            if let CodeValue::Number(value) = param {
                if let Ok(value) = value.parse::<f64>() { final_value += value; count += 1; }
                else { return None; }
            } else { return None; }
        }
        return Some(CodeValue::Number((final_value / (count as f64)).to_string()));
    }

    // TODO: RoundNumber

    // TODO: MinNumber

    // TODO: MaxNumber

    // TODO: Sine

    // TODO: Cosine

    // TODO: Tangent

    // TODO: Bitwise


    // String Manipulation

    else if (action == "String") { if let Some(spaces) = get_tag(tags, "Text Value Merging") {
        let spaces = if (spaces == "Add spaces") { " " } else { "" };
        let mut final_value = String::new();
        let mut first       = true;
        for param in params {
            if (first) { first = false; }
            else { final_value += spaces; }
            match (param) {
                CodeValue::String(value) | CodeValue::Number(value) => { final_value += value; },
                _ => { return None; }
            }
        }
        return Some(CodeValue::String(final_value));
    } }

    // TODO: ReplaceString

    // TODO: RemoveString

    // TODO: TrimString

    // TODO: SetCase

    // TODO: StringLength

    // TODO: RepeatString


    // Styled Text Manipulation

    else if (action == "StyledText") { if let Some(styles) = get_tag(tags, "Inherit Styles") { if let Some(spaces) = get_tag(tags, "Text Value Merging") {
        let spaces = match ((styles.as_str(), spaces.as_str())) {
            ("False", "Add spaces") => "<reset> ",
            ("False", _) => "",
            (_, "Add spaces") => " ",
            _ => ""
        };
        let mut final_value = String::new();
        let mut first       = true;
        for param in params {
            if (first) { first = false; }
            else { final_value += spaces; }
            match (param) {
                CodeValue::String(value) | CodeValue::Text(value) | CodeValue::Number(value) => { final_value += value; },
                _ => { return None; }
            }
        }
        return Some(CodeValue::Text(final_value));
    } } }

    // TODO: ClearFormatting

    // TODO: GetMiniMessageExpr

    // TODO: ParseMiniMessageExpr

    // TODO: TrimStyledText

    // TODO: ContentLength


    // Location Manipulation

    // TODO: GetCoord

    // TODO: SetCoord

    else if (action == "SetAllCoords") { match (get_chunk::<6>(params)) {
        [Some(CodeValue::Number(x)), Some(CodeValue::Number(y)), Some(CodeValue::Number(z)), Some(CodeValue::Number(pitch)), Some(CodeValue::Number(yaw)), None] => { if let Ok(x) = x.parse::<f64>() { if let Ok(y) = y.parse::<f64>() { if let Ok(z) = z.parse::<f64>() { if let Ok(pitch) = pitch.parse::<f64>() { if let Ok(yaw) = yaw.parse::<f64>() {
            return Some(CodeValue::Location { x, y, z, pitch, yaw });
        } } } } } }, _ => { }
    } }

    // TODO: ShiftOnAxis

    else if (action == "ShiftAllAxes") { match (get_chunk::<5>(params)) {
        [Some(CodeValue::Location { x, y, z, pitch, yaw }), Some(CodeValue::Number(dx)), Some(CodeValue::Number(dy)), Some(CodeValue::Number(dz)), None] => { if let Ok(dx) = dx.parse::<f64>() { if let Ok(dy) = dy.parse::<f64>() { if let Ok(dz) = dz.parse::<f64>() {
            return Some(CodeValue::Location { x : x + dx, y : y + dy, z : z + dz, pitch : *pitch, yaw : *yaw });
        } } } }, _ => { }
    } }

    // TODO: ShiftInDirection

    // TODO: ShiftAllDirections

    // TODO: ShiftToward

    // TODO: ShiftOnVector

    // TODO: GetDirection

    // TODO: SetDirection

    // TODO: ShiftRotation

    // TODO: FaceLocation

    // TODO: AlignLoc

    // TODO: Distance

    // TODO: GetCenterLoc


    // Item Manipulation

    // TODO: GetItemType

    // TODO: SetItemType

    // TODO: Other item operations


    // Particle Manipulation

    else if (action == "GetParticleType") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Particle { kind, .. }), None] => { return Some(CodeValue::String(kind.clone())); },
        _ => { }
    } }

    else if (action == "SetParticleType") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Particle { kind : _, spread_x, spread_y, amount, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour }), Some(CodeValue::String(new_kind)), None] => {
            return Some(CodeValue::Particle { kind : new_kind.clone(), spread_x : *spread_x, spread_y : *spread_y, amount : *amount, motion : *motion, motion_variation : *motion_variation, colour : colour.clone(), colour_variation : *colour_variation, opacity : *opacity, material : material.clone(), size : *size, size_variation : *size_variation, roll : *roll, fade_colour : fade_colour.clone() });
        }, _ => { }
    } }

    else if (action == "GetParticleAmount") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Particle { amount, .. }), None] => { return Some(CodeValue::Number(amount.to_string())); },
        _ => { }
    } }

    else if (action == "SetParticleAmount") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Particle { kind, spread_x, spread_y, amount : _, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour }), Some(CodeValue::Number(new_amount)), None] => { if let Ok(new_amount) = new_amount.parse::<f64>() {
            return Some(CodeValue::Particle { kind : kind.clone(), spread_x : *spread_x, spread_y : *spread_y, amount : new_amount.floor() as u64, motion : *motion, motion_variation : *motion_variation, colour : colour.clone(), colour_variation : *colour_variation, opacity : *opacity, material : material.clone(), size : *size, size_variation : *size_variation, roll : *roll, fade_colour : fade_colour.clone() });
        } }, _ => { }
    } }

    // TODO: GetParticleSprd

    else if (action == "SetParticleSprd") { match (get_chunk::<4>(params)) {
        [Some(CodeValue::Particle { kind, spread_x : _, spread_y : _, amount, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour }), Some(CodeValue::Number(new_spread_x)), Some(CodeValue::Number(new_spread_y)), None] => { if let Ok(new_spread_x) = new_spread_x.parse::<f64>() { if let Ok(new_spread_y) = new_spread_y.parse::<f64>() {
            return Some(CodeValue::Particle { kind : kind.clone(), spread_x : new_spread_x, spread_y : new_spread_y, amount : *amount, motion : *motion, motion_variation : *motion_variation, colour : colour.clone(), colour_variation : *colour_variation, opacity : *opacity, material : material.clone(), size : *size, size_variation : *size_variation, roll : *roll, fade_colour : fade_colour.clone() });
        } } }, _ => { }
    } }

    else if (action == "GetParticleSize") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Particle { size, .. }), None] => { return Some(CodeValue::Number(size.map_or_else(|| "0".to_string(), |size| size.to_string()))); },
        _ => { }
    } }

    else if (action == "SetParticleSize") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Particle { kind, spread_x, spread_y, amount, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour }), Some(CodeValue::Number(new_size)), None] => { if let Ok(new_size) = new_size.parse::<f64>() {
            return Some(CodeValue::Particle { kind : kind.clone(), spread_x : *spread_x, spread_y : *spread_y, amount : *amount, motion : *motion, motion_variation : *motion_variation, colour : colour.clone(), colour_variation : *colour_variation, opacity : *opacity, material : material.clone(), size : size.map(|_| new_size), size_variation : *size_variation, roll : *roll, fade_colour : fade_colour.clone() });
        } }, _ => { }
    } }

    // TODO: GetParticleMat

    // TODO: SetParticleMat

    // TODO: GetParticleColor

    // TODO: SetParticleColor

    else if (action == "GetParticleOpac") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Particle { opacity, .. }), None] => { return Some(CodeValue::Number(opacity.map_or_else(|| "0".to_string(), |opacity| opacity.to_string()))); }, // TODO: Make sure the default is actually 0
        _ => { }
    } }

    else if (action == "SetParticleOpac") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Particle { kind, spread_x, spread_y, amount, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour }), Some(CodeValue::Number(new_opacity)), None] => { if let Ok(new_opacity) = new_opacity.parse::<f64>() {
            return Some(CodeValue::Particle { kind : kind.clone(), spread_x : *spread_x, spread_y : *spread_y, amount : *amount, motion : *motion, motion_variation : *motion_variation, colour : colour.clone(), colour_variation : *colour_variation, opacity : opacity.map(|_| new_opacity), material : material.clone(), size : *size, size_variation : *size_variation, roll : *roll, fade_colour : fade_colour.clone() });
        } }, _ => { }
    } }

    else if (action == "GetParticleMotion") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Particle { motion, .. }), None] => { return Some(motion.map_or_else(|| CodeValue::Vector { x : 0.0, y : 0.0, z : 0.0 }, |(x, y, z)| CodeValue::Vector { x, y, z })) },
        _ => { }
    } }

    else if (action == "SetParticleMotion") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Particle { kind, spread_x, spread_y, amount, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour }), Some(CodeValue::Vector { x, y, z }), None] => {
            return Some(CodeValue::Particle { kind : kind.clone(), spread_x : *spread_x, spread_y : *spread_y, amount : *amount, motion : motion.map(|_| (*x, *y, *z)), motion_variation : *motion_variation, colour : colour.clone(), colour_variation : *colour_variation, opacity : *opacity, material : material.clone(), size : *size, size_variation : *size_variation, roll : *roll, fade_colour : fade_colour.clone() });
        }, _ => { }
    } }

    else if (action == "GetParticleRoll") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Particle { roll, .. }), None] => { return Some(CodeValue::Number(roll.map_or_else(|| "0".to_string(), |opacity| opacity.to_string()))); },
        _ => { }
    } }

    else if (action == "SetParticleRoll") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Particle { kind, spread_x, spread_y, amount, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour }), Some(CodeValue::Number(new_roll)), None] => { if let Ok(new_roll) = new_roll.parse::<f64>() {
            return Some(CodeValue::Particle { kind : kind.clone(), spread_x : *spread_x, spread_y : *spread_y, amount : *amount, motion : *motion, motion_variation : *motion_variation, colour : colour.clone(), colour_variation : *colour_variation, opacity : *opacity, material : material.clone(), size : *size, size_variation : *size_variation, roll : roll.map(|_| new_roll), fade_colour : fade_colour.clone() });
        } }, _ => { }
    } }

    // TODO: GetParticleFade

    // TODO: SetParticleFade


    // Vector Manipulation

    else if (action == "Vector") { match (get_chunk::<4>(params)) {
        [Some(CodeValue::Number(x)), Some(CodeValue::Number(y)), Some(CodeValue::Number(z)), None] => {
            if let Ok(x) = x.parse::<f64>() && let Ok(y) = y.parse::<f64>() && let Ok(z) = z.parse::<f64>() {
                return Some(CodeValue::Vector { x, y, z });
            }
        }, _ => { }
    } }

    else if (action == "VectorBetween") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Location { x : ax, y : ay, z : az, .. }), Some(CodeValue::Location { x : bx, y : by, z : bz, .. }), None] => {
            return Some(CodeValue::Vector { x : bx - ax, y : by - ay, z : bz - az });
        }, _ => { }
    } }

    // TODO: GetVectorComp

    // TODO: SetVectorComp

    // TODO: GetVectorLength

    // TODO: SetVectorLength

    else if (action == "MultiplyVector") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Vector { x, y, z }), Some(CodeValue::Number(factor)), None] => {
            if let Ok(factor) = factor.parse::<f64>() {
                return Some(CodeValue::Vector { x : x * factor, y : y * factor, z : z * factor });
            }
        }, _ => { }
    } }

    // TODO: AddVectors

    // TODO: SubtractVectors

    // TODO: AlignVector

    // TODO: RotateAroundAxis

    // TODO: RotateAroundVec

    // TODO: ReflectVector

    // TODO: CrossProduct

    else if (action == "DotProduct") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Vector { x : ax, y : ay, z : az }), Some(CodeValue::Vector { x : bx, y : by, z : bz }), None] => {
            return Some(CodeValue::Number((ax*bx + ay*by + az*bz).to_string()));
        }, _ => { }
    } }


    // Miscellaneous Actions

    else if (action == "GetPotionType") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Potion { kind, .. }), None] => { return Some(CodeValue::String(kind.clone())); },
        _ => { }
    } }

    else if (action == "SetPotionType") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Potion { kind : _, dur, amp }), Some(CodeValue::String(new_kind)), None] => {
            return Some(CodeValue::Potion { kind : new_kind.clone(), dur : *dur, amp : *amp });
        }, _ => { }
    } }

    else if (action == "GetPotionAmp") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Potion { amp, .. }), None] => { return Some(CodeValue::Number((amp + 1).to_string())); },
        _ => { }
    } }

    else if (action == "SetPotionAmp") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Potion { kind, dur, amp : _ }), Some(CodeValue::Number(new_amp)), None] => { if let Ok(new_amp) = new_amp.parse::<f64>() {
            return Some(CodeValue::Potion { kind : kind.clone(), dur : *dur, amp : new_amp.floor() as u8 - 1 });
        } }, _ => { }
    } }

    else if (action == "GetPotionDur") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Potion { dur, .. }), None] => { return Some(CodeValue::Number(dur.to_string())); },
        _ => { }
    } }

    else if (action == "SetPotionDur") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Potion { kind, dur : _, amp }), Some(CodeValue::Number(new_dur)), None] => { if let Ok(new_dur) = new_dur.parse::<f64>() {
            return Some(CodeValue::Potion { kind : kind.clone(), dur : new_dur.floor() as u32, amp : *amp });
        } }, _ => { }
    } }

    else if (action == "GetSoundType") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Sound { kind : SoundKind::Named(kind), .. }), None] => { return Some(CodeValue::String(kind.clone())); },
        _ => { }
    } }

    else if (action == "SetSoundType") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Sound { kind : _, volume, pitch }), Some(CodeValue::String(new_kind)), None] => {
            return Some(CodeValue::Sound { kind : SoundKind::Named(new_kind.clone()), volume : *volume, pitch : *pitch });
        }, _ => { }
    } }

    // TODO: GetSoundVariant

    // TODO: SetSoundVariant

    else if (action == "GetCustomSound") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Sound { kind : SoundKind::Keyed(key), .. }), None] => { return Some(CodeValue::String(key.clone())); },
        _ => { }
    } }

    else if (action == "SetCustomSound") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Sound { kind : _, volume, pitch }), Some(CodeValue::String(new_key)), None] => {
            return Some(CodeValue::Sound { kind : SoundKind::Keyed(new_key.clone()), volume : *volume, pitch : *pitch });
        }, _ => { }
    } }

    else if (action == "GetSoundPitch") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Sound { pitch, .. }), None] => { return Some(CodeValue::Number(pitch.to_string())); },
        _ => { }
    } }

    else if (action == "SetSoundPitch") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Sound { kind, volume, pitch : _ }), Some(CodeValue::Number(new_pitch)), None] => { if let Ok(new_pitch) = new_pitch.parse::<f64>() {
            return Some(CodeValue::Sound { kind : kind.clone(), volume : *volume, pitch : new_pitch });
        } }, _ => { }
    } }

    else if (action == "GetSoundVolume") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Sound { volume, .. }), None] => { return Some(CodeValue::Number(volume.to_string())); },
        _ => { }
    } }

    else if (action == "SetSoundVolume") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Sound { kind, volume : _, pitch }), Some(CodeValue::Number(new_volume)), None] => { if let Ok(new_volume) = new_volume.parse::<f64>() {
            return Some(CodeValue::Sound { kind : kind.clone(), volume : new_volume, pitch : *pitch });
        } }, _ => { }
    } }


    None
}



fn get_chunk<'l, const N : usize>(iterator : &mut impl Iterator<Item = &'l CodeValue>) -> [Option<&'l CodeValue>; N] {
    array::from_fn(|_| iterator.next())
}



fn get_tag<'l>(tags : &'l Vec<CodeValue>, key : &str) -> Option<&'l String> {
    for tag in tags {
        if let CodeValue::Actiontag { kind, value, variable : None } = tag {
            if (kind == key) {
                return Some(value)
            }
        }
    }
    None
}
