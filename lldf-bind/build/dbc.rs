#![allow(dead_code)]


use serde::Deserialize as Deser;


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBC {
    pub codeblocks            : Vec<DBCCodeBlock>,
    pub actions               : Vec<DBCAction>,
    #[serde(rename = "gameValueCategories")]
    pub game_value_categories : Vec<DBCGameValueCategory>,
    #[serde(rename = "gameValues")]
    pub game_values           : Vec<DBCGameValue>,
    #[serde(rename = "particleCategories")]
    pub particle_categories   : Vec<DBCParticleCategory>,
    pub particles             : Vec<DBCParticle>,
    #[serde(rename = "soundCategories")]
    pub sound_categories      : Vec<DBCSoundCategory>,
    pub sounds                : Vec<DBCSound>,
    pub potions               : Vec<DBCPotion>,
    pub cosmetics             : Vec<DBCCosmetic>,
    pub shops                 : Vec<DBCShop>
}

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCCodeBlock {
    pub name       : String,
    pub identifier : String,
    pub item       : DBCCodeBlockItem
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCCodeBlockItem {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCAction {
    pub name              : String,
    #[serde(rename = "codeblockName")]
    pub code_block_name   : String,
    pub tags              : Vec<DBCActionTag>,
    pub aliases           : Vec<String>,
    pub icon              : DBCActionIcon,
    #[serde(rename = "subActionBlocks")]
    pub sub_action_blocks : Option<Vec<String>>
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCActionTag {
    pub name           : String,
    pub options        : Vec<DBCActionTagOption>,
    #[serde(rename = "defaultOption")]
    pub default_option : String,
    pub slot           : u8
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCActionTagOption {
    pub name    : String,
    pub icon    : DBCActionTagOptionIcon,
    pub aliases : Vec<String>
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCActionTagOptionIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCActionIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String,
    #[serde(default)]
    pub cancellable             : bool,
    #[serde(rename = "cancelledAutomatically")]
    #[serde(default)]
    pub cancelled_automatically : bool,
    pub tags                    : Option<u8>,
    #[serde(default)]
    pub arguments               : Vec<DBCActionIconArgument>,
    #[serde(rename = "returnValues")]
    #[serde(default)]
    pub return_values           : Vec<DBCActionIconReturnValue>
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCActionIconArgument {
    #[serde(rename = "type")]
    pub typ         : Option<DBCValueType>,
    pub plural      : Option<bool>,
    pub optional    : Option<bool>,
    pub description : Option<Vec<String>>,
    pub notes       : Option<Vec<Vec<String>>>,
    pub text        : Option<String>
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCActionIconReturnValue {
    #[serde(rename = "type")]
    pub typ         : Option<DBCValueType>,
    pub description : Option<Vec<String>>,
    pub text        : Option<String>
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCGameValueCategory {
    pub identifier : String,
    #[serde(rename = "guiSlot")]
    pub gui_slot   : u8,
    pub icon       : DBCGameValueCategoryIcon
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCGameValueCategoryIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCGameValue {
    pub aliases  : Vec<String>,
    pub category : String,
    pub icon     : DBCGameValueIcon
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCGameValueIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String,
    #[serde(rename = "returnType")]
    pub return_type             : DBCValueType,
    #[serde(rename = "returnDescription")]
    pub return_description      : Vec<String>
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCParticleCategory {
    pub particle : String,
    pub icon     : DBCParticleCategoryIcon,
    pub category : Option<String>,
    pub fields   : Vec<String>
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCParticleCategoryIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCParticle {
    pub particle : String,
    pub icon     : DBCParticleIcon,
    pub category : Option<String>,
    pub fields   : Vec<DBCParticleField>
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCParticleIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}
#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub enum DBCParticleField {
    Motion,
    #[serde(rename = "Motion Variation")]
    MotionVariation,
    #[serde(rename = "Color")]
    Colour,
    #[serde(rename = "Color Variation")]
    ColourVariation,
    Material,
    Size,
    #[serde(rename = "Size Variation")]
    SizeVariation,
    Roll,
    #[serde(rename = "Fade Color")]
    FadeColour
}
impl DBCParticleField {
    pub fn to_type_name(&self) -> &'static str { match (self) {
        Self::Motion          => "Vector",
        Self::MotionVariation => "Float",
        Self::Colour          => "RGB",
        Self::ColourVariation => "Float",
        Self::Material        => "&str",
        Self::Size            => "Float",
        Self::SizeVariation   => "Float",
        Self::Roll            => "Float",
        Self::FadeColour      => "RGB"
    } }
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCSoundCategory {
    pub identifier         : String,
    pub icon               : DBCSoundCategoryIcon,
    #[serde(rename = "hasSubCategories")]
    pub has_sub_categories : bool
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCSoundCategoryIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCSound {
    pub sound : String,
    pub icon  : DBCSoundIcon
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCSoundIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCPotion {
    pub potion : String,
    pub icon   : DBCPotionIcon
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCPotionIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCCosmetic {
    pub id       : String,
    pub icon     : DBCCosmeticIcon,
    pub name     : String,
    pub category : DBCCosmeticCategory
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCCosmeticIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCCosmeticCategory {}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCShop {
    pub id           : String,
    pub slot         : Option<u8>,
    pub name         : Option<String>,
    pub icon         : Option<DBCShopIcon>,
    pub purchasables : Vec<DBCShopPurchasable>
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCShopIcon {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCShopPurchasable {
    pub item              : DBCShopPurchasableItem,
    pub id                : Option<String>,
    pub price             : Option<u16>,
    #[serde(rename = "currencyType")]
    pub currency_type     : Option<DBCCurrencyType>,
    #[serde(rename = "oneTimePurchase")]
    #[serde(default)]
    pub one_time_purchase : bool
}
#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCShopPurchasableItem {
    pub material                : String,
    pub name                    : String,
    pub head                    : Option<String>,
    #[serde(rename = "color")]
    pub colour                  : Option<DBCColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecated_note         : Vec<String>,
    pub description             : Vec<String>,
    pub example                 : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with              : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info         : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank           : DBCRank,
    #[serde(rename = "requireTokens")]
    pub require_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub require_rank_and_tokens : bool,
    pub advanced                : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item             : String
}


#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub enum DBCRank {
    #[serde(rename = "")]
    None,
    Noble,
    Emperor,
    Mythic,
    Overlord,
    Dev
}
impl DBCRank {
    pub fn to_rank_feature_gate(&self) -> &'static str { match (self) {
        Self::None     => "#[cfg(any(feature = \"rank_none\", feature = \"rank_noble\", feature = \"rank_emperor\", feature = \"rank_mythic\", feature = \"rank_overlord\", feature = \"rank_dev\"))]",
        Self::Noble    => "#[cfg(any(feature = \"rank_noble\", feature = \"rank_emperor\", feature = \"rank_mythic\", feature = \"rank_overlord\", feature = \"rank_dev\"))]",
        Self::Emperor  => "#[cfg(any(feature = \"rank_emperor\", feature = \"rank_mythic\", feature = \"rank_overlord\", feature = \"rank_dev\"))]",
        Self::Mythic   => "#[cfg(any(feature = \"rank_mythic\", feature = \"rank_overlord\", feature = \"rank_dev\"))]",
        Self::Overlord => "#[cfg(any(feature = \"rank_overlord\", feature = \"rank_dev\"))]",
        Self::Dev      => "#[cfg(feature = \"rank_dev\")]"
    } }
    pub fn to_rank_doc_gate(&self) -> Option<&'static str> { match (self) {
        Self::None     => None,
        Self::Noble    => Some("#[doc(cfg(feature = \"rank_noble\"))]"),
        Self::Emperor  => Some("#[doc(cfg(feature = \"rank_emperor\"))]"),
        Self::Mythic   => Some("#[doc(cfg(feature = \"rank_mythic\"))]"),
        Self::Overlord => Some("#[doc(cfg(feature = \"rank_overlord\"))]"),
        Self::Dev      => Some("#[doc(cfg(feature = \"rank_dev\"))]")
    } }
}

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct DBCColour {
    #[serde(rename = "red")]
    pub r : u8,
    #[serde(rename = "green")]
    pub g : u8,
    #[serde(rename = "blue")]
    pub b : u8
}

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub enum DBCCurrencyType {
    Token,
    Ticket
}
#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub enum DBCValueType {

    #[serde(rename = "TEXT")]
    String,
    #[serde(rename = "COMPONENT")]
    Text,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "LOCATION")]
    Location,
    #[serde(rename = "VECTOR")]
    Vector,
    #[serde(rename = "SOUND")]
    Sound,
    #[serde(rename = "PARTICLE")]
    Particle,
    #[serde(rename = "POTION")]
    Potion,
    #[serde(rename = "ITEM")]
    Item,

    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "DICT")]
    Dict,

    #[serde(rename = "VARIABLE")]
    Variable,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ANY_TYPE")]
    Any,

    #[serde(rename = "BLOCK")]
    Block,
    #[serde(rename = "BLOCK_TAG")]
    BlockState,
    #[serde(rename = "PROJECTILE")]
    Projectile,
    #[serde(rename = "SPAWN_EGG")]
    SpawnEgg,
    #[serde(rename = "VEHICLE")]
    Vehicle,
    #[serde(rename = "ENTITY_TYPE")]
    EntityType

}
