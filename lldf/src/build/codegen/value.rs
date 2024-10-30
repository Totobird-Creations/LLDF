use std::error::Error;

use llvm_ir::Name;


#[derive(Debug, Clone, PartialEq)]
pub enum CodeValue {
    String(String),
    Text(String),
    Int(i64),
    Float(f64),
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

}


impl CodeValue {
    pub fn contains_line_var(&self, check : &str) -> bool { match (self) {
        Self::Variable { name, scope : VariableScope::Line } => { name == check },
        Self::Actiontag { variable : Some(variable), .. } => { variable.contains_line_var(check) },
        _ => false
    } }
    pub fn replace_line_var(&mut self, check : &str, with : &str) -> () { match (self) {
        Self::Variable { name, scope : VariableScope::Line } => { if (name == check) { *name = with.to_string() } },
        Self::Actiontag { variable : Some(variable), .. } => { variable.replace_line_var(check, with) },
        _ => { }
    } }
    pub fn as_actiontag(&self) -> Result<(String, Option<CodeValue>), Box<dyn Error>> { match (self) {
        Self::String(string) => Ok((string.clone(), None)),
        var @ Self::Variable { .. } => Ok((String::new(), Some(var.clone()))),
        _ => Err("Non string nor variable value used as tag".into())
    } }
}
