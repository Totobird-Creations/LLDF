use crate::types::String;


pub trait ToString {
    #[rustc_conversion_suggestion]
    fn to_string(&self) -> String;
}
