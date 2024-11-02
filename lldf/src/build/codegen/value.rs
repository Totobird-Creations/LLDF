use std::error::Error;

use llvm_ir::Name;
use serde_json as json;


#[derive(Debug, Clone, PartialEq)]
pub enum CodeValue {
    String(String),
    Text(String),
    Number(f64),
    Location { x : f64, y : f64, z : f64, pitch : f64, yaw : f64 },
    Vector { x : f64, y : f64, z : f64 },
    SoundNamed { name : String, volume : f64, pitch : f64 },
    SoundKeyed { key : String, volume : f64, pitch : f64 },
    Particle {
        kind             : String,
        cluster_x        : f64,
        cluster_y        : f64,
        cluster_amount   : u64,
        motion           : Option<(f64, f64, f64)>,
        motion_variation : Option<f64>,
        colour           : Option<u32>,
        colour_variation : Option<f64>,
        material         : Option<String>,
        size             : Option<f64>,
        size_variation   : Option<f64>,
        roll             : Option<f64>,
        fade_colour      : Option<u32>
    },
    Potion {
        kind : String,
        dur  : u32,
        amp  : u8
    },
    Variable {
        name  : String,
        scope : VariableScope
    },
    Gamevalue {
        kind   : String,
        target : String
    },
    Parameter {
        name        : String,
        typ         : ParameterType,
        plural      : bool,
        optional    : bool,
        description : Option<String>,
        note        : Option<String>
    },
    Actiontag {
        kind     : String,
        value    : String,
        variable : Option<Box<CodeValue>>
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum VariableScope {
    Unsaved,
    Saved,
    Local,
    Line
}
impl AsRef<str> for VariableScope {
    fn as_ref(&self) -> &str { match (self) {
        Self::Unsaved => "unsaved",
        Self::Saved   => "saved",
        Self::Local   => "local",
        Self::Line    => "line"
    } }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    String,
    Text,
    Number,
    Location,
    Vector,
    Sound,
    Particle,
    Potion,
    Item,
    Any,
    Variable,
    List,
    Dict
}
impl AsRef<str> for ParameterType {
    fn as_ref(&self) -> &str { match (self) {
        Self::String   => "txt",
        Self::Text     => "comp",
        Self::Number   => "num",
        Self::Location => "loc",
        Self::Vector   => "vec",
        Self::Sound    => "snd",
        Self::Particle => "par",
        Self::Potion   => "pot",
        Self::Item     => "item",
        Self::Any      => "any",
        Self::Variable => "var",
        Self::List     => "list",
        Self::Dict     => "dict"
    } }
}


impl CodeValue {

    pub fn line_variable_name(name : &Name) -> Self {
        Self::Variable { name : match (name) {
            Name::Name   (name   ) => format!("local.name.{}",  name    ),
            Name::Number (number ) => format!("local.number.{}", number )
        }, scope : VariableScope::Line }
    }

    pub fn line_variable<S : Into<String>>(name : S) -> Self {
        Self::Variable { name : name.into(), scope : VariableScope::Line }
    }

    pub fn unsaved_variable_name(name : &Name) -> Self {
        Self::Variable { name : match (name) {
            Name::Name   (name   ) => format!("global.name.{}",  name    ),
            Name::Number (number ) => format!("global.number.{}", number )
        }, scope : VariableScope::Unsaved }
    }

    pub fn unsaved_variable<S : Into<String>>(name : S) -> Self {
        Self::Variable { name : name.into(), scope : VariableScope::Unsaved }
    }


    pub fn is_constant(&self) -> bool { match (self) {
        CodeValue::String     ( _  ) => true,
        CodeValue::Text       ( _  ) => true,
        CodeValue::Number     ( _  ) => true,
        CodeValue::Location   { .. } => true,
        CodeValue::Vector     { .. } => true,
        CodeValue::SoundNamed { .. } => true,
        CodeValue::SoundKeyed { .. } => true,
        CodeValue::Particle   { .. } => true,
        CodeValue::Potion     { .. } => true,
        CodeValue::Variable   { .. } => false,
        CodeValue::Gamevalue  { .. } => false,
        CodeValue::Parameter  { .. } => false,
        CodeValue::Actiontag  { .. } => false,
    } }

}


impl CodeValue {

    pub fn as_actiontag(&self) -> Result<(String, Option<CodeValue>), Box<dyn Error>> { match (self) {
        Self::String(string) => Ok((string.clone(), None)),
        var @ Self::Variable { .. } => Ok((String::new(), Some(var.clone()))),
        _ => Err("Non string nor variable value used as tag".into())
    } }

    pub fn replace_line_var(&mut self, var_name : &str, with : &CodeValue) -> () { match (self) {
        CodeValue::Variable  { name, scope : VariableScope::Line } => { if (name == var_name) { *self = with.clone() } },
        CodeValue::Parameter { name, ..                          } => { if (name == var_name) { panic!("Attempted to replace line var in parameter") } },
        CodeValue::Actiontag { variable : Some(variable), ..     } => variable.replace_line_var(var_name, with),
        _ => { }
    } }

    pub fn contains_line_var(&self, var_name : &str) -> bool { match (self) {
        CodeValue::Variable  { name, scope : VariableScope::Line } => name == var_name,
        CodeValue::Parameter { name, ..                          } => name == var_name,
        CodeValue::Actiontag { variable : Some(variable), ..     } => variable.contains_line_var(var_name),
        _ => false
    } }

}


impl CodeValue {
    pub fn to_json(&self, codeblock : &str, action : &str) -> json::Value { match (self) {
        Self::String     ( string              ) => json::json!({ "id" : "txt",  "data" : { "name" : string } }),
        Self::Text       ( text                ) => json::json!({ "id" : "comp", "data" : { "name" : text } }),
        Self::Number     ( num                 ) => json::json!({ "id" : "num",  "data" : { "name" : num.to_string() } }),
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
