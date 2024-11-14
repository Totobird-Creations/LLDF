use super::*;
use crate::build::codegen::{ CodeValue, CodeblockBlock, VariableScope };
use std::array;


/// Removes constant declarations and distributes the value to later usages.
/// 
/// **This optimisation requires the following guarantees to be upheld:**
/// - Variables may only be assigned ONCE, EXCEPT in `switch`-style statements.
/// Failure to uphold the guarantees may result in broken codegen.
pub fn constant_propagation(line : &mut CodeLine, other_functions : &Vec<&mut ParsedFunction>) -> bool {
    let mut did_something = false;

    for i in (0..line.blocks.len()).rev() {
        let block = &line.blocks[i];
        if let Codeblock::Block(CodeblockBlock { block, action : Some(action), params, tags, .. }) = block {
            if (block == "set_var") {
                // Check that the destination variable is a line variable.
                if let CodeValue::Variable { name : dest_name, scope : VariableScope::Line | VariableScope::Local } = &params[0] {
                    if (
                        // Check that the destination variable is not tied to a parameter.
                        (! line.blocks.iter().any(|block| block.contains_param(dest_name)))
                        // Check that the destination variable is not used in another block.
                        && other_functions.iter().all(|other_function| ! other_function.line.blocks.iter().any(|block| block.is_var_used(dest_name)))
                    ) {
                        // Check that this value can be propagated.
                        if let Some(value) = check_value_to_propagate(line, i, dest_name, action, &mut params.iter().skip(1), &tags) {
                            // Check if all codeblocks in the line allow replacing the variable.
                            // Also check that the value is only set once.
                            let mut can_replace = true;
                            let mut write_count = 0usize;
                            for j in 0..line.blocks.len() {
                                let other_block = &line.blocks[j];
                                if (! other_block.can_replace_var_with_constant(dest_name)) { can_replace = false; break; }
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
                                    block.replace_var_with_constant(&dest_name, &value);
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


fn check_value_to_propagate<'l>(
    line : &CodeLine,
    i : usize,
    dest_name : &str,
    action : &str,
    params : &mut impl Iterator<Item = &'l CodeValue>,
    tags : &Vec<CodeValue>
) -> Option<CodeValue> {


    // Variable Setting

    if (action == "=") { match (get_chunk::<2>(params)) {
        [Some(value), None] => {
            if (value.is_constant()) { return Some(value.clone()); }
            else if let CodeValue::Gamevalue { .. } = value { }
            else {
                // Allow propagation if the value is ONLY used in the codeblock IMMEDIATELY following the current codeblock.
                //  Game values are never allowed to be propagated.
                for j in 0..line.blocks.len() {
                    let block = &line.blocks[j];
                    if (j != i + 1 && block.is_var_used(dest_name)) {
                        return None;
                    }
                }
                return Some(value.clone());
            }
        },
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

    else if (action == "EXponent") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Number(base)), exponent, None] => { if let Ok(base) = base.parse::<f64>() {
            let exponent = if let Some(exponent) = exponent {
                if let CodeValue::Number(exponent) = exponent {
                    if let Ok(exponent) = exponent.parse::<f64>() { exponent } else { return None; }
                } else { return None; }
            } else { 2.0 };
            return Some(CodeValue::Number(base.powf(exponent).to_string()));
        } }, _ => { }
    } }

    else if (action == "Root") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Number(radicand)), degree, None] => { if let Ok(radicand) = radicand.parse::<f64>() {
            let degree = if let Some(degree) = degree {
                if let CodeValue::Number(degree) = degree {
                    if let Ok(degree) = degree.parse::<f64>() { degree } else { return None; }
                } else { return None; }
            } else { 2.0 };
            return Some(CodeValue::Number(radicand.powf(1.0 / degree).to_string()));
        } }, _ => { }
    } }

    else if (action == "Logarithm") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Number(antilogarithm)), Some(CodeValue::Number(base)), None] => { if let Ok(antilogarithm) = antilogarithm.parse::<f64>() { if let Ok(base) = base.parse::<f64>() {
            return Some(CodeValue::Number((antilogarithm.log10() / base.log10()).to_string()));
        } } }, _ => { }
    } }

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

    else if (action == "MinNumber") && let Some(CodeValue::Number(base_value)) = params.next() { if let Ok(mut final_value) = base_value.parse::<f64>() {
        for param in params {
            if let CodeValue::Number(value) = param {
                if let Ok(value) = value.parse::<f64>() { final_value = final_value.min(value); }
                else { return None; }
            } else { return None; }
        }
        return Some(CodeValue::Number(final_value.to_string()));
    } }

    else if (action == "MaxNumber") && let Some(CodeValue::Number(base_value)) = params.next() { if let Ok(mut final_value) = base_value.parse::<f64>() {
        for param in params {
            if let CodeValue::Number(value) = param {
                if let Ok(value) = value.parse::<f64>() { final_value = final_value.max(value); }
                else { return None; }
            } else { return None; }
        }
        return Some(CodeValue::Number(final_value.to_string()));
    } }

    else if (action == "Sine") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Number(value)), None] => { if let Ok(mut value) = value.parse::<f64>() {
            value *= if let Some(mode) = get_tag(tags, "Input") && mode == "Radians" { 1.0 } else { 0.0174532925199 };
            if let Some(mode) = get_tag(tags, "Sine Variant") { match (mode.as_str()) {
                "Sine"                   => { return Some(CodeValue::Number(value.sin().to_string())); },
                "Inverse sine (arcsine)" => { return Some(CodeValue::Number(value.asin().to_string())); },
                "Hyperbolic sine"        => { return Some(CodeValue::Number(value.sinh().to_string())); }
                _ => { }
            } }
        } }, _ => { }
    } }

    else if (action == "Cosine") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Number(value)), None] => { if let Ok(mut value) = value.parse::<f64>() {
            value *= if let Some(mode) = get_tag(tags, "Input") && mode == "Radians" { 1.0 } else { 0.0174532925199 };
            if let Some(mode) = get_tag(tags, "Cosine Variant") { match (mode.as_str()) {
                "Cosine"                     => { return Some(CodeValue::Number(value.cos().to_string())); },
                "Inverse cosine (arccosine)" => { return Some(CodeValue::Number(value.acos().to_string())); },
                "Hyperbolic cosine"          => { return Some(CodeValue::Number(value.cosh().to_string())); }
                _ => { }
            } }
        } }, _ => { }
    } }

    else if (action == "Tangent") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Number(value)), None] => { if let Ok(mut value) = value.parse::<f64>() {
            value *= if let Some(mode) = get_tag(tags, "Input") && mode == "Radians" { 1.0 } else { 0.0174532925199 };
            if let Some(mode) = get_tag(tags, "Tangent Variant") { match (mode.as_str()) {
                "Tangent"                      => { return Some(CodeValue::Number(value.tan().to_string())); },
                "Inverse tangent (arctangent)" => { return Some(CodeValue::Number(value.atan().to_string())); },
                "Hyperbolic tangent"           => { return Some(CodeValue::Number(value.tanh().to_string())); }
                _ => { }
            } }
        } }, _ => { }
    } }

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

    else if (action == "StringLength") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::String(value)), None] => {
            return Some(CodeValue::Number(value.len().to_string()));
        }, _ => { }
    } }

    else if (action == "RepeatString") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::String(value)), Some(CodeValue::Number(repeat)), None] => { if let Ok(repeat) = repeat.parse::<f64>() {
            return Some(CodeValue::String(value.repeat(repeat.floor() as usize)));
        } }, _ => { }
    } }


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

    else if (action == "GetCoord") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Location { x, y, z, pitch, yaw }), None] => { if let Some(origin) = get_tag(tags, "Coordinate Type") { if (origin == "Plot coordinate") {
            if let Some(coordinate) = get_tag(tags, "Coordinate") { match (coordinate.as_str()) {
                "X"     => { return Some(CodeValue::Number(x     .to_string())) },
                "Y"     => { return Some(CodeValue::Number(y     .to_string())) },
                "Z"     => { return Some(CodeValue::Number(z     .to_string())) },
                "Pitch" => { return Some(CodeValue::Number(pitch .to_string())) },
                "Yaw"   => { return Some(CodeValue::Number(yaw   .to_string())) },
                _ => { }
            } }
        } } }, _ => { }
    } }

    else if (action == "SetCoord") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Location { x, y, z, pitch, yaw }), Some(CodeValue::Number(new_value)), None] => { if let Ok(new_value) = new_value.parse::<f64>() { if let Some(origin) = get_tag(tags, "Coordinate Type") { if (origin == "Plot coordinate") {
            if let Some(coordinate) = get_tag(tags, "Coordinate") { match (coordinate.as_str()) {
                "X"     => { return Some(CodeValue::Location { x : new_value, y : *y ,        z : *z,        pitch : *pitch,    yaw : *yaw      }) },
                "Y"     => { return Some(CodeValue::Location { x : *x,        y : new_value , z : *z,        pitch : *pitch,    yaw : *yaw      }) },
                "Z"     => { return Some(CodeValue::Location { x : *x,        y : *y ,        z : new_value, pitch : *pitch,    yaw : *yaw      }) },
                "Pitch" => { return Some(CodeValue::Location { x : *x,        y : *y ,        z : *z,        pitch : new_value, yaw : *yaw      }) },
                "Yaw"   => { return Some(CodeValue::Location { x : *x,        y : *y ,        z : *z,        pitch : *pitch,    yaw : new_value }) },
                _ => { }
            } }
        } } } }, _ => { }
    } }

    else if (action == "SetAllCoords") { match (get_chunk::<6>(params)) {
        [Some(CodeValue::Number(x)), Some(CodeValue::Number(y)), Some(CodeValue::Number(z)), Some(CodeValue::Number(pitch)), Some(CodeValue::Number(yaw)), None] => { if let Ok(x) = x.parse::<f64>() { if let Ok(y) = y.parse::<f64>() { if let Ok(z) = z.parse::<f64>() { if let Ok(pitch) = pitch.parse::<f64>() { if let Ok(yaw) = yaw.parse::<f64>() {
            return Some(CodeValue::Location { x, y, z, pitch, yaw });
        } } } } } }, _ => { }
    } }

    else if (action == "ShiftOnAxis") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Location { x, y, z, pitch, yaw }), Some(CodeValue::Number(delta)), None] => { if let Ok(delta) = delta.parse::<f64>() {
            if let Some(coordinate) = get_tag(tags, "Coordinate") { match (coordinate.as_str()) {
                "X" => { return Some(CodeValue::Location { x : x + delta, y : *y, z : *z, pitch : *pitch, yaw : *yaw }); },
                "Y" => { return Some(CodeValue::Location { x : *x, y : y + delta, z : *z, pitch : *pitch, yaw : *yaw }); },
                "Z" => { return Some(CodeValue::Location { x : *x, y : *y, z : z + delta, pitch : *pitch, yaw : *yaw }); },
                _ => { }
            } }
        } }, _ => { }
    } }

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

    else if (action == "GetItemAmount") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Item { count, .. }), None] => { return Some(CodeValue::Number(count.to_string())); },
        _ => { }
    } }

    else if (action == "SetItemAmount") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Item { id, count : _, nbt }), Some(CodeValue::Number(new_count)), None] => { if let Ok(new_count) = new_count.parse::<f64>() {
            return Some(CodeValue::Item { id : id.clone(), count : new_count.floor() as u8, nbt : nbt.clone() });
        } }, _ => { }
    } }


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

    else if (action == "GetParticleSprd") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Particle { spread_x, spread_y, .. }), None] => { if let Some(direction) = get_tag(tags, "Spread") { match (direction.as_str()) {
            "Horizontal" => { return Some(CodeValue::Number(spread_x.to_string())); },
            "Vertical"   => { return Some(CodeValue::Number(spread_y.to_string())); }
            _ => { }
        } } },
        _ => { }
    } }

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

    else if (action == "GetVectorComp") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Vector { x, y, z }), None] => {
            if let Some(component) = get_tag(tags, "Component") { match (component.as_str()) {
                "X" => { return Some(CodeValue::Number(x.to_string())) },
                "Y" => { return Some(CodeValue::Number(y.to_string())) },
                "Z" => { return Some(CodeValue::Number(z.to_string())) },
                _ => { }
            } }
        }, _ => { }
    } }

    else if (action == "SetVectorComp") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Vector { x, y, z }), Some(CodeValue::Number(new_value)), None] => { if let Ok(new_value) = new_value.parse::<f64>() {
            if let Some(component) = get_tag(tags, "Component") { match (component.as_str()) {
                "X" => { return Some(CodeValue::Vector { x : new_value, y : *y,        z : *z        }) },
                "Y" => { return Some(CodeValue::Vector { x : *x,        y : new_value, z : *z        }) },
                "Z" => { return Some(CodeValue::Vector { x : *x,        y : *y,        z : new_value }) },
                _ => { }
            } }
        } }, _ => { }
    } }

    else if (action == "GetVectorLength") { match (get_chunk::<2>(params)) {
        [Some(CodeValue::Vector { x, y, z }), None] => {
            let length_squared = x*x + y*y + z*z;
            if let Some(squared) = get_tag(tags, "Length Type") { if (squared == "Length Squared") {
                return Some(CodeValue::Number(length_squared.to_string()));
            } }
            return Some(CodeValue::Number(length_squared.sqrt().to_string()));
        }, _ => { }
    } }

    else if (action == "SetVectorLength") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Vector { x, y, z }), Some(CodeValue::Number(new_length)), None] => { if let Ok(new_length) = new_length.parse::<f64>() {
            let old_length = (x*x + y*y + z*z).sqrt();
            let factor = new_length / old_length;
            return Some(CodeValue::Vector { x : x * factor, y : y * factor, z : z * factor });
        } }, _ => { }
    } }

    else if (action == "MultiplyVector") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Vector { x, y, z }), Some(CodeValue::Number(factor)), None] => {
            if let Ok(factor) = factor.parse::<f64>() {
                return Some(CodeValue::Vector { x : x * factor, y : y * factor, z : z * factor });
            }
        }, _ => { }
    } }

    else if (action == "AddVectors") {
        let mut final_x = 0.0;
        let mut final_y = 0.0;
        let mut final_z = 0.0;
        for param in params {
            if let CodeValue::Vector { x, y, z } = param {
                final_x += x; final_y += y; final_z += z;
            } else { return None; }
        }
        return Some(CodeValue::Vector { x : final_x, y : final_y, z : final_z });
    }

    else if (action == "SubtractVectors") && let Some(CodeValue::Vector { x : mut final_x, y : mut final_y, z : mut final_z }) = params.next() {
        for param in params {
            if let CodeValue::Vector { x, y, z } = param {
                final_x -= x; final_y -= y; final_z -= z;
            } else { return None; }
        }
        return Some(CodeValue::Vector { x : final_x, y : final_y, z : final_z });
    }

    // TODO: AlignVector

    // TODO: RotateAroundAxis

    // TODO: RotateAroundVec

    else if (action == "ReflectVector") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Vector { x : vx, y : vy, z : vz }), Some(CodeValue::Vector { x : mut nx, y : mut ny, z : mut nz }), None] => {
            let n_length = (nx*nx + ny*ny + nz*nz).sqrt(); if (n_length != 0.0) {
                nx = nx / n_length; ny = ny / n_length; nz = nz / n_length;
                let dot = vx*nx + vy*ny + vz*nz;
                return Some(CodeValue::Vector {
                    x : vx - (2.0 * dot * nx),
                    y : vy - (2.0 * dot * ny),
                    z : vz - (2.0 * dot * nz)
                });
            }
        }, _ => { }
    } }

    else if (action == "CrossProduct") { match (get_chunk::<3>(params)) {
        [Some(CodeValue::Vector { x : ax, y : ay, z : az }), Some(CodeValue::Vector { x : bx, y : by, z : bz }), None] => {
            return Some(CodeValue::Vector { x : ay*bz - by*az, y : az*bx - bz*ax, z : ax*by - bx*ay });
        }, _ => { }
    } }

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
        if let CodeValue::Actiontag { kind, value, variable : None, block_override : _ } = tag {
            if (kind == key) {
                return Some(value)
            }
        }
    }
    None
}
