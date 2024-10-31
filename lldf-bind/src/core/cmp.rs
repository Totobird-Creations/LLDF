use super::marker::Sized;
use super::option::Option;


#[lang = "eq"]
#[rustc_diagnostic_item = "PartialEq"]
pub trait PartialEq<Rhs: ?Sized = Self> {

    #[must_use]
    #[rustc_diagnostic_item = "cmp_partialeq_eq"]
    fn eq(&self, other: &Rhs) -> bool;

    #[inline]
    #[must_use]
    #[rustc_diagnostic_item = "cmp_partialeq_ne"]
    fn ne(&self, other : &Rhs) -> bool {
        ! self.eq(other)
    }

}

#[rustc_builtin_macro]
pub macro PartialEq( $item:item ) {
/* compiler built-in */
}


#[lang = "partial_ord"]
#[rustc_diagnostic_item = "PartialOrd"]
pub trait PartialOrd<Rhs : ?Sized = Self> : PartialEq<Rhs> {

    #[must_use]
    #[rustc_diagnostic_item = "cmp_partialord_cmp"]
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    #[inline(always)]
    #[must_use]
    #[rustc_diagnostic_item = "cmp_partialord_lt"]
    fn lt(&self, other: &Rhs) -> bool {
        if let Option::Some(Ordering::Less) = self.partial_cmp(other) { true } else { false }
    }

    #[inline(always)]
    #[must_use]
    #[rustc_diagnostic_item = "cmp_partialord_le"]
    fn le(&self, other: &Rhs) -> bool {
        if let Option::Some(Ordering::Less | Ordering::Equal) = self.partial_cmp(other) { true } else { false }
    }

    #[inline(always)]
    #[must_use]
    #[rustc_diagnostic_item = "cmp_partialord_gt"]
    fn gt(&self, other: &Rhs) -> bool {
        if let Option::Some(Ordering::Greater) = self.partial_cmp(other) { true } else { false }
    }

    #[inline(always)]
    #[must_use]
    #[rustc_diagnostic_item = "cmp_partialord_ge"]
    fn ge(&self, other: &Rhs) -> bool {
        if let Option::Some(Ordering::Greater | Ordering::Equal) = self.partial_cmp(other) { true } else { false }
    }

}

#[rustc_builtin_macro]
pub macro PartialOrd( $item:item ) {
    /* compiler built-in */
}


#[lang = "Ordering"]
#[repr(i8)]
pub enum Ordering {
    Less    = -1,
    Equal   = 0,
    Greater = 1,
}
