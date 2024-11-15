#![feature(
    exit_status_error,
    decl_macro,
    iter_intersperse,
    iter_map_windows,
    str_from_utf16_endian,
    iter_array_chunks,
    let_chains,
    slice_as_chunks
)]


pub mod cli;

pub mod build;


pub const MODULE_NAME : &'static str = module_path!();
