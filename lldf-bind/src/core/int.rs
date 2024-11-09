use super::clone::Clone;
use super::cmp::{ PartialEq, PartialOrd, Ordering };
use super::marker::Copy;
use super::ops::{ Add, Sub, Neg, Not };
use super::option::Option;

impl Clone for bool  { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for u8    { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for i8    { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for u16   { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for i16   { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for u32   { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for i32   { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for u64   { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for i64   { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for u128  { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for i128  { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for usize { #[inline(never)] fn clone(&self) -> Self { *self } }
impl Clone for isize { #[inline(never)] fn clone(&self) -> Self { *self } }

impl Copy for bool  {}
impl Copy for u8    {}
impl Copy for i8    {}
impl Copy for u16   {}
impl Copy for i16   {}
impl Copy for u32   {}
impl Copy for i32   {}
impl Copy for u64   {}
impl Copy for i64   {}
impl Copy for u128  {}
impl Copy for i128  {}
impl Copy for usize {}
impl Copy for isize {}

impl Add for u8 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i8 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u16 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i16 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u32 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i32 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u64 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i64 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for u128 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for i128 { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for usize { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Add for isize { type Output = Self; #[inline(never)] fn add(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }

// TODO: Bounds check for unsigned integers.
impl Sub for u8 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for i8 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for u16 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for i16 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for u32 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for i32 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for u64 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for i64 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for u128 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for i128 { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for usize { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Sub for isize { type Output = Self; #[inline(never)] fn sub(self, _ : Self) -> Self::Output { loop { /* compiler built-in */ } } }

impl Neg for i8 { type Output = Self; #[inline(never)] fn neg(self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Neg for i16 { type Output = Self; #[inline(never)] fn neg(self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Neg for i32 { type Output = Self; #[inline(never)] fn neg(self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Neg for i64 { type Output = Self; #[inline(never)] fn neg(self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Neg for i128 { type Output = Self; #[inline(never)] fn neg(self) -> Self::Output { loop { /* compiler built-in */ } } }
impl Neg for isize { type Output = Self; #[inline(never)] fn neg(self) -> Self::Output { loop { /* compiler built-in */ } } }

impl Not for bool { type Output = Self; #[inline(never)] fn not(self) -> Self::Output { loop { /* compiler built-in */ } } }

impl PartialEq for bool  { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for u8    { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for i8    { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for u16   { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for i16   { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for u32   { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for i32   { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for u64   { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for i64   { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for u128  { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for i128  { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for usize { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }
impl PartialEq for isize { #[inline(never)] fn eq(&self, _ : &Self) -> bool { loop { /* compiler built-in */ } } }

impl PartialOrd for bool  { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for u8    { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for i8    { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for u16   { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for i16   { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for u32   { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for i32   { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for u64   { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for i64   { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for u128  { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for i128  { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for usize { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
impl PartialOrd for isize { #[inline(never)] fn partial_cmp(&self, _ : &Self) -> Option<Ordering> { loop { /* compiler built-in */ } } }
