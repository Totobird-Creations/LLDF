use super::marker::Copy;
use super::cmp::Ordering;


#[rustc_intrinsic]
#[rustc_intrinsic_must_be_overridden]
pub const fn three_way_compare<T : Copy>(_lhs : T, _rhs : T) -> Ordering { loop { } }


#[rustc_intrinsic]
#[rustc_intrinsic_must_be_overridden]
pub const unsafe fn unreachable() -> ! { loop { } }
