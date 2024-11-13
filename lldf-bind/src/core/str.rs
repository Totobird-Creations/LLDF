use crate::types::String;
use crate::core::clone::Clone;
use crate::core::convert::From;
use crate::core::marker::Copy;
use crate::core::string::ToString;


impl Clone for &str {
    fn clone(&self) -> Self { self }
}

impl Copy for &str {}

impl ToString for &str {
    #[inline(always)]
    fn to_string(&self) -> String { From::<&str>::from(self) }
}
