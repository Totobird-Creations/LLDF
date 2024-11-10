#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "add"]
    fn add(self, rhs : Rhs) -> Self::Output;
}

#[lang = "sub"]
pub trait Sub<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "sub"]
    fn sub(self, rhs : Rhs) -> Self::Output;
}

#[lang = "mul"]
pub trait Mul<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "mul"]
    fn mul(self, rhs : Rhs) -> Self::Output;
}

#[lang = "neg"]
pub trait Neg {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "beg"]
    fn neg(self) -> Self::Output;
}
