use super::clone::Clone;
use super::marker::Copy;


impl Clone for &str {
    fn clone(&self) -> Self { self }
}

impl Copy for &str {}
