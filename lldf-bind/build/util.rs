/*#[track_caller]
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
}*/


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
