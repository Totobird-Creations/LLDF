use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A selection consisting of any number of player entities.
#[repr(transparent)]
pub struct PlayerSel {
    pub(crate) uuids : List<Uuid>
}

impl PlayerSel {

    pub unsafe fn from_uuid_unchecked(uuid : Uuid) -> Self {
        Self { uuids : transmute_unchecked(DF_ACTION__SetVariable_CreateList(uuid)) }
    }

    pub unsafe fn from_uuids_unchecked(uuids : List<Uuid>) -> Self { Self { uuids } }

}

/// `PLAYER_ACTION` / `Item Management`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/GiveItems)]
    #[inline(always)]
    pub fn give_item<I : AsRef<Item>>(&self, item : I) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_GiveItems(item.as_ref().to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetHotbar)] // FIXME: Arrays are completely broken
    #[inline(always)]
    pub fn set_hotbar_items(&self, items : [Item; 9]) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetHotbar(items);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SetInventory

    #[lldf_bind_proc::dfdoc(PlayerAction/SetSlotItem)]
    #[inline(always)]
    pub fn set_item_in_slot<U : Into<UInt>>(&self, slot : U, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetSlotItem(item.to_opaque(), slot.into());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = MainHand })]
    #[inline(always)]
    pub fn set_mainhand_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_MainHand(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = OffHand })]
    #[inline(always)]
    pub fn set_offhand_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_OffHand(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Head })]
    #[inline(always)]
    pub fn set_head_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Head(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Chest })]
    #[inline(always)]
    pub fn set_chest_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Chest(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Legs })]
    #[inline(always)]
    pub fn set_legs_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Legs(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Feet })]
    #[inline(always)]
    pub fn set_feet_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Feet(item);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SetArmour

    // TODO: ReplaceItems

    // TODO: RemoveItems

    // TODO: ClearItems

    // TODO: ClearInventory tags?
    #[lldf_bind_proc::dfdoc(PlayerAction/ClearInv { ClearCraftingAndCursor = True, ClearMode = EntireInventory })]
    #[inline(always)]
    pub fn clear_inventory(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_ClearInv_ClearCraftingAndCursor_True_ClearMode_EntireInventory();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetCursorItem)]
    #[inline(always)]
    pub fn set_cursor_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetCursorItem(item);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SaveInv)]
    #[inline(always)]
    pub fn save_inventory(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SaveInv();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/LoadInv)]
    #[inline(always)]
    pub fn load_saved_inventory(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_LoadInv();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetItemCooldown)]
    #[inline(always)]
    pub fn set_item_cooldown(&self, item : Item, cooldown_ticks : UInt) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetItemCooldown(item, cooldown_ticks);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/GetItemCooldown)]
    #[inline(always)]
    pub fn get_item_cooldown(&self, item : Item) -> UInt { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        let out = DF_ACTION__PlayerAction_GetItemCooldown(item);
        DF_ACTION__SelectObject_Reset();
        out
    } }

}

/// `PLAYER_ACTION` / `Communication`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/SendMessage)]
    #[inline(always)]
    pub fn send_message<T : Into<Text>>(&self, text : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False(String::new(), text.into());
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SendMessageSeq

    #[lldf_bind_proc::dfdoc(PlayerAction/SendTitle)]
    #[inline(always)]
    pub fn send_title<T : Into<Text>, S : Into<Text>, U0 : Into<UInt>, U1 : Into<UInt>, U2 : Into<UInt>>(&self, title : T, subtitle : S, fade_in_ticks : U0, hold_ticks : U1, fade_out_ticks : U2) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SendTitle(title.into(), subtitle.into(), hold_ticks.into(), fade_in_ticks.into(), fade_out_ticks.into());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/ActionBar)]
    #[inline(always)]
    pub fn send_actionbar<T : DFValue>(&self, text : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_ActionBar_InheritStyles_False_TextValueMerging_NoSpaces(String::new(), text.to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/OpenBook)]
    #[inline(always)]
    pub fn open_book(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_OpenBook(item);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetBossBar)]
    #[inline(always)]
    pub fn set_boss_bar<T : Into<Text>, F0 : Into<Float>, F1 : Into<Float>, P : Into<UInt>>(&self, title : T, progress : F0, maximum : F1, position : P, sky_effect : SkyEffect, style : BarStyle, colour : BarColour) -> () { unsafe {
        let position = DF_TRANSMUTE__Opaque_UInt(DF_ACTION__SetVariable_Specialcharplus(position.into().to_opaque(), UInt::from(1usize).to_opaque()));
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SpecialcharspaceSetBossBarSpecialcharspace_SkyEffect_DynamicNone_BarStyle_DynamicSolid_BarColor_DynamicPurple(sky_effect.to_string(), style.to_string(), colour.to_string(), title.into(), progress.into(), maximum.into(), position);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/RemoveBossBar)]
    #[inline(always)]
    pub fn remove_boss_bar<P : Into<UInt>>(&self, position : P) -> () { unsafe {
        let position = DF_TRANSMUTE__Opaque_UInt(DF_ACTION__SetVariable_Specialcharplus(position.into().to_opaque(), UInt::from(1usize).to_opaque()));
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_RemoveBossBar(position);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/RemoveBossBar)]
    #[inline(always)]
    pub fn clear_boss_bars(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_RemoveBossBar();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SendAdvancement)]
    #[inline(always)]
    pub fn send_advancement<T : Into<Text>, I : AsRef<Item>>(&self, title : T, icon : I, frame : AdvancementFrame) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SendAdvancement_ToastType_DynamicAdvancement(frame.to_string(), title.into(), icon.as_ref().to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetTabListInfo)]
    #[inline(always)]
    pub fn send_tablist_header<T : Into<Text>>(&self, title : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetTabListInfo_PlayerListField_Header(title.into());
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetTabListInfo)]
    #[inline(always)]
    pub fn send_tablist_footer<T : Into<Text>>(&self, title : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetTabListInfo_PlayerListField_Footer(title.into());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/PlaySound)]
    #[inline(always)]
    pub fn play_sound<L : AsRef<Location>, S : AsRef<Sound>>(&self, location : L, sound : S, channel : SoundChannel) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_PlaySound_SoundSource_DynamicMaster(channel.to_string(), sound.as_ref().to_opaque(), location.as_ref().to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/StopSound)]
    #[inline(always)]
    pub fn stop_sound(&self, sound : Sound, channel : SoundChannel) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_StopSound_SoundSource_DynamicMaster(channel.to_string(), sound);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/StopSound)]
    #[inline(always)]
    pub fn stop_all_sounds(&self, channel : SoundChannel) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_StopSound_SoundSource_DynamicMaster(channel.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: PlaySoundSeq

    // TODO: PlayEntitySound

}

/// `PLAYER_ACTION` / `Inventory Menus`
impl PlayerSel {

    // TODO: ShowInv

    // TODO: ExpandInv

    // TODO: SetMenuItem

    // TODO: SetInvName

    // TODO: AddInvRow

    // TODO: RemoveInvRow

    // TODO: CloseInv

    // TODO: OpenBlockInv

}

/// `PLAYER_ACTION` / `Scoreboard Manipulation`
impl PlayerSel {

    // TODO: SetScoreObj

    // TODO: SetSidebar

    // TODO: SetScore

    // TODO: RemoveScore

    // TODO: ScoreDefFormat

    // TODO: ScoreLineFormat

    // TODO: ClearScoreboard

}

/// `PLAYER_ACTION` / `Statistics`
impl PlayerSel {

    // TODO: Damage

    // TODO: Heal

    // TODO: SetHealth

    // TODO: SetMaxHealth

    // TODO: SetAbsorption

    // TODO: SetFoodLevel

    // TODO: GiveFood

    // TODO: SetSaturation

    // TODO: GiveSaturation

    // TODO: SetExhaustion

    // TODO: GiveExhaustion

    // TODO: GiveExp

    // TODO: SetExp

    #[lldf_bind_proc::dfdoc(PlayerAction/GivePotion { OverwriteEffect = True })]
    #[inline(always)]
    pub fn give_potion(&self, potion : Potion, icon : Flag, particles : PotionParticles) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_GivePotion_ShowIcon_DynamicTrue_OverwriteEffect_True_EffectParticles_DynamicTrue(icon.to_string(), particles.to_string(), potion);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: RemovePotion

    // TODO: ClearPotions

    // TODO: SetSlot

    // TODO: SetFireTicks

    // TODO: SetFreezeTicks

    // TODO: SetAirTicks

    // TODO: SetInvulTicks

    // TODO: SetFallDistance

    // TODO: HealthAttribute

    // TODO: CombatAttribute

    // TODO: KBAttribute

    // TODO: MovementAttribute

    // TODO: FallingAttribute

    // TODO: MiningAttribute

    // TODO: ReachAttribute

    // TODO: MiscAttribute

}

/// `PLAYER_ACTION` / `Settings`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/SurvivalMode)]
    #[inline(always)]
    pub fn set_gamemode_survival(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SurvivalMode();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/AdventureMode)]
    #[inline(always)]
    pub fn set_gamemode_adventure(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_AdventureMode();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/CreativeMode)]
    #[inline(always)]
    pub fn set_gamemode_creative(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_CreativeMode();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SpectatorMode)]
    #[inline(always)]
    pub fn set_gamemode_spectator(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SpectatorMode();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetAllowFlight)]
    #[inline(always)]
    pub fn set_allow_flight(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetAllowFlight_AllowFlight_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetAllowPVP)]
    #[inline(always)]
    pub fn set_allow_pvp(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetAllowPVP_PVP_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetDropsEnabled)]
    #[inline(always)]
    pub fn set_death_drops(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetDropsEnabled_SpawnDeathDrops_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetInventoryKept)]
    #[inline(always)]
    pub fn set_keep_inventory(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetInventoryKept_InventoryKept_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetCollidable)]
    #[inline(always)]
    pub fn set_entity_collision(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetCollidable_Collision_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: EnableBlocks

    // TODO: DisableBlocks

    #[lldf_bind_proc::dfdoc(PlayerAction/InstantRespawn)]
    #[inline(always)]
    pub fn set_instant_respawn(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_InstantRespawn_InstantRespawn_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetReducedDebug)]
    #[inline(always)]
    pub fn set_reduced_debug_info(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_InstantRespawn_InstantRespawn_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetHandCrafting)]
    #[inline(always)]
    pub fn set_allow_hand_crafting(&self, enable : Toggle) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetHandCrafting_AllowHandCrafting_DynamicFalse(enable.to_string());
        DF_ACTION__SelectObject_Reset();
    } }

}

/// `PLAYER_ACTION` / `Movement`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/Teleport)]
    #[inline(always)]
    pub fn set_location(&self, location : Location) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_Teleport_KeepVelocity_True_KeepCurrentRotation_True(location);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: LaunchUp

    // TODO: LaunchFwd

    // TODO: LaunchToward

    // TODO: RideEntity

    // TODO: SetFlying

    // TODO: SetGliding

    // TODO: BoostElytra

    // TODO: SetRotation

    // TODO: FaceLocation

    // TODO: SetVelocity

    // TODO: SpectateTarget

    // TODO: SetSpawnPoint

    // TODO: SpectatorCollision

}

/// `PLAYER_ACTION` / `World`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/SetTickRate)]
    #[inline(always)]
    pub fn set_tick_rate(&self, ticks_per_second : UInt) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetTickRate(ticks_per_second);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: LaunchProj

    // TODO: ResourcePack

    // TODO: SetPlayerTime

    // TODO: SetPlayerWeather

    // TODO: SetRainLevel

    // TODO: SetCompass

    // TODO: DisplayBlock

    // TODO: DispHeadTexture

    // TODO: ClearDispBlock

    // TODO: DisplayFracture

    // TODO: DisplayBlockOpen

    // TODO: DisplayBellRing

    // TODO: DisplayGateway

    // TODO: DisplaySignText

    // TODO: DisplayHologram

    // TODO: SetFogDistance

    // TODO: SetWorldBorder

    // TODO: ShiftWorldBorder

    // TODO: RmWorldBorder

    // TODO: DisplayPickup

    // TODO: SetEntityHidden

    // TODO: DisplayEquipment

}

/// `PLAYER_ACTION` / `Visual Effects`
impl PlayerSel {

    // TODO: Particle

    // TODO: ParticleLine

    // TODO: ParticleLineA

    // TODO: ParticleCircle

    // TODO: ParticleCircleA

    // TODO: ParticleCuboid

    // TODO: ParticleCuboidA

    // TODO: ParticleSpiral

    // TODO: ParticleSpiralA

    // TODO: ParticleSphere

    // TODO: ParticleRay

    // TODO: DisplayLightning

    // TODO: Vibration

}

/// `PLAYER_ACTION` / `Apperance`
impl PlayerSel {

    // TODO: MobDisguise

    // TODO: PlayerDisguise

    // TODO: BlockDisguise

    // TODO: SetDisguiseVisible

    // TODO: Undisguise

    // TODO: SetChatTag

    // TODO: ChatStyle

    #[inline(always)]
    fn _set_name_colour(&self, colour : NameColour) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SetNameColor_NameColor_DynamicNone(colour.to_string());
        DF_ACTION__SelectObject_Reset();
    } }
    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(PlayerAction/SetNameColor)]
    #[inline(always)]
    pub fn set_name_colour(&self, colour : NameColour) -> () { self._set_name_colour(colour); }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(PlayerAction/SetNameColor)]
    #[inline(always)]
    pub fn set_name_color(&self, color : NameColor) -> () { self._set_name_colour(color); }

    // TODO: SetNameVisible

    // TODO: SetNamePrefix

    // TODO: SetArrowsStuck

    // TODO: SetStingsStuck

    // TODO: SetVisualFire

    // TODO: AttackAnimation

    // TODO: HurtAnimation

    // TODO: WakeUpAnimation

    // TODO: SetStatus

    // TODO: SetSkin

    // TODO: SetShoulder

}

/// `PLAYER_ACTION` / `Miscellaneous`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/RollbackBlocks)]
    #[inline(always)]
    pub fn rollback_blocks(&self, time_seconds : UInt) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_RollbackBlocks(time_seconds);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SendToPlot)]
    #[inline(always)]
    pub fn send_to_plot(&self, plot_id : UInt) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_SendToPlot(String::from(plot_id));
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/Kick)]
    #[inline(always)]
    pub fn kick(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.to_opaque());
        DF_ACTION__PlayerAction_Kick();
        DF_ACTION__SelectObject_Reset();
    } }

}

/// `IF_PLAYER` / `Toggleable Conditions`
impl PlayerSel {

    // TODO: IsSneaking

    // TODO: IsSprinting

    // TODO: IsGliding

    // TODO: IsFlying

    // TODO: IsGrounded

    // TODO: IsSwimming

    // TODO: IsBlocking

    // TODO: UsingPack

}

/// `IF_PLAYER` / `Locational Conditions`
impl PlayerSel {

    // TODO: IsLookingAt

    // TODO: StandingOn

    // TODO: IsNear

    // TODO: InWorldBorder

}

/// `IF_PLAYER` / `Item Conditions`
impl PlayerSel {

    // TODO: IsHolding

    // TODO: HasItem

    // TODO: IsWearing

    // TODO: IsUsingItem

    // TODO: NoItemCooldown

    // TODO: HasSlotItem

    // TODO: MenuSlotEquals

    // TODO: CursorItem

    // TODO: HasRoomForItem

}

/// `IF_PLAYER` / `Miscellaneous Conditions`
impl PlayerSel {

    // TODO: NameEquals

    // TODO: SlotEquals

    // TODO: IsInGameMode

    // TODO: HasPotion

    // TODO: IsRiding

    // TODO: InvOpen

    // TODO: HasPermission

    // TODO: MainHandEquals

}

/// `SELECT_OBJECT` / `Creating Selections`
impl PlayerSel {

    // TODO: RandomPlayer

    // TODO: PlayerName

    // TODO: AllPlayers

    // TODO: Invert

}

/// `SELECT_OBJECT` / `Selection Filters`
impl PlayerSel {

    // TODO: FilterRandom

    // TODO: FilterDistance

    // TODO: FilterSort

    // TODO: FilterRay

}

/// `Game Value` / `Statistical Values`
impl PlayerSel {

    // TODO: health

    // TODO: max_health

    // TODO: absorption_health

    // TODO: food

    // TODO: saturation

    // TODO: exhaustion

    // TODO: attack_damage

    // TODO: attack_speed

    // TODO: current_attack_cooldown

    // TODO: held_attack_cooldown

    // TODO: armour

    // TODO: armour_toughness

    // TODO: invul_ticks

    // TODO: exp_level

    // TODO: exp_progress

    // TODO: fire_ticks

    // TODO: freeze_ticks

    // TODO: air_ticks

    // TODO: fall_distance

    // TODO: held_slot

    // TODO: ping

    // TODO: steer_sideways

    // TODO: steer_forward

    // TODO: item_usage_progress

    // TODO: flight_speed

    // TODO: walk_speed

    // TODO: width

    // TODO: height

}

/// `Game Value` / `Locational Values`
impl PlayerSel {

    // TODO: location

    // TODO: target_block_location

    // TODO: target_block_side

    // TODO: eye_location

    // TODO: midpoint_location

    // TODO: x

    // TODO: y

    // TODO: z

    // TODO: yaw

    // TODO: pitch

    // TODO: body_yaw

    // TODO: supporting_block_location

    // TODO: spawn_location

    // TODO: velocity

    // TODO: direction

}

/// `Game Value` / `Item Values`
impl PlayerSel {

    // TODO: mainhand_item

    // TODO: offhand_item

    // TODO: armour_items

    // TODO: hotbar_items

    // TODO: inventory_items

    // TODO: cursor_item

    // TODO: menu_items

}

/// `Game Value` / `Informational Values`
impl PlayerSel {

    // TODO: name

    #[inline(always)]
    pub unsafe fn uuid_unchecked(&self) -> Uuid { unsafe {
        self.uuids.get_unchecked(0usize)
    } }

    // TODO: uuid (with sel size check)

    // TODO: gamemode

    // TODO: menu_title

    // TODO: potions

    // TODO: vehicle_uuid

    // TODO: passenger_uuids

    // TODO: leading_uuid

    // TODO: lead_uuids

    // TODO: shooter_uuid

    // TODO: pose

}

/// `Game Value` / `Plot Values`
impl PlayerSel {

    // TODO: len

    #[inline(always)]
    pub fn uuids<'l>(&'l self) -> &'l List<Uuid> { &self.uuids }

    // TODO: names

}


unsafe impl DFSel for PlayerSel {}





extern "C" {

    fn DF_ACTION__SelectObject_PlayerName( target : DFOpaqueValue ) -> ();
    fn DF_ACTION__SelectObject_Reset( ) -> ();

    fn DF_TRANSMUTE__Opaque_UInt( from : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_Specialcharplus( a : DFOpaqueValue, b : DFOpaqueValue ) -> DFOpaqueValue;


    fn DF_ACTION__SetVariable_CreateList( ... ) -> DFOpaqueValue;

    fn DF_ACTION__PlayerAction_GiveItems( item : DFOpaqueValue ) -> ();
    fn DF_ACTION__PlayerAction_SetHotbar( items : [Item; 9] ) -> ();
    fn DF_ACTION__PlayerAction_SetSlotItem( item : DFOpaqueValue, slot : UInt ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_MainHand( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_OffHand( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Head( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Chest( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Legs( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Feet( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_ClearInv_ClearCraftingAndCursor_True_ClearMode_EntireInventory( ) -> ();
    fn DF_ACTION__PlayerAction_SetCursorItem( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SaveInv( ) -> ();
    fn DF_ACTION__PlayerAction_LoadInv( ) -> ();
    fn DF_ACTION__PlayerAction_SetItemCooldown( item : Item, cooldown_ticks : UInt ) -> ();
    fn DF_ACTION__PlayerAction_GetItemCooldown( item : Item ) -> UInt;

    fn DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False( _ : String, message : Text ) -> ();
    fn DF_ACTION__PlayerAction_SendTitle( title : Text, subtitle : Text, fade_in_ticks : UInt, hold_ticks : UInt, fade_out_ticks : UInt ) -> ();
    fn DF_ACTION__PlayerAction_ActionBar_InheritStyles_False_TextValueMerging_NoSpaces( _ : String, message : DFOpaqueValue ) -> ();
    fn DF_ACTION__PlayerAction_OpenBook( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SpecialcharspaceSetBossBarSpecialcharspace_SkyEffect_DynamicNone_BarStyle_DynamicSolid_BarColor_DynamicPurple( sky_effect : String, bar_style : String, bar_colour : String, title : Text, progress : Float, maximum : Float, position : UInt ) -> ();
    fn DF_ACTION__PlayerAction_RemoveBossBar( ... ) -> ();
    fn DF_ACTION__PlayerAction_SendAdvancement_ToastType_DynamicAdvancement( frame : String, title : Text, icon : DFOpaqueValue ) -> ();
    fn DF_ACTION__PlayerAction_SetTabListInfo_PlayerListField_Header( title : Text ) -> ();
    fn DF_ACTION__PlayerAction_SetTabListInfo_PlayerListField_Footer( title : Text ) -> ();
    fn DF_ACTION__PlayerAction_PlaySound_SoundSource_DynamicMaster( channel : String, sound : DFOpaqueValue, location : DFOpaqueValue ) -> ();
    fn DF_ACTION__PlayerAction_StopSound_SoundSource_DynamicMaster( channel : String, ... ) -> ();

    fn DF_ACTION__PlayerAction_GivePotion_ShowIcon_DynamicTrue_OverwriteEffect_True_EffectParticles_DynamicTrue( icon : String, particles : String, potion : Potion ) -> ();

    fn DF_ACTION__PlayerAction_SurvivalMode( ) -> ();
    fn DF_ACTION__PlayerAction_AdventureMode( ) -> ();
    fn DF_ACTION__PlayerAction_CreativeMode( ) -> ();
    fn DF_ACTION__PlayerAction_SpectatorMode( ) -> ();
    fn DF_ACTION__PlayerAction_SetAllowFlight_AllowFlight_DynamicFalse( enable : String ) -> ();
    fn DF_ACTION__PlayerAction_SetAllowPVP_PVP_DynamicFalse( enable : String ) -> ();
    fn DF_ACTION__PlayerAction_SetDropsEnabled_SpawnDeathDrops_DynamicFalse( enable : String ) -> ();
    fn DF_ACTION__PlayerAction_SetInventoryKept_InventoryKept_DynamicFalse( enable : String ) -> ();
    fn DF_ACTION__PlayerAction_SetCollidable_Collision_DynamicFalse( enable : String ) -> ();
    fn DF_ACTION__PlayerAction_InstantRespawn_InstantRespawn_DynamicFalse( enable : String ) -> ();
    fn DF_ACTION__PlayerAction_SetHandCrafting_AllowHandCrafting_DynamicFalse( enable : String ) -> ();

    fn DF_ACTION__PlayerAction_Teleport_KeepVelocity_True_KeepCurrentRotation_True( location : Location ) -> ();

    fn DF_ACTION__PlayerAction_SetTickRate( ticks_per_second : UInt ) -> ();

    fn DF_ACTION__PlayerAction_SetNameColor_NameColor_DynamicNone( name_color : String ) -> ();

    fn DF_ACTION__PlayerAction_RollbackBlocks( time_seconds : UInt ) -> ();
    fn DF_ACTION__PlayerAction_SendToPlot( plot_id : String ) -> ();
    fn DF_ACTION__PlayerAction_Kick( ) -> ();

}
