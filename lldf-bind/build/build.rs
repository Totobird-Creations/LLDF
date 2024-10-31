#![feature(decl_macro)]


mod dbc;
mod util;

use std::env::current_dir;
use std::fs::File;
use std::io::Write;
use std::error::Error;

use dirs::download_dir;
use chrono::Utc;
use inflector::Inflector;


fn main() -> Result<(), Box<dyn Error>> {
    let cwd = current_dir()?;

    // TODO: find a public actiondump api.
    //let data = reqwest::blocking::get("https://dfonline.dev/public/db.json")?.text()?; // OUTDATED
    let data = std::fs::read_to_string(download_dir().unwrap().join("actiondump.json"))?;
    let mut out : dbc::DBC = serde_json::from_str(&data)?;

    let autogen_message = format!("*\\[{}\\] Automatically generated from the action dump.*", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));


    // Sounds
    /*out.sounds.sort_by(|a, b| a.sound.cmp(&b.sound));
    out.sounds.dedup_by(|a, b| a.sound == b.sound);
    {
        let mut file = File::create(cwd.join("src/bind/sound.rs"))?;
        writeln!(file, "//! {}", autogen_message)?;
        writeln!(file)?;
        writeln!(file, "#[allow(unused_imports)]")?;
        writeln!(file, "use crate::prelude::*;")?;
        writeln!(file)?;
        writeln!(file, "impl Sound {{")?;
        writeln!(file)?;
        for sound in &mut out.sounds {

            util::write_doc_comment(&mut file, "    ",
                &sound.sound.to_title_case(),
                &mut sound.icon.deprecated_note,
                None,
                &sound.icon.description.iter().filter_map(|line| if (*line != "§8Middle-click to view") { Some(line.clone()) } else { None }).collect(),
                None,
                &sound.icon.example,
                &sound.icon.works_with,
                &sound.icon.additional_info,
                &sound.icon.required_rank,
                &autogen_message
            )?;

            writeln!(file,
                "    #[inline(always)] pub fn {}(volume : f64, pitch : f64) -> Self {{ unsafe {{ #[allow(deprecated)] DF_SOUND_{}(volume, pitch) }} }}",
                sound.sound.to_snake_case(),
                sound.sound.to_class_case()
            )?;

            writeln!(file)?;
        }
        writeln!(file, "}}")?;
        writeln!(file)?;
        writeln!(file, "extern \"C\" {{")?;
        for sound in &mut out.sounds {

            util::write_doc_comment(&mut file, "    ",
                &sound.sound.to_title_case(),
                &mut sound.icon.deprecated_note,
                None,
                &sound.icon.description.iter().filter_map(|line| if (*line != "§8Middle-click to view") { Some(line.clone()) } else { None }).collect(),
                None,
                &sound.icon.example,
                &sound.icon.works_with,
                &sound.icon.additional_info,
                &sound.icon.required_rank,
                &autogen_message
            )?;

            writeln!(file,
                "    pub fn DF_SOUND_{}(volume : f64, pitch : f64) -> Sound;",
                sound.sound.to_class_case()
            )?;

        }
        writeln!(file, "}}")?;
    }*/


    // Particles
    /*out.particles.sort_by(|a, b| a.particle.cmp(&b.particle));
    out.particles.dedup_by(|a, b| a.particle == b.particle);
    {
        let mut file = File::create(cwd.join("src/bind/particle.rs"))?;
        writeln!(file, "//! {}", autogen_message)?;
        writeln!(file)?;
        writeln!(file, "#[allow(unused_imports)]")?;
        writeln!(file, "use crate::prelude::*;")?;
        writeln!(file)?;
        writeln!(file, "impl Particle {{")?;
        writeln!(file)?;
        for particle in &mut out.particles {

            util::write_doc_comment(&mut file, "    ",
                &particle.particle.to_title_case(),
                &mut particle.icon.deprecated_note,
                None,
                &particle.icon.description,
                None,
                &particle.icon.example,
                &particle.icon.works_with,
                &particle.icon.additional_info,
                &particle.icon.required_rank,
                &autogen_message
            )?;

            write!(file,
                "    #[inline(always)] pub fn {}( cluster_x : Float, cluster_y : Float, cluster_count : UInt, ",
                particle.particle.to_snake_case()
            )?;

            for field in &particle.fields { write!(file,
                "{} : {}, ",
                format!("{:?}", field).to_snake_case(),
                field.to_type_name()
            )?; }

            write!(file,
                ") -> Self {{ unsafe {{ #[allow(deprecated)] DF_PARTICLE_{}( cluster_x, cluster_y, cluster_count, ",
                particle.particle.to_class_case()
            )?;
            for field in &particle.fields { write!(file,
                "{}, ",
                format!("{:?}", field).to_snake_case()
            )?; }
            writeln!(file, ") }} }}")?;

            writeln!(file)?;
        }
        writeln!(file, "}}")?;
        writeln!(file)?;
        writeln!(file, "extern \"C\" {{")?;
        for particle in &mut out.particles {

            util::write_doc_comment(&mut file, "    ",
                &particle.particle.to_title_case(),
                &mut particle.icon.deprecated_note,
                None,
                &particle.icon.description,
                None,
                &particle.icon.example,
                &particle.icon.works_with,
                &particle.icon.additional_info,
                &particle.icon.required_rank,
                &autogen_message
            )?;

            write!(file,
                "    pub fn DF_PARTICLE_{}( cluster_x : Float, cluster_y : Float, cluster_count : UInt, ",
                particle.particle.to_class_case()
            )?;

            for field in &particle.fields { write!(file,
                "{} : {}, ",
                format!("{:?}", field).to_snake_case(),
                field.to_type_name()
            )?; }

            writeln!(file, ") -> Particle;")?;

        }
        writeln!(file, "}}")?;
    }*/


    // Actions
    for action in &mut out.actions {
        action.name = action.name.trim().to_string();
    }
    out.actions.sort_by(|a, b| a.name.cmp(&b.name));
    out.actions.dedup_by(|a, b| (a.code_block_name == b.code_block_name) && (a.name == b.name));
    {
        let mut file = File::create(cwd.join("src/bind/_action.rs"))?;
        writeln!(file, "//! {}", autogen_message)?;
        writeln!(file)?;
        writeln!(file, "#[allow(unused_imports)]")?;
        writeln!(file, "use crate::prelude::*;")?;
        writeln!(file)?;
        writeln!(file, "#[allow(private_interfaces)]")?;
        writeln!(file, "extern \"C\" {{")?;
        writeln!(file)?;
        for action in &mut out.actions {
            if (action.code_block_name == "PLAYER EVENT" || action.code_block_name == "ENTITY EVENT" || action.name == "dynamic") {
                continue;
            }

            // Skip the actions that have tags.
            if (! action.tags.is_empty()) { continue; }

            util::write_doc_comment(&mut file, "    ",
                &format!("{}: {}",
                    action.code_block_name.to_class_case(),
                    util::remove_sections(&action.name)
                ),
                &mut action.icon.deprecated_note,
                None,
                &action.icon.description,
                Some(&action.tags),
                &action.icon.example,
                &action.icon.works_with,
                &action.icon.additional_info,
                &action.icon.required_rank,
                &autogen_message
            )?;

            write!(file,
                "    pub fn DF_ACTION__{}_{}",
                action.code_block_name.to_class_case(),
                util::symbols_to_names(&action.name).replace(" ", "")
            )?;
            write!(file, "( ")?;
            writeln!(file, "... ) -> ();")?;

            writeln!(file)?;

        }
        writeln!(file, "}}")?;
    }


    // Game Values
    for game_value in &mut out.game_values {
        game_value.icon.name = util::symbols_to_names(&util::remove_sections(game_value.icon.name.trim()));
    }
    out.game_values.sort_by(|a, b| a.icon.name.cmp(&b.icon.name));
    out.actions.dedup_by(|a, b| a.icon.name == b.icon.name);
    {
        let mut file = File::create(cwd.join("src/bind/gamevalue.rs"))?;
        writeln!(file, "//! {}", autogen_message)?;
        writeln!(file)?;
        writeln!(file, "#[allow(unused_imports)]")?;
        writeln!(file, "use crate::prelude::*;")?;
        writeln!(file)?;
        writeln!(file, "#[allow(private_interfaces)]")?;
        writeln!(file, "extern \"C\" {{")?;
        writeln!(file)?;
        for game_value in &mut out.game_values {
            let name = game_value.icon.name.replace(" ", "");
            for target in ["Default", "Damager", "Killer", "Victim", "Shooter", "Projectile", "LastEntity", "Selection"] {
                util::write_doc_comment(&mut file, "    ",
                    &format!("{} : {target}", game_value.icon.name),
                    &mut game_value.icon.deprecated_note,
                    Some(&game_value.icon.return_type),
                    &game_value.icon.description,
                    None,
                    &game_value.icon.example,
                    &game_value.icon.works_with,
                    &game_value.icon.additional_info,
                    &game_value.icon.required_rank,
                    &autogen_message
                )?;
                writeln!(file, "    pub fn DF_GAMEVALUE__{}_{}() -> *const crate::bind::DFOpaqueValue;", name, target)?;
                if (game_value.category == "Plot Values" || game_value.category == "Event Values") { break; }
            }

            writeln!(file)?;
        }
        writeln!(file, "}}")?;
    }


    Ok(())
}
