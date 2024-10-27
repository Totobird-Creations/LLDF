use super::clone::Clone;

impl Clone for f32 {
    fn clone(&self) -> Self { *self }
}

impl Clone for f64 {
    fn clone(&self) -> Self { *self }
}


use super::marker::Copy;

impl Copy for f32 {}

impl Copy for f64 {}
