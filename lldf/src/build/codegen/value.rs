use std::error::Error;

use serde_json as json;


#[derive(Debug, Clone, PartialEq)]
pub enum CodeValue { // TODO: Add item
    String(String),
    Text(String),
    Number(String),
    Location { x : f64, y : f64, z : f64, pitch : f64, yaw : f64 },
    Vector { x : f64, y : f64, z : f64 },
    Sound { kind : SoundKind, volume : f64, pitch : f64 },
    Particle {
        kind             : String,
        spread_x         : f64,
        spread_y         : f64,
        amount           : u64,
        motion           : Option<(f64, f64, f64)>,
        motion_variation : Option<f64>,
        colour           : Option<u32>,
        colour_variation : Option<f64>,
        opacity          : Option<f64>,
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
    Item {
        id    : String,
        count : u8,
        nbt   : String
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
pub enum SoundKind {
    Named(String),
    Keyed(String)
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

    pub fn is_constant(&self) -> bool { match (self) {
        CodeValue::String(value) | CodeValue::Text(value) | CodeValue::Number(value)
            => ! (value.contains("%var(") || value.contains("%index(") || value.contains("%entry(")),
        CodeValue::Location  { .. } => true,
        CodeValue::Vector    { .. } => true,
        CodeValue::Sound     { .. } => true,
        CodeValue::Particle  { .. } => true,
        CodeValue::Potion    { .. } => true,
        CodeValue::Item      { .. } => true,
        CodeValue::Variable  { .. } => false,
        CodeValue::Gamevalue { .. } => false,
        CodeValue::Parameter { .. } => false,
        CodeValue::Actiontag { .. } => false,
    } }

    pub fn is_variable(&self) -> bool { match (self) {
        CodeValue::Variable { .. } => true,
        _                          => false
    } }

}


impl CodeValue {

    pub fn as_actiontag(&self) -> Result<(String, Option<CodeValue>), Box<dyn Error>> { match (self) {
        Self::String(string) => Ok((string.clone(), None)),
        var @ Self::Variable { .. } => Ok((String::new(), Some(var.clone()))),
        _ => Err("Non string nor variable value used as tag".into())
    } }

    pub fn contains_param(&self, var_name : &str) -> bool { match (self) {
        CodeValue::Parameter { name, .. } => name == var_name,
        _ => false
    } }

    pub fn can_replace_line_var_with_constant(&self, var_name : &str) -> bool { match (self) {
        CodeValue::String(value) | CodeValue::Text(value) | CodeValue::Number(value) | CodeValue::Variable { name : value, .. }
            => ! (value.contains(&format!("%index({},", var_name)) || value.contains(&format!("%entry({},", var_name))),
        CodeValue::Parameter { name, .. } => name != var_name,
        CodeValue::Actiontag { variable : Some(variable), .. } => variable.can_replace_line_var_with_constant(var_name),
        _ => true
    } }
    pub fn replace_line_var_with_constant(&mut self, var_name : &str, with : &CodeValue) -> () { match (self) {
        CodeValue::String(value) | CodeValue::Text(value) | CodeValue::Number(value)
            => {
                let replacing = format!("%var({})", var_name);
                if (value.contains(&replacing)) {
                    *value = value.replace(&replacing, &with.to_pvar_string());
                }
            },
        CodeValue::Variable { name, .. } => {
            if (name == var_name) { *self = with.clone(); }
            else {
                let replacing = format!("%var({})", var_name);
                if (name.contains(&replacing)) {
                    *name = name.replace(&replacing, &with.to_pvar_string());
                }
            }
        },
        CodeValue::Parameter { name, .. } => { if (name == var_name) { panic!("Attempted to replace line var in parameter") } },
        CodeValue::Actiontag { value, variable, .. } => {
            if let Some(var) = variable {
                var.replace_line_var_with_constant(var_name, with);
                if (var.is_constant()) {
                    *value    = var.to_pvar_string();
                    *variable = None;
                }
            }
        },
        _ => { }
    } }
    fn to_pvar_string(&self) -> String { match (self) {
        CodeValue::String   ( value    ) => value.clone(),
        CodeValue::Text     ( value    ) => value.clone(),
        CodeValue::Number   ( value    ) => value.to_string(),
        CodeValue::Variable { name, .. } => format!("%var({})", name),
        _ => unreachable!()
    } }

    pub fn is_line_var_used(&self, var_name : &str) -> bool { match (self) {
        CodeValue::String(value) | CodeValue::Text(value) | CodeValue::Number(value)
            => value.contains(&format!("%var({})", var_name)) || value.contains(&format!("%index({},", var_name)) || value.contains(&format!("%entry({},", var_name)),
        CodeValue::Variable { name, .. }
            => (name == var_name) || name.contains(&format!("%var({})", var_name)) || name.contains(&format!("%index({},", var_name)) || name.contains(&format!("%entry({},", var_name)),
        CodeValue::Parameter { name, .. } => name == var_name,
        CodeValue::Actiontag { variable : Some(variable), .. } => variable.is_line_var_used(var_name),
        _ => false
    } }
    pub fn replace_line_var(&mut self, var_name : &str, with : &str) -> () { match (self) {
        CodeValue::String(value) | CodeValue::Text(value) | CodeValue::Number(value)
            => { *value = value
                    .replace(&format!("%var({})", var_name), &format!("%var({})", with))
                    .replace(&format!("%index({},", var_name), &format!("%index({},", with))
                    .replace(&format!("%entry({},", var_name), &format!("%entry({},", with));
            },
        CodeValue::Variable { name, .. } => {
            if (name == var_name) { *name = with.to_string(); }
            else { *name = name
                    .replace(&format!("%var({})", var_name), &format!("%var({})", with))
                    .replace(&format!("%index({},", var_name), &format!("%index({},", with))
                    .replace(&format!("%entry({},", var_name), &format!("%entry({},", with));
            }
        },
        CodeValue::Parameter { name, .. } => { if (name == var_name) { *name = with.to_string(); } },
        CodeValue::Actiontag { variable : Some(variable), .. } => variable.replace_line_var(var_name, with),
        _ => { }
    } }

}


impl CodeValue {
    pub fn to_json(&self, codeblock : &str, action : &str) -> json::Value { match (self) {
        Self::String   ( string              ) => json::json!({ "id" : "txt",  "data" : { "name" : string } }),
        Self::Text     ( text                ) => json::json!({ "id" : "comp", "data" : { "name" : text } }),
        Self::Number   ( num                 ) => json::json!({ "id" : "num",  "data" : { "name" : num.to_string() } }),
        Self::Location { x, y, z, pitch, yaw } => json::json!({ "id" : "loc",  "data" : { "isBlock" : false, "loc" : { "x" : x, "y" : y, "z" : z, "pitch" : pitch, "yaw" : yaw } } }),
        Self::Vector   { x, y, z             } => json::json!({ "id" : "vec",  "data" : { "x" : x, "y" : y, "z" : z } }),
        Self::Sound    { kind, volume, pitch } => {
            let mut data = json::json!({
                "vol"   : volume,
                "pitch" : pitch
            });
            match (kind) {
                SoundKind::Named(name ) => { data["sound" ] = json::Value::String(name .clone()) },
                SoundKind::Keyed(key  ) => { data["key"   ] = json::Value::String(key  .clone()) },
            }
            json::json!({ "id" : "snd", "data" : data })
        },
        Self::Particle { kind, spread_x, spread_y, amount: cluster_amount, motion, motion_variation, colour, colour_variation, opacity, material, size, size_variation, roll, fade_colour } => {
            let mut data = json::json!({});
            if let Some((x, y, z)) = motion {
                data["x"] = json::Number::from_f64(*x).unwrap().into();
                data["y"] = json::Number::from_f64(*y).unwrap().into();
                data["z"] = json::Number::from_f64(*z).unwrap().into();
            }
            if let Some(motion_variation ) = motion_variation { data["motionVariation" ] = json::Number::from_f64  (*motion_variation    ).unwrap().into() }
            if let Some(colour           ) = colour           { data["rgb"             ] = json::Number::from_u128 (*colour as u128      ).unwrap().into() }
            if let Some(colour_variation ) = colour_variation { data["colorVariation"  ] = json::Number::from_f64  (*colour_variation    ).unwrap().into() }
            if let Some(opacity          ) = opacity          { data["opacity"         ] = json::Number::from_f64  (*opacity             ).unwrap().into() }
            if let Some(material         ) = material         { data["material"        ] = json::Value::String(material.clone()) }
            if let Some(size             ) = size             { data["size"            ] = json::Number::from_f64  (*size                ).unwrap().into() }
            if let Some(size_variation   ) = size_variation   { data["sizeVariation"   ] = json::Number::from_f64  (*size_variation      ).unwrap().into() }
            if let Some(roll             ) = roll             { data["roll"            ] = json::Number::from_f64  (*roll                ).unwrap().into() }
            if let Some(fade_colour      ) = fade_colour      { data["rgb_fade"        ] = json::Number::from_u128 (*fade_colour as u128 ).unwrap().into() }
            json::json!({ "id" : "part", "data" : {
                "particle" : kind,
                "cluster"  : { "amount" : cluster_amount, "horizontal" : spread_x, "vertical" : spread_y },
                "data"     : data
            } })
        },
        Self::Potion    { kind, dur, amp } => json::json!({ "id" : "pot",    "data" : { "pot" : kind, "dur" : dur, "amp" : amp } }),
        Self::Variable  { name, scope    } => json::json!({ "id" : "var",    "data" : { "name" : name, "scope" : scope.as_ref() } }),
        Self::Gamevalue { kind, target   } => json::json!({ "id" : "g_val",  "data" : { "type" : kind, "target" : target } }),
        Self::Item      { id, count, nbt } => json::json!({ "id" : "item",   "data" : { "item" : format!("{{Count:{}b,DF_NBT:3700,id:\"minecraft:{}\",tag:{}}}", count, id, nbt) }}),
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
