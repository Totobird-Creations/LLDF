mod name_colour {
    #[lldf_bind_proc::actiontag]
    pub enum NameColour {
        Black      = "Black",
        DarkBlue   = "Dark blue",
        DarkGreen  = "Dark green",
        DarkAqua   = "Dark aqua",
        DarkRed    = "Dark red",
        DarkPurple = "Dark purple",
        Gold       = "Gold",
        #[cfg(any(not(feature = "en_us"), doc))]
        #[doc(cfg(not(feature = "en_us")))]
        Grey       = "Gray",
        #[cfg(any(feature = "en_us", doc))]
        #[doc(cfg(feature = "en_us"))]
        Gray       = "Gray",
        #[cfg(any(not(feature = "en_us"), doc))]
        #[doc(cfg(not(feature = "en_us")))]
        DarkGrey   = "Dark gray",
        #[cfg(any(feature = "en_us", doc))]
        #[doc(cfg(feature = "en_us"))]
        DarkGray   = "Dark gray",
        Blue       = "Blue",
        Green      = "Green",
        Aqua       = "Aqua",
        Red        = "Red",
        Purple     = "Light purple",
        Yellow     = "Yellow",
        White      = "White",
        None       = "None"
    }
}

#[cfg(any(not(feature = "en_us"), doc))]
#[doc(cfg(not(feature = "en_us")))]
pub use name_colour::NameColour;
#[cfg(any(feature = "en_us", doc))]
#[doc(cfg(feature = "en_us"))]
pub use name_colour::NameColour as NameColor;
