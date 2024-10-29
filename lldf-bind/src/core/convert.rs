use super::marker::Sized;


#[rustc_diagnostic_item = "Into"]
pub trait Into<T> : Sized {
    #[must_use]
    fn into(self) -> T;
}
impl<T> Into<T> for T {
    fn into(self) -> T { self }
}


#[rustc_diagnostic_item = "From"]
pub trait From<T> : Sized {
    #[must_use]
    fn from(value : T) -> Self;
}


pub trait AsRef<T : ?Sized> {
    fn as_ref(&self) -> &T;
}
impl<T> AsRef<T> for T {
    fn as_ref(&self) -> &T { self }
}
impl<T> AsRef<T> for &T {
    fn as_ref(&self) -> &T { self }
}
impl<T> AsRef<T> for &mut T {
    fn as_ref(&self) -> &T { self }
}

pub trait AsMut<T : ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}
impl<T> AsMut<T> for T {
    fn as_mut(&mut self) -> &mut T { self }
}
impl<T> AsMut<T> for &mut T {
    fn as_mut(&mut self) -> &mut T { self }
}
