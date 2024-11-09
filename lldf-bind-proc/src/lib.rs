#![feature(proc_macro_diagnostic)]


use proc_macro::TokenStream;
use proc_macro::Span;
use syn::parse;
use syn::{ FnArg, Ident, Item, ItemFn, LitStr, Pat, Path, PathArguments, PathSegment, PatType, PatWild, Type, TypePath };
use syn::punctuated::Punctuated;
use quote::quote;


include!("compile_warning.rs");

include!("event.rs");

include!("dfdoc.rs");

include!("actiontag.rs");
