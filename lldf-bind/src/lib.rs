#![feature(
    no_core,
    lang_items,
    arbitrary_self_types,
    rustc_attrs,
    decl_macro,
    auto_traits,
    fundamental,
    allow_internal_unstable,
    intrinsics,
    doc_cfg,
    transparent_unions,
    specialization,
    trivial_bounds
)]

#![no_main]
#![no_std]
#![no_core]


macro feature_require_one {
    { $name:tt => $( $ranks:tt ),+ } => {
        #[cfg(not(any( $( feature = $ranks ),+ )))]
        compile_error!(concat!("No ", $name, " feature enabled. Enable one of ", $( "`", $ranks, "`", )+ "."));
        crate::feature_require_one!{ @mutex $name => $( $ranks ),+ }
    },
    { @mutex $name:tt => $_:tt } => { },
    { @mutex $name:tt => $rank_0:tt $(, $ranks:tt )+ } => {
        #[cfg(all(feature = $rank_0, any( $( feature = $ranks ),* )))]
        compile_error!(concat!("Multiple ", $name, " features enabled. Disable all but one."));
        crate::feature_require_one!{ @mutex $name => $( $ranks ),+ }
    }
}
// Error if the number of enabled server rank features is not exactly one.
feature_require_one!{ "server rank" => "rank_none", "rank_noble", "rank_emperor", "rank_mythic", "rank_overlord", "rank_dev" }
// Error if the number of enabled game version features is not exactly one.
feature_require_one!{ "game version" => "mc_1_21_1" }

// Warn if dev rank is enabled.
#[cfg(feature = "rank_dev")]
compile_warning!("Developer server rank enabled. You probably shouldn't be using this.");

// Warn if target family is not wasm.
#[cfg(not(target_family = "wasm"))]
compile_warning!("Target family is not wasm. It probably should be.");/*  */


pub mod core;
use crate::core::prelude::*;

mod bind;

pub mod types;


/// The essentials.
pub mod prelude {
    pub use crate::core;
    pub use crate::core::prelude::*;

    pub use lldf_bind_proc::event;

    pub use crate::types::*;
}





#[doc(hidden)]
pub mod __private;
