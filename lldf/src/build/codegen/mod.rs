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
#[derive(Debug)]
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
}
impl CodeValue {
    pub fn to_json(&self, codeblock : &str, action : &str) -> json::Value { match (self) {
        Self::String     ( string              ) => json::json!({ "id" : "txt",  "data" : { "name" : string } }),
        Self::Text       ( text                ) => json::json!({ "id" : "comp", "data" : { "name" : text } }),
        Self::Int        ( num                 ) => json::json!({ "id" : "num",  "data" : { "name" : num.to_string() } }),
        Self::Float      ( num                 ) => json::json!({ "id" : "num",  "data" : { "name" : num.to_string() } }),
        Self::Location   { x, y, z, pitch, yaw } => json::json!({ "id" : "loc",  "data" : { "isBlock" : false, "loc" : { "x" : x, "y" : y, "z" : z, "pitch" : pitch, "yaw" : yaw } } }),
        Self::Vector     { x, y, z             } => json::json!({ "id" : "vec",  "data" : { "x" : x, "y" : y, "z" : z } }),
        Self::SoundNamed { name, volume, pitch } => json::json!({ "id" : "snd",  "data" : { "sound" : name, "vol" : volume, "pitch" : pitch } }),
        Self::SoundKeyed { key, volume, pitch  } => json::json!({ "id" : "snd",  "data" : { "key" : key, "vol" : volume, "pitch" : pitch } }),
        Self::Particle   { kind, cluster_x, cluster_y, cluster_amount, motion, motion_variation, colour, colour_variation, material, size, size_variation, roll, fade_colour } => {
            let mut data = json::json!({});
            if let Some((x, y, z)) = motion {
                data["x"] = json::Number::from_f64(*x).unwrap().into();
                data["y"] = json::Number::from_f64(*y).unwrap().into();
                data["z"] = json::Number::from_f64(*z).unwrap().into();
            }
            if let Some(motion_variation ) = motion_variation { data["motionVariation" ] = json::Number::from_f64  (*motion_variation    ).unwrap().into() }
            if let Some(colour           ) = colour           { data["rgb"             ] = json::Number::from_u128 (*colour as u128      ).unwrap().into() }
            if let Some(colour_variation ) = colour_variation { data["colorVariation"  ] = json::Number::from_f64  (*colour_variation    ).unwrap().into() }
            if let Some(material         ) = material         { data["material"        ] = json::Value::String(material.clone()) }
            if let Some(size             ) = size             { data["size"            ] = json::Number::from_f64  (*size                ).unwrap().into() }
            if let Some(size_variation   ) = size_variation   { data["sizeVariation"   ] = json::Number::from_f64  (*size_variation      ).unwrap().into() }
            if let Some(roll             ) = roll             { data["roll"            ] = json::Number::from_f64  (*roll                ).unwrap().into() }
            if let Some(fade_colour      ) = fade_colour      { data["rgb_fade"        ] = json::Number::from_u128 (*fade_colour as u128 ).unwrap().into() }
            json::json!({ "id" : "part", "data" : {
                "particle" : kind,
                "cluster"  : { "amount" : cluster_amount, "horizontal" : cluster_x, "vertical" : cluster_y },
                "data"     : data
            } })
        },
        Self::Potion    { kind, dur, amp } => json::json!({ "id" : "pot",    "data" : { "pot" : kind, "dur" : dur, "amp" : amp } }),
        Self::Variable  { name, scope    } => json::json!({ "id" : "var",    "data" : { "name" : name, "scope" : scope.as_ref() } }),
        Self::Gamevalue { kind, target   } => json::json!({ "id" : "g_val",  "data" : { "type" : kind, "target" : target } }),
        Self::Parameter { name, typ, plural, optional, description, note } => {
            let mut data = json::json!({
                "name"     : name,
                "type"     : typ.as_ref(),
                "plural"   : plural,
                "optional" : optional
            });
            if let Some(description ) = description { data["description" ] = json::Value::String(description .clone()) }
            if let Some(note        ) = note        { data["note"        ] = json::Value::String(note        .clone()) }
            json::json!({ "id" : "pn_el", "data" : data })
        },
        Self::Actiontag { kind, value, variable } => {
            let mut data = json::json!({
                "block"  : codeblock,
                "action" : action,
                "tag"    : kind,
                "option" : value
            });
            if let Some(variable) = variable { data["variable"] = variable.to_json(codeblock, action); }
            json::json!({ "id" : "bl_tag", "data" : data })
        }
    } }
}
