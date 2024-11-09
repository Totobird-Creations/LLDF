#![allow(dead_code)]


use serde::Deserialize as Deser;


pub type Items = Vec<Item>;

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id           : u32,
    pub name         : String,
    #[serde(rename = "displayName")]
    pub display_name : String,
    #[serde(rename = "stackSize")]
    pub stack_size   : u8,
    #[serde(rename = "enchantCategories")]
    pub enchants     : Option<Vec<String>>,
    #[serde(rename = "repairWith")]
    pub repair_with  : Option<Vec<String>>,
    #[serde(rename = "maxDurability")]
    pub durability   : Option<u32>
}


pub type Sounds = Vec<Sound>;

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct Sound {
    pub id   : u32,
    pub name : String,
}
