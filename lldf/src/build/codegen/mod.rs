mod value;
pub use value::*;
pub mod opt;


use std::io::Write;
use std::error::Error;

use serde_json as json;

use flate2::Compression;
use flate2::write::GzEncoder;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;


pub struct CodeLine {
    pub blocks : Vec<Codeblock>
}
impl CodeLine {
    pub fn new() -> Self { Self {
        blocks : Vec::new()
    } }
}

#[derive(Debug, Clone)]
pub enum Codeblock {
    Block(CodeblockBlock),
    Bracket {
        kind : BracketKind,
        side : BracketSide
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CodeblockBlock {
    pub block  : String,
    pub action : Option<String>,
    pub data   : Option<String>,
    pub attr   : Option<String>,
    pub params : Vec<CodeValue>,
    pub tags   : Vec<CodeValue>
}
#[derive(Debug, Clone)]
pub enum BracketKind {
    Normal,
    Repeat
}
#[derive(Debug, Clone)]
pub enum BracketSide {
    Open,
    Close
}


impl Codeblock {

    pub const OPEN_COND_BRACKET  : Self = Self::Bracket { kind : BracketKind::Normal, side : BracketSide::Open  };
    pub const CLOSE_COND_BRACKET : Self = Self::Bracket { kind : BracketKind::Normal, side : BracketSide::Close };

    pub fn event<C : Into<String>, A : Into<String>>(codeblock : C, action : A) -> Self { Self::Block(CodeblockBlock {
        block  : codeblock.into(),
        action : Some(action.into()),
        data   : None,
        attr   : Some(String::from("LS-CANCEL")),
        params : vec![],
        tags   : vec![]
    }) }

    pub fn function<S : Into<String>>(data : S, params : Vec<CodeValue>, hidden : bool) -> Self { Self::Block(CodeblockBlock {
        block  : String::from("func"),
        action : None,
        data   : Some(data.into()),
        attr   : None,
        params,
        tags   : vec![ CodeValue::Actiontag {
            kind     : String::from("Is Hidden"),
            value    : String::from(if (hidden) { "True" } else { "False" }),
            variable : None
        } ]
    }) }

    pub fn call_func<S : Into<String>>(data : S, params : Vec<CodeValue>) -> Self { Self::Block(CodeblockBlock {
        block  : String::from("call_func"),
        action : None,
        data   : Some(data.into()),
        attr   : None,
        params,
        tags   : Vec::new()
    }) }

    pub fn action<C : Into<String>, A : Into<String>>(codeblock : C, action : A, params : Vec<CodeValue>, tags : Vec<CodeValue>) -> Self { Self::Block(CodeblockBlock {
        block  : codeblock.into(),
        action : Some(action.into()),
        data   : None,
        attr   : None,
        params,
        tags
    }) }

    pub fn ifs<C : Into<String>, A : Into<String>>(codeblock : C, action : A, not : bool, params : Vec<CodeValue>, tags : Vec<CodeValue>) -> Self { Self::Block(CodeblockBlock {
        block  : codeblock.into(),
        action : Some(action.into()),
        data   : None,
        attr   : not.then(|| String::from("NOT")),
        params,
        tags
    }) }

    pub fn elses() -> Self { Self::Block(CodeblockBlock {
        block  : String::from("else"),
        action : None,
        data   : None,
        attr   : None,
        params : Vec::new(),
        tags   : Vec::new()
    }) }

}

impl CodeLine {
    pub fn to_json(&self) -> json::Value {
        let mut blocks : Vec<json::Value> = Vec::new();
        for block in &self.blocks {
            blocks.push(block.to_json());
        }
        json::json!({ "blocks" : blocks })
    }
    pub fn to_b64(&self) -> String {
        let json = json::to_string(&self.to_json()).unwrap();
        let mut e = GzEncoder::new(Vec::new(), Compression::best());
        e.write_all(json.as_bytes()).unwrap();
        let compressed = e.finish().unwrap();
        BASE64_STANDARD.encode(compressed)
    }
}

impl Codeblock {

    pub fn to_json(&self) -> json::Value { match (self) {
        Self::Block(block) => block.to_json(),
        Self::Bracket { kind, side } => json::json!({
            "id"     : "bracket",
            "direct" : match (side) {
                BracketSide::Open  => "open",
                BracketSide::Close => "close",
            },
            "type" : match (kind) {
                BracketKind::Normal => "norm",
                BracketKind::Repeat => "repeat",
            }
        })
    } }

    pub fn contains_param(&self, var_name : &str) -> bool { match (self) {
        Codeblock::Block(block) => block.contains_param(var_name),
        _ => false
    } }

    pub fn can_replace_line_var_with_constant(&self, var_name : &str) -> bool { match (self) {
        Codeblock::Block(block) => block.can_replace_line_var_with_constant(var_name),
        _ => true
    } }
    pub fn replace_line_var_with_constant(&mut self, var_name : &str, with : &CodeValue) -> () { match (self) {
        Codeblock::Block(block) => block.replace_line_var_with_constant(var_name, with),
        _ => { }
    } }

    pub fn is_line_var_used(&self, var_name : &str) -> bool { match (self) {
        Codeblock::Block(block) => block.is_line_var_used(var_name),
        _ => false
    } }

    pub fn setvar_like_line(&self) -> (bool, Option<&String>) { match (self) {
        Codeblock::Block(block) => block.setvar_like_line(),
        _ => (false, None)
    } }

}

impl CodeblockBlock {

    pub fn to_json(&self) -> json::Value {
        let mut items = Vec::new();
        for (i, param) in self.params.iter().enumerate() {
            items.push(json::json!({
                "item" : param.to_json(&self.block, self.action.as_ref().map_or("dynamic", |a| a)),
                "slot" : i
            }));
        }
        for (i, tag) in self.tags.iter().rev().enumerate() {
            items.push(json::json!({
                "item" : tag.to_json(&self.block, self.action.as_ref().map_or("dynamic", |a| a)),
                "slot" : 26 - i
            }));
        }
        let mut block = json::json!({
            "id"    : "block",
            "block" : self.block,
            "args"  : {
                "items" : items
            }
        });
        if let Some(action ) = &self.action { block["action"    ] = json::Value::String(action .to_string()); }
        if let Some(data   ) = &self.data   { block["data"      ] = json::Value::String(data   .to_string()); }
        if let Some(attr   ) = &self.attr   { block["attribute" ] = json::Value::String(attr   .to_string()); }
        block
    }

    pub fn contains_param(&self, var_name : &str) -> bool {
        self.params.iter().any(|param| param.contains_param(var_name))
            || self.tags.iter().any(|tag| tag.contains_param(var_name))
    }

    pub fn can_replace_line_var_with_constant(&self, var_name : &str) -> bool {
        if (self.block == "call_func") {
            ! self.is_line_var_used(var_name)
        } else {
            self.params.iter().all(|param| param.can_replace_line_var_with_constant(var_name))
                || self.tags.iter().all(|tag| tag.can_replace_line_var_with_constant(var_name))
        }
    }
    pub fn replace_line_var_with_constant(&mut self, var_name : &str, with : &CodeValue) -> () {
        for param in &mut self.params {
            param.replace_line_var_with_constant(var_name, with);
        }
        for tag in &mut self.tags {
            tag.replace_line_var_with_constant(var_name, with);
        }
    }

    pub fn is_line_var_used(&self, var_name : &str) -> bool {
        let (setvar_like, _) = self.setvar_like_line();
        self.params.iter().skip(if (setvar_like) { 1 } else { 0 }).any(|param| param.is_line_var_used(var_name))
            || self.tags.iter().any(|tag| tag.is_line_var_used(var_name))
    }

    pub fn setvar_like_line(&self) -> (bool, Option<&String>) { if let Some(action) = &self.action {
        if (
            self.block == "set_var"
            || (self.block == "repeat" && (action != "Forever" && action != "While"))
            || (self.block == "entity_action" && (action == "GetCustomTag" || action == "GetAllEntityTags"))
        ) { return (true,
            if let Some(CodeValue::Variable { name, scope : VariableScope::Line }) = self.params.get(0) {
                Some(name)
            } else { None }
        ); }
    } (false, None) }

}

impl BracketKind { pub fn from<S : AsRef<str>>(from : S) -> Result<Self, Box<dyn Error>> { match (from.as_ref()) {
    "Normal" => Ok(Self::Normal),
    "Repeat" => Ok(Self::Repeat),
    kind => Err(format!("Invalid bracket kind {:?}", kind).into())
} } }

impl BracketSide { pub fn from<S : AsRef<str>>(from : S) -> Result<Self, String> { match (from.as_ref()) {
    "Open"  => Ok(Self::Open),
    "Close" => Ok(Self::Close),
    side => Err(format!("Invalid bracket side {:?}", side).into())
} } }
