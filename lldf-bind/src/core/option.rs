use super::clone::Clone;
use super::macros::derive;
use super::marker::Copy;
use super::string::ToString;
use super::convert::From;

use crate::types::String;


#[derive(Clone, Copy)]
#[rustc_diagnostic_item = "Option"]
#[lang = "Option"]
pub enum Option<T> {
    #[lang = "None"]
    None,
    #[lang = "Some"]
    Some(T)
}


impl<T : ToString> ToString for Option<T> {
    #[inline(always)]
    fn to_string(&self) -> String {
        match (self) {
            Self::None        => String::from("None"),
            Self::Some(inner) => String::from("Some(") + inner.to_string() + ")"
        }
    }
}
