use super::clone::Clone;

impl Clone for u8 { fn clone(&self) -> Self { *self } }
impl Clone for i8 { fn clone(&self) -> Self { *self } }
impl Clone for u16 { fn clone(&self) -> Self { *self } }
impl Clone for i16 { fn clone(&self) -> Self { *self } }
impl Clone for u32 { fn clone(&self) -> Self { *self } }
impl Clone for i32 { fn clone(&self) -> Self { *self } }
impl Clone for u64 { fn clone(&self) -> Self { *self } }
impl Clone for i64 { fn clone(&self) -> Self { *self } }
impl Clone for u128 { fn clone(&self) -> Self { *self } }
impl Clone for i128 { fn clone(&self) -> Self { *self } }
impl Clone for usize { fn clone(&self) -> Self { *self } }
impl Clone for isize { fn clone(&self) -> Self { *self } }


use super::marker::Copy;

impl Copy for u8 {}
impl Copy for i8 {}
impl Copy for u16 {}
impl Copy for i16 {}
impl Copy for u32 {}
impl Copy for i32 {}
impl Copy for u64 {}
impl Copy for i64 {}
impl Copy for u128 {}
impl Copy for i128 {}
impl Copy for usize {}
impl Copy for isize {}


use super::ops::Add;

impl Add for u8 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i8 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u16 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i16 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u32 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i32 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u64 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i64 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u128 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i128 { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for usize { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for isize { type Output = Self; fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
