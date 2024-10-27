use super::clone::Clone;

impl Clone for &str {
    fn clone(&self) -> Self { self }
}


use super::marker::Copy;

impl Copy for &str {}
