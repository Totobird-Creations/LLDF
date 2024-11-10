use crate::types::String;


pub trait ToString {
    #[rustc_conversion_suggestion]
    fn to_string(&self) -> String;
}

pub trait FromStringUnchecked {
    #[rustc_conversion_suggestion]
    unsafe fn from_string_unchecked(from : String) -> Self;
}
