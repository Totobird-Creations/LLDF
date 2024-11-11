use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct Item {
    _opaque_type : u8
}

impl Item {

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemType { ReturnValueType = ItemID })]
    #[inline(always)]
    pub fn mat_id(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetItemType_ReturnValueType_ItemID(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemType { ReturnValueType = ItemName })]
    #[inline(always)]
    pub fn mat_name(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetItemType_ReturnValueType_ItemName(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemType { ReturnValueType = Item })]
    #[inline(always)]
    pub fn without_nbt(&self) -> Item { unsafe {
        DF_ACTION__SetVariable_GetItemType_ReturnValueType_Item(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemName)]
    #[inline(always)]
    pub fn name(&self) -> Text { unsafe {
        DF_ACTION__SetVariable_GetItemName(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemName)]
    #[inline(always)]
    pub fn with_name<T : Into<Text>>(&self, name : T) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemName(self.clone(), name.into())
    } }

    // TODO: lore

    // TODO: lore_line

    // TODO: with_lore

    // TODO: with_lore_line

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemAmount)]
    #[inline(always)]
    pub fn count(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_GetItemAmount(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemAmount)]
    #[inline(always)]
    pub fn with_count<U : Into<UInt>>(&self, count : U) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemAmount(self.clone(), count.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetMaxAmount)]
    #[inline(always)]
    pub fn stack_size(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_GetMaxAmount(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetMaxAmount)]
    #[inline(always)]
    pub fn with_stack_size<U : Into<UInt>>(&self, count : U) -> Item { unsafe {
        DF_ACTION__SetVariable_SetMaxAmount(self.clone(), count.into())
    } }

    // TODO: dura

    // TODO: with_dura

    // TODO: with_max_dura

    // TODO: with_unbreakable

    // TODO: enchants

    // TODO: with_enchant

    // TODO: without_enchant

    // TODO: without_any_enchants

    #[lldf_bind_proc::dfdoc(SetVariable/GetHeadOwner { TextValue = OwnerName })]
    /// ##### Unsafe
    /// - **May cause large plot CPU usage spikes, causing plot to lagslay.**
    /// - **May return a non-[`String`](String), `0` value.**
    #[inline(always)]
    pub unsafe fn head_name(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetHeadOwner_TextValue_OwnerName(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetHeadOwner { TextValue = OwnerUUID })]
    /// ##### Unsafe
    /// - **May cause large plot CPU usage spikes, causing plot to lagslay.**
    /// - **May return a non-[`String`](String), `0` value.**
    #[inline(always)]
    pub unsafe fn head_uuid(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetHeadOwner_TextValue_OwnerUUID(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetHeadTexture)]
    /// ##### Unsafe
    /// - **May cause large plot CPU usage spikes, causing plot to lagslay.**
    #[inline(always)]
    pub unsafe fn with_head_name(&self, name : String) -> Item { unsafe {
        DF_ACTION__SetVariable_SetHeadTexture(self.clone(), name)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetHeadTexture)]
    /// ##### Unsafe
    /// - **May cause large plot CPU usage spikes, causing plot to lagslay.**
    #[inline(always)]
    pub unsafe fn with_head_uuid(&self, uuid : UUID) -> Item { unsafe {
        DF_ACTION__SetVariable_SetHeadTexture(self.clone(), uuid.to_string())
    } }

    // TODO: with_head_texture

    // TODO: book_pages

    // TODO: book_page

    // TODO: with_book_pages

    // TODO: with_book_page

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemTag)]
    #[inline(always)]
    pub fn tag(&self, key : String) -> String { unsafe {
        DF_ACTION__SetVariable_String(DF_ACTION__SetVariable_GetItemTag(self.clone(), key))
    } }

    /// Returns a list of existing tag keys.
    #[inline(always)]
    pub fn tags(&self) -> List<String> { unsafe {
        let dict = DF_ACTION__SetVariable_GetAllItemTags(self.clone());
        dict.keys()
    } }

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

    #[lldf_bind_proc::dfdoc(SetVariable/GetLodestoneLoc)]
    #[inline(always)]
    pub fn compass_target(&self) -> Location { unsafe {
        DF_ACTION__SetVariable_GetLodestoneLoc(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetLodestoneLoc)]
    #[inline(always)]
    pub fn with_compass_target(&self, location : Location) -> Item { unsafe {
        DF_ACTION__SetVariable_SetLodestoneLoc_RequireLodestoneAtLocation_False(self.clone(), location)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetArmorTrim)]
    #[inline(always)]
    pub fn with_trim(&self, pattern : TrimPattern, material : TrimMaterial) -> Item { unsafe {
        DF_ACTION__SetVariable_SetArmorTrim_TrimPattern_DynamicNone_TrimMaterial_DynamicAmethyst(pattern.to_string(), material.to_string(), self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetArmorTrim)]
    #[inline(always)]
    pub fn without_trim(&self) -> Item { unsafe {
        DF_ACTION__SetVariable_SetArmorTrim_TrimPattern_None_TrimMaterial_Amethyst(self.clone())
    } }

    // TODO: nutrition

    // TODO: saturation

    // TODO: eating_time

    // TODO: with_food

    // TODO: with_tool

    // TODO: with_tool_rules

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemHideTooltip)]
    #[inline(always)]
    pub fn with_tooltip(&self, tooltip : Toggle) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemHideTooltip_Tooltip_DynamicEnable(tooltip.to_string(), self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetArmorTrim)]
    #[inline(always)]
    pub fn with_glowing(&self, glowing : ItemGlowing) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemGlowing_Glowing_DynamicDefault(glowing.to_string(), self.clone())
    } }


    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetItemColor)]
    #[inline(always)]
    pub fn colour(&self) -> Colour { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_GetItemColor(self.clone()))
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetItemColor)]
    #[inline(always)]
    pub fn color(&self) -> Color { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_GetItemColor(self.clone()))
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetItemColor)]
    #[inline(always)]
    pub fn with_colour(&self, colour : Colour) -> Colour { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_SetItemColor(self.clone(), colour.hexcode()))
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetItemColor)]
    #[inline(always)]
    pub fn with_color(&self, color : Color) -> Color { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_SetItemColor(self.clone(), color.hexcode()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemAttribute)]
    #[inline(always)]
    pub fn attribute(&self, attribute : ItemAttr, slot : ItemAttrSlot) -> Float { unsafe {
        DF_ACTION__SetVariable_GetItemAttribute_Attribute_DynamicArmor_ActiveEquipmentSlot_DynamicAny(attribute.to_string(), slot.to_string(), self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/AddItemAttribute)]
    #[inline(always)]
    pub fn with_attribute<F : Into<Float>>(&self, attribute : ItemAttr, slot : ItemAttrSlot, operation : ItemAttrOp, amount : F) -> Item { unsafe {
        DF_ACTION__SetVariable_AddItemAttribute_Attribute_DynamicArmor_Operation_DynamicAddNumber_ActiveEquipmentSlot_DynamicAny(attribute.to_string(), operation.to_string(), slot.to_string(), self.clone(), amount.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetMapTexture)]
    #[inline(always)]
    pub fn with_map_texture(&self, image_uri : String) -> Item { unsafe {
        DF_ACTION__SetVariable_SetMapTexture(self.clone(), image_uri)
    } }

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
    fn DF_ACTION__SetVariable_String( from : DFOpaqueValue ) -> String;

    fn DF_ACTION__SetVariable_GetItemType_ReturnValueType_ItemID( item : Item ) -> String;
    fn DF_ACTION__SetVariable_GetItemType_ReturnValueType_ItemName( item : Item ) -> String;
    fn DF_ACTION__SetVariable_GetItemType_ReturnValueType_Item( item : Item ) -> Item;
    fn DF_ACTION__SetVariable_GetItemName( item : Item ) -> Text;
    fn DF_ACTION__SetVariable_SetItemName( item : Item, name : Text ) -> Item;
    fn DF_ACTION__SetVariable_GetItemAmount( item : Item ) -> UInt;
    fn DF_ACTION__SetVariable_SetItemAmount( item : Item, count : UInt ) -> Item;
    fn DF_ACTION__SetVariable_GetMaxAmount( item : Item ) -> UInt;
    fn DF_ACTION__SetVariable_SetMaxAmount( item : Item, count : UInt ) -> Item;
    fn DF_ACTION__SetVariable_GetHeadOwner_TextValue_OwnerName( item : Item ) -> String;
    fn DF_ACTION__SetVariable_GetHeadOwner_TextValue_OwnerUUID( item : Item ) -> String;
    fn DF_ACTION__SetVariable_SetHeadTexture( item : Item, name_uuid_or_texture : String ) -> Item;
    fn DF_ACTION__SetVariable_GetItemTag( item : Item, key : String ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_GetAllItemTags( item : Item ) -> Dict<DFOpaqueValue>;
    fn DF_ACTION__SetVariable_GetLodestoneLoc( item : Item ) -> Location;
    fn DF_ACTION__SetVariable_SetLodestoneLoc_RequireLodestoneAtLocation_False( item : Item, location : Location ) -> Item;
    fn DF_ACTION__SetVariable_SetArmorTrim_TrimPattern_DynamicNone_TrimMaterial_DynamicAmethyst( pattern : String, material : String, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetArmorTrim_TrimPattern_None_TrimMaterial_Amethyst( item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemHideTooltip_Tooltip_DynamicEnable( tooltip : String, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemGlowing_Glowing_DynamicDefault( glowing : String, item : Item ) -> Item;

    fn DF_ACTION__SetVariable_GetItemColor( item : Item ) -> String;
    fn DF_ACTION__SetVariable_SetItemColor( item : Item, hexcode : String ) -> Item;
    fn DF_ACTION__SetVariable_GetItemAttribute_Attribute_DynamicArmor_ActiveEquipmentSlot_DynamicAny( attribute : String, slot : String, item : Item ) -> Float;
    fn DF_ACTION__SetVariable_AddItemAttribute_Attribute_DynamicArmor_Operation_DynamicAddNumber_ActiveEquipmentSlot_DynamicAny( attribute : String, operation : String, slot : String, item : Item, amount : Float ) -> Item;
    fn DF_ACTION__SetVariable_SetMapTexture( item : Item, image_uri : String ) -> Item;

}


include!("../../bind/item.rs");
