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

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemLore)]
    #[inline(always)]
    pub fn lore(&self) -> List<Text> { unsafe {
        DF_ACTION__SetVariable_GetItemLore(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetLoreLine)]
    #[inline(always)]
    pub fn lore_line<U : Into<UInt>>(&self, index : U) -> Text { unsafe { // TODO: Add bounds check
        let index = DF_TRANSMUTE__Opaque_UInt(DF_ACTION__SetVariable_Specialcharplus(index.into().to_opaque(), UInt::from(1usize).to_opaque()));
        DF_ACTION__SetVariable_GetLoreLine(self.clone(), index)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemLore)]
    #[inline(always)]
    pub fn with_lore(&self, lore : List<Text>) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemLore(self.clone(), lore)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemLore)]
    #[inline(always)]
    pub fn with_lore_line<U : Into<UInt>>(&self, index : U, line : Text) -> Item { unsafe {
        let index = DF_TRANSMUTE__Opaque_UInt(DF_ACTION__SetVariable_Specialcharplus(index.into().to_opaque(), UInt::from(1usize).to_opaque()));
        DF_ACTION__SetVariable_SetItemLore(self.clone(), line, index)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/AddItemLore)]
    #[inline(always)]
    pub fn with_lore_line_added(&self, line : Text) -> Item { unsafe {
        DF_ACTION__SetVariable_AddItemLore(self.clone(), line)
    } }

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
    /// ##### Warning
    /// - **Numeric tag values will be converted to strings. Thus, [`String`](String) `"0"` and [`Float`](Float) `0`** are not differentiable.
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

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemTag)]
    #[inline(always)]
    pub fn with_tag(&self, key : String, value : String) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemTag(self.clone(), key, value)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/RemoveItemTag)]
    #[inline(always)]
    pub fn without_tag(&self, key : String) -> Item { unsafe {
        DF_ACTION__SetVariable_RemoveItemTag(self.clone(), key)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ClearItemTag)]
    #[inline(always)]
    pub fn without_tags(&self) -> Item { unsafe {
        DF_ACTION__SetVariable_ClearItemTag(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetModelData)]
    #[inline(always)]
    pub fn with_modeldata(&self, index : UInt) -> Item { unsafe {
        DF_ACTION__SetVariable_SetModelData(self.clone(), index)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemEffects)]
    #[inline(always)]
    pub fn potions(&self) -> List<Potion> { unsafe {
        DF_ACTION__SetVariable_GetItemEffects(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemEffects)]
    #[inline(always)]
    pub fn with_potions(&self, potions : List<Potion>) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemEffects(self.clone(), potions)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_trim(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_DynamicNoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_colour(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_DynamicNoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_color(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_DynamicNoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_enchants(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_DynamicNoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_attributes(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_DynamicNoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_unbreakable(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_DynamicNoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_breakables(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_DynamicNoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_placeables(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_DynamicNoChange_PotionEffects_NoChange_Others_NoChange(visible, self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_potions(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_DynamicNoChange_Others_NoChange(visible, self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFlags)]
    #[inline(always)]
    pub fn with_hideflag_others(&self, visible : Visible) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_DynamicNoChange(visible, self.clone())
    } }

    // TODO: placeables

    // TODO: with_placeables

    // TODO: breakables

    // TODO: with_breakables

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemRarity)]
    #[inline(always)]
    pub fn rarity(&self) -> ItemRarity { unsafe {
        ItemRarity::from_string_unchecked(DF_ACTION__SetVariable_GetItemRarity(self.clone()))
    } }

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

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemFood { FoodProperty = Nutrition })]
    #[inline(always)]
    pub fn nutrition(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetItemFood_FoodProperty_Nutrition(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemFood { FoodProperty = Saturation })]
    #[inline(always)]
    pub fn saturation(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetItemFood_FoodProperty_Saturation(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetItemFood { FoodProperty = EatingDuration })]
    #[inline(always)]
    pub fn eating_time(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetItemFood_FoodProperty_EatingDuration(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetItemFood)]
    #[inline(always)]
    pub fn with_food(&self, nutrition : Float, saturation : Float, eating_time_seconds : Float, always_edible : Flag) -> Item { unsafe {
        DF_ACTION__SetVariable_SetItemFood_CanAlwaysEat_DynamicFalse(self.clone(), always_edible.to_string(), nutrition, saturation, eating_time_seconds)
    } }

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
    /// ##### Unsafe
    /// - **May cause large plot CPU usage spikes, causing plot to lagslay.**
    #[inline(always)]
    pub unsafe fn with_map_texture(&self, image_uri : String) -> Item { unsafe {
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

    fn DF_TRANSMUTE__Opaque_UInt( from : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_Specialcharplus( a : DFOpaqueValue, b : DFOpaqueValue ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_GetItemType_ReturnValueType_ItemID( item : Item ) -> String;
    fn DF_ACTION__SetVariable_GetItemType_ReturnValueType_ItemName( item : Item ) -> String;
    fn DF_ACTION__SetVariable_GetItemType_ReturnValueType_Item( item : Item ) -> Item;
    fn DF_ACTION__SetVariable_GetItemName( item : Item ) -> Text;
    fn DF_ACTION__SetVariable_SetItemName( item : Item, name : Text ) -> Item;
    fn DF_ACTION__SetVariable_GetItemLore( item : Item ) -> List<Text>;
    fn DF_ACTION__SetVariable_GetLoreLine( item : Item, index : UInt ) -> Text;
    fn DF_ACTION__SetVariable_SetItemLore( item : Item, ... ) -> Item;
    fn DF_ACTION__SetVariable_AddItemLore( item : Item, line : Text ) -> Item;
    fn DF_ACTION__SetVariable_GetItemAmount( item : Item ) -> UInt;
    fn DF_ACTION__SetVariable_SetItemAmount( item : Item, count : UInt ) -> Item;
    fn DF_ACTION__SetVariable_GetMaxAmount( item : Item ) -> UInt;
    fn DF_ACTION__SetVariable_SetMaxAmount( item : Item, count : UInt ) -> Item;
    fn DF_ACTION__SetVariable_GetHeadOwner_TextValue_OwnerName( item : Item ) -> String;
    fn DF_ACTION__SetVariable_GetHeadOwner_TextValue_OwnerUUID( item : Item ) -> String;
    fn DF_ACTION__SetVariable_SetHeadTexture( item : Item, name_uuid_or_texture : String ) -> Item;
    fn DF_ACTION__SetVariable_GetItemTag( item : Item, key : String ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_GetAllItemTags( item : Item ) -> Dict<DFOpaqueValue>;
    fn DF_ACTION__SetVariable_SetItemTag( item : Item, key : String, value : String ) -> Item;
    fn DF_ACTION__SetVariable_RemoveItemTag( item : Item, key : String ) -> Item;
    fn DF_ACTION__SetVariable_ClearItemTag( item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetModelData( item : Item, index : UInt ) -> Item;
    fn DF_ACTION__SetVariable_GetItemEffects( item : Item ) -> List<Potion>;
    fn DF_ACTION__SetVariable_SetItemEffects( item : Item, potions : List<Potion> ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_DynamicNoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_DynamicNoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_DynamicNoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_DynamicNoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_DynamicNoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_DynamicNoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_DynamicNoChange_PotionEffects_NoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_DynamicNoChange_Others_NoChange( visible : Visible, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemFlags_ArmorTrim_NoChange_Color_NoChange_Enchantments_NoChange_Attributes_NoChange_Unbreakable_NoChange_CanDestroy_NoChange_CanPlaceOn_NoChange_PotionEffects_NoChange_Others_DynamicNoChange( visible : Visible, item : Item ) -> Item;

    fn DF_ACTION__SetVariable_GetItemRarity( item : Item ) -> String;
    fn DF_ACTION__SetVariable_GetLodestoneLoc( item : Item ) -> Location;
    fn DF_ACTION__SetVariable_SetLodestoneLoc_RequireLodestoneAtLocation_False( item : Item, location : Location ) -> Item;
    fn DF_ACTION__SetVariable_SetArmorTrim_TrimPattern_DynamicNone_TrimMaterial_DynamicAmethyst( pattern : String, material : String, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetArmorTrim_TrimPattern_None_TrimMaterial_Amethyst( item : Item ) -> Item;
    fn DF_ACTION__SetVariable_GetItemFood_FoodProperty_Nutrition( item : Item ) -> Float;
    fn DF_ACTION__SetVariable_GetItemFood_FoodProperty_Saturation( item : Item ) -> Float;
    fn DF_ACTION__SetVariable_GetItemFood_FoodProperty_EatingDuration( item : Item ) -> Float;
    fn DF_ACTION__SetVariable_SetItemFood_CanAlwaysEat_DynamicFalse( item : Item, always_edible : String, nutrition : Float, saturation : Float, eating_time_seconds : Float ) -> Item;
    fn DF_ACTION__SetVariable_SetItemHideTooltip_Tooltip_DynamicEnable( tooltip : String, item : Item ) -> Item;
    fn DF_ACTION__SetVariable_SetItemGlowing_Glowing_DynamicDefault( glowing : String, item : Item ) -> Item;

    fn DF_ACTION__SetVariable_GetItemColor( item : Item ) -> String;
    fn DF_ACTION__SetVariable_SetItemColor( item : Item, hexcode : String ) -> Item;
    fn DF_ACTION__SetVariable_GetItemAttribute_Attribute_DynamicArmor_ActiveEquipmentSlot_DynamicAny( attribute : String, slot : String, item : Item ) -> Float;
    fn DF_ACTION__SetVariable_AddItemAttribute_Attribute_DynamicArmor_Operation_DynamicAddNumber_ActiveEquipmentSlot_DynamicAny( attribute : String, operation : String, slot : String, item : Item, amount : Float ) -> Item;
    fn DF_ACTION__SetVariable_SetMapTexture( item : Item, image_uri : String ) -> Item;

}


include!("../../bind/item.rs");
