#[lldf_bind_proc::actiontag]
pub enum Toggle {
    Enable  = "Enable",
    Disable = "Disable"
}

#[lldf_bind_proc::actiontag]
pub enum Flag {
    True  = "True",
    False = "False"
}

#[lldf_bind_proc::actiontag]
pub enum Visible {
    Hide = "Hide",
    Show = "Show"
}



// `PLAYER_ACTION`


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
pub enum PotionParticles {
    Regular = "Regular",
    Ambient = "Ambient",
    None    = "None"
}

mod name_colour;
#[cfg(any(not(feature = "en_us"), doc))]
#[doc(cfg(not(feature = "en_us")))]
pub use name_colour::NameColour;
#[cfg(any(feature = "en_us", doc))]
#[doc(cfg(feature = "en_us"))]
pub use name_colour::NameColour as NameColor;



// `SET_VARIABLE` / `Item Manipulation`


#[lldf_bind_proc::actiontag]
pub enum ItemRarity {
    Common   = "Common",
    Uncommon = "Uncommon",
    Rare     = "Rare",
    Epic     = "Epic"
}

#[lldf_bind_proc::actiontag]
pub enum ItemGlowing {
    Enable  = "Enable",
    Disable = "Disable",
    Default = "Default"
}

#[lldf_bind_proc::actiontag]
pub enum TrimPattern {
    Bolt      = "Bolt",
    Coast     = "Coast",
    Dune      = "Dune",
    Eye       = "Eye",
    Flow      = "Flow",
    Rib       = "Rib",
    Sentry    = "Sentry",
    Snout     = "Snout",
    Spire     = "Spire",
    Tide      = "Tide",
    Vex       = "Vex",
    Ward      = "Ward",
    Wayfinder = "Wayfinder",
    Shaper    = "Shaper",
    Silence   = "Silence",
    Raiser    = "Raiser",
    Host      = "Host",
    Wild      = "Wild"
}

#[lldf_bind_proc::actiontag]
pub enum TrimMaterial {
    Amethyst  = "Amethyst",
    Copper    = "Copper",
    Diamond   = "Diamond",
    Emerald   = "Emerald",
    Gold      = "Gold",
    Iron      = "Iron",
    Lapis     = "Lapis lazuli",
    Netherite = "Netherite",
    Quartz    = "Quartz",
    Redstone  = "Redstone"
}

#[lldf_bind_proc::actiontag]
pub enum ItemAttr {
    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    Armour = "Armor",
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    Armor = "Armor",
    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    ArmourToughness = "Armor toughness",
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    ArmorToughness               = "Armor toughness",
    AttackDamage                 = "Attack damage",
    AttackKnockback              = "Attack knockback",
    AttackSpeed                  = "Attack speed",
    BurningTime                  = "Burning time",
    ExplosionKnockbackResistance = "Explosion knockback resistance",
    FallDamageMultiplier         = "Fall damage multiplier",
    FlyingSpeed                  = "Flying speed",
    FollowRange                  = "Follow range",
    Gravity                      = "Gravity",
    JumpStrength                 = "Jump strength",
    KnockbackResistance          = "Knockback resistance",
    Luck                         = "Luck",
    MaximumAbsorptionHealth      = "Maximum absorption health",
    MaximumHealth                = "Maximum health",
    MovementEfficiency           = "Movement efficiency",
    WalkingSpeed                 = "Walking speed",
    OxygenBonus                  = "Oxygen bonus",
    SafeFallDistance             = "Safe fall distance",
    Scale                        = "Scale",
    StepHeight                   = "Step height",
    WaterMovementEfficiency      = "Water movement efficiency",
    BlockBreakSpeed              = "Block break speed",
    BlockInteractionRange        = "Block interaction range",
    EntityInteractionRange       = "Entity interaction range",
    MiningEfficiency             = "Mining efficiency",
    SneakingSpeed                = "Sneaking speed",
    SubmergedMiningSpeed         = "Submerged mining speed",
    SweepingDamageRatio          = "Sweeping damage ratio",
    ZombieSpawnReinforcements    = "Zombie spawn reinforcements"
}

#[lldf_bind_proc::actiontag]
pub enum ItemAttrOp {
    Add             = "Add number",
    AddPercent      = "Add percentage to base",
    MultiplyPercent = "Multiply modifier by percentage"
}

#[lldf_bind_proc::actiontag]
pub enum ItemAttrSlot {
    Any      = "Any",
    MainHand = "Main hand",
    OffHand  = "Off hand",
    Head     = "Head",
    Chest    = "Body",
    Legs     = "Legs",
    Feet     = "Feet"
}



// `SET_VARIABLE` / `Vector Manipulation`


#[lldf_bind_proc::actiontag]
pub enum Direction {
    North = "north",
    East  = "east",
    South = "south",
    West  = "west",
    Up    = "up",
    Down  = "down"
}
