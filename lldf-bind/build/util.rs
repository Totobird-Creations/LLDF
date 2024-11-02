use crate::dbc::{ DBCRank, DBCValueType, DBCActionTag };

use std::io;


#[track_caller]
pub fn symbols_to_names(from : &str) -> String {
    let mut out = String::with_capacity(from.len());
    for ch in from.chars() { match (ch) {
        '+'  => out.push_str("Specialcharplus"),
        '-'  => out.push_str("Specialcharminus"),
        '/'  => out.push_str("Specialcharslash"),
        '%'  => out.push_str("Specialcharpercent"),
        '!'  => out.push_str("Specialcharexclamation"),
        '='  => out.push_str("Specialcharequals"),
        '['  => out.push_str("Specialcharleftbracket"),
        ']'  => out.push_str("Specialcharrightbracket"),
        '<'  => out.push_str("Specialcharleftangle"),
        '>'  => out.push_str("Specialcharrightangle"),
        '('  => out.push_str("Specialcharleftparenthesis"),
        ')'  => out.push_str("Specialcharrightparenthesis"),
        '\'' => out.push_str("Specialcharapostrophe"),
        ','  => out.push_str("Specialcharcomma"),
        '|'  => out.push_str("Specialcharpipe"),
        '&'  => out.push_str("Specialcharampersand"),
        '~'  => out.push_str("Specialchartilde"),
        '^'  => out.push_str("Specialcharcaret"),
        ':'  => out.push_str("Specialcharcolon"),
        '.'  => out.push_str("Specialcharperiod"),
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | ' ' => out.push(ch),
        _ => { panic!("Could not convert symbol character {:?} to a name.", ch) }
    } }
    out
}


pub fn remove_sections(from : &str) -> String {
    let mut out   = String::with_capacity(from.len());
    let mut chars = from.chars();
    while let Some(ch) = chars.next() { match (ch) {
        '§' => { chars.next(); },
        '<' => { out.push_str("&lt;"); }
        '>' => { out.push_str("&ht;"); }
        '[' => { out.push_str("\\["); }
        ']' => { out.push_str("\\]"); }
        _   => { out.push(ch); }
    } }
    out.shrink_to_fit();
    out
}


pub fn write_doc_comment<W : io::Write>(
    f               : &mut W,
    prefix          : &str,
    title           : &str,
    deprecated_note : &mut Vec<String>,
    typ             : Option<&DBCValueType>,
    description     : &Vec<String>,
    tags            : Option<&Vec<DBCActionTag>>,
    examples        : &Vec<String>,
    works_with      : &Vec<String>,
    additional_info : &Vec<Vec<String>>,
    required_rank   : &DBCRank,
    // TODO: arguments and return value.
    autogen_message : &str
) -> io::Result<()> {
    if (description.is_empty()) {
        deprecated_note.push("*Assumed.*".to_string());
    }

    writeln!(f, "{prefix}/// ### `{}`", remove_sections(title))?;

    if (deprecated_note.len() > 0) {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### **__Deprecated__**")?;
        for line in &*deprecated_note {
            writeln!(f, "{prefix}/// {}", remove_sections(line))?;
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
            writeln!(f, "{prefix}/// {}", remove_sections(line))?;
        }
    } else {
        writeln!(f, "{prefix}/// *No description available.*")?;
    }

    if let Some(tags) = tags {
        if (tags.len() > 0) {
            writeln!(f, "{prefix}/// ")?;
            writeln!(f, "{prefix}/// ##### Tags")?;
            for tag in tags {
                writeln!(f, "{prefix}/// - **{}**", tag.name)?;
                for option in &tag.options {
                    write!(f, "{prefix}///   - `{}`", option.name)?;
                    if (option.name == tag.default_option) {
                        write!(f, " (default)")?;
                    }
                    writeln!(f, ":")?;
                    if (option.icon.description.len() > 0) {
                        for line in description {
                            writeln!(f, "{prefix}///       {}", remove_sections(line))?;
                        }
                    } else {
                        writeln!(f, "{prefix}///       *No description available.*")?;
                    }
                }
            }
        }
    }

    if (examples.len() > 0) {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### Examples")?;
        for line in examples {
            writeln!(f, "{prefix}/// - {}", remove_sections(line))?;
        }
    }

    if (works_with.len() > 0) {
        writeln!(f, "{prefix}/// ")?;
        writeln!(f, "{prefix}/// ##### Works With")?;
        for line in works_with {
            writeln!(f, "{prefix}/// - {}", remove_sections(line))?;
        }
    }

    if (additional_info.len() > 0) {
        for group in additional_info {
            writeln!(f, "{prefix}/// ")?;
            writeln!(f, "{prefix}/// ##### Additional Info")?;
            for line in group {
                writeln!(f, "{prefix}/// {}", remove_sections(line))?;
            }
        }
    }

    if let DBCRank::None = required_rank {} else {
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
