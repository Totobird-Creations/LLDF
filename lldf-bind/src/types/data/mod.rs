#[lldf_bind_proc::actiontag]
pub enum SoundChannel {
    Master   = "Master",
    Music    = "Music",
    Jukebox  = "Jukebox/Note Blocks",
    Weather  = "Weather",
    Blocks   = "Blocks",
    Hostile  = "Hostile Creatures",
    Friendly = "Friendly Creatures",
    Players  = "Players",
    Ambient  = "Ambient/Environment",
    Voice    = "Voice/Speech"
}

mod name_colour;
pub use name_colour::*;
