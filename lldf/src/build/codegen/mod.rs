mod value;
pub use value::*;
pub mod opt;


use std::io::Write;

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

#[derive(Debug)]
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
    pub params : Vec<CodeValue>,
    pub tags   : Vec<CodeValue>
}
#[derive(Debug)]
pub enum BracketKind {
    Normal,
    Repeat
}
#[derive(Debug)]
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
        params : vec![],
        tags   : vec![]
    }) }

    pub fn function<S : Into<String>>(data : S, params : Vec<CodeValue>, hidden : bool) -> Self { Self::Block(CodeblockBlock {
        block  : String::from("func"),
        action : None,
        data   : Some(data.into()),
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
        params,
        tags   : Vec::new()
    }) }

    pub fn action<C : Into<String>, A : Into<String>>(codeblock : C, action : A, params : Vec<CodeValue>, tags : Vec<CodeValue>) -> Self { Self::Block(CodeblockBlock {
        block  : codeblock.into(),
        action : Some(action.into()),
        data   : None,
        params,
        tags
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
        Self::Bracket { .. } => todo!()
    } }

    pub fn replace_line_var(&mut self, var_name : &str, with : &CodeValue) -> () { match (self) {
        Codeblock::Block(block) => block.replace_line_var(var_name, with),
        _ => { }
    } }

    pub fn contains_line_var(&self, var_name : &str) -> bool { match (self) {
        Codeblock::Block(block) => block.contains_line_var(var_name),
        _ => false
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
        if let Some(action ) = &self.action { block["action" ] = json::Value::String(action .to_string()); }
        if let Some(data   ) = &self.data   { block["data"   ] = json::Value::String(data   .to_string()); }
        block
    }

    pub fn replace_line_var(&mut self, var_name : &str, with : &CodeValue) -> () {
        for param in &mut self.params {
            param.replace_line_var(var_name, with);
        }
        for tag in &mut self.tags {
            tag.replace_line_var(var_name, with);
        }
    }

    pub fn contains_line_var(&self, var_name : &str) -> bool {
        self.params.iter().any(|param| param.contains_line_var(var_name))
            || self.tags.iter().any(|tag| tag.contains_line_var(var_name))
    }

}
