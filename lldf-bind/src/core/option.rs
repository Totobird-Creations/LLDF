use super::clone::Clone;
use super::macros::derive;
use super::marker::Copy;


#[derive(Clone, Copy)]
#[rustc_diagnostic_item = "Option"]
#[lang = "Option"]
pub enum Option<T> {
    #[lang = "None"]
    None,
    #[lang = "Some"]
    Some(T)
}
