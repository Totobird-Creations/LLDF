#![feature(decl_macro)]


mod dbc;
mod util;

use std::env::current_dir;
use std::fs::File;
use std::io::{ self, Write };
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


    // Doc
    {
        let mut file = File::create(cwd.join("src/bind/doc.rs"))?;
        writeln!(file, "pub macro dfdoc {{")?;
        for action in &mut out.actions {

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
                &autogen_message
            )?;
            writeln!(file, "        $($item)*")?;
            writeln!(file, "    }},")?;

        }
        writeln!(file, "    ( {{ $a:ident / $b:ident }} {{ $($_:tt)* }} ) => {{")?;
        writeln!(file, "        compile_error!(concat!(\"Unknown target `\", stringify!($a), \"/\", stringify!($b), \"`\"));")?;
        writeln!(file, "    }}")?;
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
