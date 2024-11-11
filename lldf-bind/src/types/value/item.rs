use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct Item {
    _opaque_type : u8
}

impl Item {

    // TODO: mat_id

    // TODO: mat_name

    // TODO: without_nbt

    // TODO: name

    // TODO: with_name

    // TODO: lore

    // TODO: lore_line

    // TODO: with_lore

    // TODO: with_lore_line

    // TODO: count

    // TODO: with_count

    // TODO: stack_size

    // TODO: with_stack_size

    // TODO: dura

    // TODO: with_dura

    // TODO: with_max_dura

    // TODO: with_unbreakable

    // TODO: enchants

    // TODO: with_enchant

    // TODO: without_enchant

    // TODO: without_any_enchants

    // TODO: head_name

    // TODO: head_uuid

    // TODO: with_head_name

    // TODO: with_head_uuid

    // TODO: with_head_texture

    // TODO: book_pages

    // TODO: book_page

    // TODO: with_book_pages

    // TODO: with_book_page

    // TODO: tag

    // TODO: tags

    // TODO: with_tag

    // TODO: without_tag

    // TODO: without_tags

    // TODO: with_modeldata

    // TODO: potions

    // TODO: with_potions

    // TODO: with_hideflag_trim

    // TODO: with_hideflag_colour

    // TODO: with_hideflag_enchants

    // TODO: with_hideflag_attributes

    // TODO: with_hideflag_unbreakable

    // TODO: with_hideflag_breakables

    // TODO: with_hideflag_placeables

    // TODO: with_hideflag_potions

    // TODO: with_hideflag_others

    // TODO: placeables

    // TODO: with_placeables

    // TODO: breakables

    // TODO: with_breakables

    // TODO: rarity

    // TODO: compass_target

    // TODO: with_compass_target

    // TODO: nutrition

    // TODO: saturation

    // TODO: eating_time

    // TODO: with_food

    // TODO: with_tool

    // TODO: with_tool_rules

    // TODO: with_tooltip

    // TODO: with_trim

    // TODO: without_trim

    // TODO: colour

    // TODO: with_colour

    // TODO: attribute

    // TODO: with_attribute

    // TODO: with_map_texture

    // TODO: from_block_tag

    // TODO: from_item_tag

}

unsafe impl DFValue for Item {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Item ) -> DFOpaqueValue;

}


include!("../../bind/item.rs");
