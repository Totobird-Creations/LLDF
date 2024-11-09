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

#[lldf_bind_proc::actiontag]
pub enum SkyEffect {
    None      = "None",
    Fog       = "Create fog",
    Darken    = "Darken sky",
    DarkenFog = "Both"
}

#[lldf_bind_proc::actiontag]
pub enum BarStyle {
    Solid = "Solid",
    Seg6  = "6 segments",
    Seg10 = "10 segments",
    Seg12 = "12 segments",
    Seg20 = "20 segments"
}

mod bar_colour;
#[cfg(any(not(feature = "en_us"), doc))]
#[doc(cfg(not(feature = "en_us")))]
pub use bar_colour::BarColour;
#[cfg(any(feature = "en_us", doc))]
#[doc(cfg(feature = "en_us"))]
pub use bar_colour::BarColour as BarColor;

#[lldf_bind_proc::actiontag]
pub enum AdvancementFrame {
    Task      = "Advancement",
    Goal      = "Goal",
    Challenge = "Challenge"
}

#[lldf_bind_proc::actiontag]
pub enum Gamemode {
    Survival  = "Survival",
    Adventure = "Adventure",
    Creative  = "Creative",
    Spectator = "Spectator"
}

#[lldf_bind_proc::actiontag]
pub enum Flag {
    Enable  = "Enable",
    Disable = "Disable"
}

mod name_colour;
#[cfg(any(not(feature = "en_us"), doc))]
#[doc(cfg(not(feature = "en_us")))]
pub use name_colour::NameColour;
#[cfg(any(feature = "en_us", doc))]
#[doc(cfg(feature = "en_us"))]
pub use name_colour::NameColour as NameColor;
