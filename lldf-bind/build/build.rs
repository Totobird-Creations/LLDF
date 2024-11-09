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
    let sounds : Option<data::Sounds > = if let Some(version) = &version { serde_json::from_str(&reqwest::blocking::get(format!("https://github.com/PrismarineJS/minecraft-data/raw/refs/heads/master/data/pc/{}/sounds.json", version))?.text()?)? } else { None };

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
                &mut action.icon.deprecated_note,
                None,
                &action.icon.description,
                &action.icon.example,
                &action.icon.works_with,
                &action.icon.additional_info,
                &action.icon.required_rank,
                &dbc_autogen_message
            )?;
            writeln!(file, "        $($item)*")?;
            writeln!(file, "    }},")?;

        }
        writeln!(file, "    ( {{ $a:ident / $b:ident }} {{ $($_:tt)* }} ) => {{")?;
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
    {
        let mut file = File::create(cwd.join("src/bind/sound.rs"))?;
        if let Some(sounds) = sounds {
            writeln!(file, "extern \"C\" {{")?;
            for sound in &sounds {
                let name = sound.name.split(".").map(|part| part.to_title_case().replace(" ", "")).intersperse("_".to_string()).collect::<String>();
                writeln!(file, "    fn DF_SOUND__{}( ) -> Sound;", name)?;
            }
            writeln!(file, "}}")?;
            writeln!(file, "impl Sound {{")?;
            for sound in &sounds {
                let name = sound.name.split(".").map(|part| part.to_title_case().replace(" ", "")).intersperse("_".to_string()).collect::<String>();
                writeln!(file, "    /// `{}`", sound.name)?;
                writeln!(file, "    #[inline(always)] pub fn {}() -> Self {{ unsafe {{ DF_SOUND__{}() }} }}", sound.name.replace(".", "_"), name)?;
            }
            writeln!(file, "}}")?;
        }
    }

    // Particles
    dbc.particles.sort_by(|a, b| a.particle.cmp(&b.particle));
    dbc.particles.dedup_by(|a, b| a.particle == b.particle);
    {
        let mut file = File::create(cwd.join("src/bind/particle.rs"))?;
        writeln!(file, "extern \"C\" {{")?;
        for particle in &dbc.particles {
            let name = particle.particle.to_title_case().replace(" ", "");
            write!(file, "    fn DF_PARTICLE__{}", name)?;
            for field in &particle.fields {
                write!(file, "_{:?}", field)?;
            }
            writeln!(file, "( ) -> Particle;")?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "impl Particle {{")?;
        for particle in &dbc.particles {
            let name = particle.particle.to_title_case().replace(" ", "");
            writeln!(file, "    /// `{}`", particle.particle)?;
            write!(file, "    #[inline(always)] pub fn {}() -> Self {{ unsafe {{ DF_PARTICLE__{}", particle.particle.to_snake_case(), name)?;
            for field in &particle.fields {
                write!(file, "_{:?}", field)?;
            }
            writeln!(file, "() }} }}")?;
        }
        writeln!(file, "}}")?;
    }


    Ok(())
}


fn write_doc_comment<W : Write>(
    f               : &mut W,
    prefix          : &str,
    title           : &str,
    deprecated_note : &mut Vec<String>,
    typ             : Option<&dbc::DBCValueType>,
    description     : &Vec<String>,
    examples        : &Vec<String>,
    works_with      : &Vec<String>,
    additional_info : &Vec<Vec<String>>,
    required_rank   : &dbc::DBCRank,
    // TODO: Arguments and return value
    autogen_message : &str
) -> io::Result<()> {
    if (description.is_empty()) {
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

    writeln!(f, "{prefix}$(")?;
    writeln!(f, "{prefix}    /// ")?;
    writeln!(f, "{prefix}    /// ##### Tags")?;
    writeln!(f, "{prefix}    $(")?;
    writeln!(f, "{prefix}        #[doc = crate::core::concat!(\"- `\", crate::core::stringify!($tagident), \"`: `\", crate::core::stringify!($tagvalue), \"`\")]")?;
    writeln!(f, "{prefix}    )*")?;
    writeln!(f, "{prefix})?")?;

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
