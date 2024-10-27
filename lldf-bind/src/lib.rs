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
    transparent_unions
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
// Error if the number of enabled plot size features is not exactly one.
feature_require_one!{ "plot size" => "plot_50x", "plot_100x", "plot_300x", "plot_1000x" }

// Warn if dev rank is enabled.
#[cfg(feature = "rank_dev")]
compile_warning!("Developer server rank enabled. You probably shouldn't be using this.");

// Warn if public internals is enabled.
#[cfg(feature = "internals")]
compile_warning!("Public internals enabled. Here be dragons.");

// Warn if target family is not wasm.
#[cfg(not(target_family = "wasm"))]
compile_warning!("Target family is not wasm. It probably should be.");/*  */


pub mod core;
use crate::core::prelude::*;


#[cfg(feature = "internals")]
#[doc(cfg(feature = "internals"))]
pub mod bind;
#[cfg(not(feature = "internals"))]
mod bind;

pub mod types;


/// The essentials.
pub mod prelude {
    pub use crate::core::prelude::*;

    pub use lldf_bind_proc::event;

    pub use crate::types::value::DFValue;
    pub use crate::types::value::string::String;
    pub use crate::types::value::number::{ UInt, Int, Float };
    pub use crate::types::value::text::Text;
    pub use crate::types::value::location::Location;
    pub use crate::types::value::vector::Vector;
    pub use crate::types::value::sound::Sound;
    pub use crate::types::value::particle::Particle;
    pub use crate::types::value::potion::Potion;
    pub use crate::types::value::item::Item;
    pub use crate::types::value::colour::RGB;
    pub use crate::types::value::list::List;
    pub use crate::types::value::dict::Dict;

    pub use crate::types::sel::DFSel;
    pub use crate::types::sel::player::PlayerSel;
    pub use crate::types::sel::entity::EntitySel;

    pub use crate::types::game::Game;
}
