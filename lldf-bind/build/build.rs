#![feature(
    decl_macro,
    iter_intersperse
)]


mod dbc;
mod data;
mod util;

use std::env::current_dir;
use std::fs::File;
use std::io::{ self, Write };
use std::error::Error;
use std::env::var_os;

use dirs::download_dir;
use chrono::Utc;
use inflector::Inflector;


fn main() -> Result<(), Box<dyn Error>> {
    let cwd = current_dir()?;

    // TODO: find a public actiondump api.
    //let data = reqwest::blocking::get("https://dfonline.dev/public/db.json")?.text()?; // OUTDATED
    let mut dbc : dbc::DBC = serde_json::from_str(&std::fs::read_to_string(download_dir().unwrap().join("actiondump.json"))?)?;
    let version = get_game_version();
    let items  : Option<data::Items  > = if let Some(version) = &version { serde_json::from_str(&reqwest::blocking::get(format!("https://github.com/PrismarineJS/minecraft-data/raw/refs/heads/master/data/pc/{}/items.json",  version))?.text()?)? } else { None };
    //let sounds : Option<data::Sounds > = if let Some(version) = &version { serde_json::from_str(&reqwest::blocking::get(format!("https://github.com/PrismarineJS/minecraft-data/raw/refs/heads/master/data/pc/{}/sounds.json", version))?.text()?)? } else { None };

    let dbc_autogen_message = format!("*\\[{}\\] Automatically generated from the action dump.*", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));

    // Doc
    {
        let mut file = File::create(cwd.join("src/bind/doc.rs"))?;
        writeln!(file, "pub macro dfdoc {{")?;
        for action in &mut dbc.actions {

            let code_block_name = action.code_block_name.to_title_case().replace(" ", "");
            let action_name     = action.name.to_title_case().replace(" ", "");
            writeln!(file, "    ( {{ {} / {} $( {{ $($tagident:ident = $tagvalue:ident),* }} )? }} {{ $($item:tt)* }} ) => {{", code_block_name, action_name)?;
            write_doc_comment(
                &mut file,
                "        ",
                &format!("{}: {}", code_block_name, action_name),
                &mut action.icon.deprecated_note, true,
                None,
                &action.icon.description,
                true,
                &action.icon.example,
                &action.icon.works_with,
                &action.icon.additional_info,
                &action.icon.required_rank,
                &dbc_autogen_message
            )?;
            writeln!(file, "        $($item)*")?;
            writeln!(file, "    }},")?;

        }
        writeln!(file, "    ( {{ $a:ident / $b:ident $( {{ $($_tagident:ident = $_tagvalue:ident),* }} )? }} {{ $($_:tt)* }} ) => {{")?;
        writeln!(file, "        crate::core::compile_error!(crate::core::concat!(\"Unknown target `\", crate::core::stringify!($a), \"/\", crate::core::stringify!($b), \"`\"));")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
    }

    // Items
    {
        let mut file = File::create(cwd.join("src/bind/item.rs"))?;
        if let Some(items) = items {
            writeln!(file, "extern \"C\" {{")?;
            for item in &items {
                let name = item.name.to_title_case().replace(" ", "");
                writeln!(file, "    fn DF_ITEM__{}( ) -> Item;", name)?;
            }
            writeln!(file, "}}")?;
            writeln!(file, "impl Item {{")?;
            for item in &items {
                let name = item.name.to_title_case().replace(" ", "");
                writeln!(file, "    /// `{}`", item.display_name)?;
                writeln!(file, "    #[inline(always)] pub fn {}() -> Self {{ unsafe {{ DF_ITEM__{}() }} }}", item.name, name)?;
            }
            writeln!(file, "}}")?;
        }
    }

    // Sounds
    dbc.sounds.sort_by(|a, b| a.icon.name.cmp(&b.icon.name));
    dbc.sounds.dedup_by(|a, b| a.icon.name == b.icon.name);
    {
        let mut file = File::create(cwd.join("src/bind/sound.rs"))?;
        writeln!(file, "extern \"C\" {{")?;
        for sound in &dbc.sounds {
            let name = util::symbols_to_names(&sound.icon.name).to_title_case().replace(" ", "");
            write!(file, "    fn DF_SOUND__{}", name)?;
            writeln!(file, "( ) -> Sound;")?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "impl Sound {{")?;
        for sound in &mut dbc.sounds {
            let name = util::symbols_to_names(&sound.icon.name).to_title_case().replace(" ", "");
            write_doc_comment(&mut file, "    ",
                &sound.icon.name,
                &mut sound.icon.deprecated_note, false,
                None,
                &sound.icon.description,
                false,
                &sound.icon.example,
                &sound.icon.works_with,
                &sound.icon.additional_info,
                &sound.icon.required_rank,
                &dbc_autogen_message
            )?;
            writeln!(file, "    #[inline(always)] pub fn {}() -> Self {{ unsafe {{ DF_SOUND__{}() }} }}", sound.icon.name.to_snake_case(), name)?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "#[lldf_bind_proc::actiontag]")?;
        writeln!(file, "pub enum SoundKind {{")?;
        for sound in &mut dbc.sounds {
            let name = util::symbols_to_names(&sound.icon.name).to_title_case().replace(" ", "");
            writeln!(file, "    {} = \"{}\",", name, sound.icon.name)?;
        }
        writeln!(file, "}}")?;
    }

    // Particles
    dbc.particles.sort_by(|a, b| a.particle.cmp(&b.particle));
    dbc.particles.dedup_by(|a, b| a.particle == b.particle);
    {
        let mut file = File::create(cwd.join("src/bind/particle.rs"))?;
        writeln!(file, "extern \"C\" {{")?;
        for particle in &dbc.particles {
            let name = util::symbols_to_names(&particle.icon.name).to_title_case().replace(" ", "");
            write!(file, "    fn DF_PARTICLE__{}", name)?;
            for field in &particle.fields {
                write!(file, "_{:?}", field)?;
            }
            writeln!(file, "( ) -> Particle;")?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "impl Particle {{")?;
        for particle in &mut dbc.particles {
            let name = util::symbols_to_names(&particle.icon.name).to_title_case().replace(" ", "");
            write_doc_comment(&mut file, "    ",
                &particle.icon.name,
                &mut particle.icon.deprecated_note, false,
                None,
                &particle.icon.description,
                false,
                &particle.icon.example,
                &particle.icon.works_with,
                &particle.icon.additional_info,
                &particle.icon.required_rank,
                &dbc_autogen_message
            )?;
            write!(file, "    #[inline(always)] pub fn {}() -> Self {{ unsafe {{ DF_PARTICLE__{}", particle.icon.name.to_snake_case(), name)?;
            for field in &particle.fields {
                write!(file, "_{:?}", field)?;
            }
            writeln!(file, "() }} }}")?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "#[lldf_bind_proc::actiontag]")?;
        writeln!(file, "pub enum ParticleKind {{")?;
        for particle in &mut dbc.particles {
            let name = util::symbols_to_names(&particle.icon.name).to_title_case().replace(" ", "");
            writeln!(file, "    {} = \"{}\",", name, particle.icon.name)?;
        }
        writeln!(file, "}}")?;
    }

    // Potions
    {
        let mut file = File::create(cwd.join("src/bind/potion.rs"))?;
        writeln!(file, "extern \"C\" {{")?;
        for potion in &dbc.potions {
            let name = util::symbols_to_names(&potion.icon.name).to_title_case().replace(" ", "");
            write!(file, "    fn DF_POTION__{}", name)?;
            writeln!(file, "( ) -> Potion;")?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "impl Potion {{")?;
        for potion in &mut dbc.potions {
            let name = util::symbols_to_names(&potion.icon.name).to_title_case().replace(" ", "");
            write_doc_comment(&mut file, "    ",
                &potion.icon.name,
                &mut potion.icon.deprecated_note, false,
                None,
                &potion.icon.description,
                false,
                &potion.icon.example,
                &potion.icon.works_with,
                &potion.icon.additional_info,
                &potion.icon.required_rank,
                &dbc_autogen_message
            )?;
            writeln!(file, "    #[inline(always)] pub fn {}() -> Self {{ unsafe {{ DF_POTION__{}() }} }}", potion.icon.name.replace("'", "").to_snake_case(), name)?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "#[lldf_bind_proc::actiontag]")?;
        writeln!(file, "pub enum PotionKind {{")?;
        for potion in &mut dbc.potions {
            let name = util::symbols_to_names(&potion.icon.name).to_title_case().replace(" ", "");
            writeln!(file, "    {} = \"{}\",", name, potion.icon.name)?;
        }
        writeln!(file, "}}")?;
    }


    Ok(())
}


fn write_doc_comment<W : Write>(
    f                 : &mut W,
    prefix            : &str,
    title             : &str,
    deprecated_note   : &mut Vec<String>,
    assume_deprecated : bool,
    typ               : Option<&dbc::DBCValueType>,
    description       : &Vec<String>,
    tags              : bool,
    examples          : &Vec<String>,
    works_with        : &Vec<String>,
    additional_info   : &Vec<Vec<String>>,
    required_rank     : &dbc::DBCRank,
    // TODO: Arguments and return value
    autogen_message   : &str
) -> io::Result<()> {
    if (assume_deprecated && description.is_empty()) {
        deprecated_note.push("*Assumed.*".to_string());
    }

    writeln!(f, "{prefix}/// ### `{}`", util::remove_sections(title))?;

    if (deprecated_note.len() > 0) {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### **__Deprecated__**")?;
        for line in &*deprecated_note {
            writeln!(f, "{prefix}/// {}", util::remove_sections(line))?;
        }
    }

    if let Some(typ) = typ {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### Type")?;
        writeln!(f, "{prefix}/// {:?}", typ)?;
    }

    writeln!(f, "{prefix}/// ")?;
    writeln!(f, "{prefix}/// ##### Description")?;
    if (description.len() > 0) {
        for line in description {
            writeln!(f, "{prefix}/// {}", util::remove_sections(line))?;
        }
    } else {
        writeln!(f, "{prefix}/// *No description available.*")?;
    }

    if (tags) {
        writeln!(f, "{prefix}$(")?;
        writeln!(f, "{prefix}    /// ")?;
        writeln!(f, "{prefix}    /// ##### Tags")?;
        writeln!(f, "{prefix}    $(")?;
        writeln!(f, "{prefix}        #[doc = crate::core::concat!(\"- `\", crate::core::stringify!($tagident), \"`: `\", crate::core::stringify!($tagvalue), \"`\")]")?;
        writeln!(f, "{prefix}    )*")?;
        writeln!(f, "{prefix})?")?;
    }

    if (examples.len() > 0) {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### Examples")?;
        for line in examples {
            writeln!(f, "{prefix}/// - {}", util::remove_sections(line))?;
        }
    }

    if (works_with.len() > 0) {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### Works With")?;
        for line in works_with {
            writeln!(f, "{prefix}/// - {}", util::remove_sections(line))?;
        }
    }

    if (additional_info.len() > 0) {
        for group in additional_info {
            writeln!(f, "{prefix}/// ")?;
            writeln!(f, "{prefix}/// ##### Additional Info")?;
            for line in group {
                writeln!(f, "{prefix}/// {}", util::remove_sections(line))?;
            }
        }
    }

    if let dbc::DBCRank::None = required_rank {} else {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### Requires Rank: **<u>{:?}</u>**", required_rank)?;
    }

    writeln!(f, "{prefix}/// ")?;
    writeln!(f, "{prefix}/// ## ")?;
    writeln!(f, "{prefix}/// {}", autogen_message)?;

    writeln!(f, "{prefix}{}", required_rank.to_rank_feature_gate())?;
    if let Some(rank_doc_gate) = required_rank.to_rank_doc_gate() {
        writeln!(f, "{prefix}{}", rank_doc_gate)?;
    }

    if (deprecated_note.len() > 0) {
        writeln!(f, "{prefix}#[deprecated] ")?;
    }

    Ok(())
}


fn get_game_version() -> Option<String> {
    fn get_game_version_cases() -> Result<(), String> {
        get_game_version_case("1_21_1")?;
        Ok(())
    }
    fn get_game_version_case(version : &str) -> Result<(), String> {
        if let Some(_) = var_os(format!("CARGO_FEATURE_MC_{}", version)) { Err(version.replace("_", ".")) } else { Ok(()) }
    }
    get_game_version_cases().err()
}
