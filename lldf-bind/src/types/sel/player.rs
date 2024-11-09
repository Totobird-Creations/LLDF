use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[repr(transparent)]
pub struct PlayerSel {
    pub(crate) uuids : List<String>
}

impl PlayerSel {

    #[doc(hidden)]
    pub unsafe fn from_uuids(uuids : List<String>) -> Self { Self { uuids } }

}

/// `PLAYER_ACTION` / `Item Management`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/GiveItems)]
    #[inline(always)]
    pub fn give_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_GiveItems(item);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SetHotbar

    // TODO: SetInventory

    #[lldf_bind_proc::dfdoc(PlayerAction/SetSlotItem)]
    #[inline(always)]
    pub fn set_item_in_slot(&self, slot : UInt, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetItemInSlot(item, slot);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = MainHand })]
    #[inline(always)]
    pub fn set_mainhand_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_MainHand(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = OffHand })]
    #[inline(always)]
    pub fn set_offhand_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_OffHand(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Head })]
    #[inline(always)]
    pub fn set_head_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Head(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Chest })]
    #[inline(always)]
    pub fn set_chest_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Chest(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Legs })]
    #[inline(always)]
    pub fn set_legs_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Legs(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[lldf_bind_proc::dfdoc(PlayerAction/SetEquipment { EquipmentSlot = Feet })]
    #[inline(always)]
    pub fn set_feet_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Feet(item);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SetArmour

    // TODO: ReplaceItems

    // TODO: RemoveItems

    // TODO: ClearItems

    // TODO: ClearInventory

    #[lldf_bind_proc::dfdoc(PlayerAction/SetCursorItem)]
    #[inline(always)]
    pub fn set_cursor_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetCursorItem(item);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SaveInv)]
    #[inline(always)]
    pub fn save_inventory(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SaveInv();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/LoadInv)]
    #[inline(always)]
    pub fn load_saved_inventory(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_LoadInv();
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SetItemCooldown)]
    #[inline(always)]
    pub fn set_item_cooldown(&self, item : Item, cooldown_ticks : UInt) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetItemCooldown(item, cooldown_ticks);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/GetItemCooldown)]
    #[inline(always)]
    pub fn get_item_cooldown(&self, item : Item) -> UInt { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        let out = DF_ACTION__PlayerAction_GetItemCooldown(item);
        DF_ACTION__SelectObject_Reset();
        out
    } }

}

/// `PLAYER_ACTION` / `Communication`
impl PlayerSel {

    #[lldf_bind_proc::dfdoc(PlayerAction/SendMessage)]
    #[inline(always)]
    pub fn send_message<T : DFValue>(&self, text : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False(text.to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SendMessageSeq

    #[lldf_bind_proc::dfdoc(PlayerAction/SendTitle)]
    #[inline(always)]
    pub fn send_title<T : DFValue, S : DFValue>(&self, title : T, subtitle : S, fade_in_ticks : UInt, hold_ticks : UInt, fade_out_ticks : UInt) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SendTitle_TextValueMerging_NoSpaces_InheritStyles_False(title.to_opaque(), subtitle.to_opaque(), fade_in_ticks, hold_ticks, fade_out_ticks);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/ActionBar)]
    #[inline(always)]
    pub fn send_actionbar<T : DFValue>(&self, text : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_ActionBar(text.to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: OpenBook

    // TODO: SetBossBar

    // TODO: RemoveBossBar

    // TODO: SendAdvancement

    // TODO: SetTagListInfo

    // TODO: PlaySound

    // TODO: StopSound

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

    // TODO: GivePotion

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

    // TODO: MiscAttribute

}

/// `PLAYER_ACTION` / `Settings`
impl PlayerSel {

    // TODO: SurvivalMode

    // TODO: AdventureMode

    // TODO: CreativeMode

    // TODO: SpectatorMode

    // TODO: SetAllowFlight

    // TODO: SetAllowPVP

    // TODO: SetDropsEnabled

    // TODO: SetInventoryKept

    // TODO: SetCollidable

    // TODO: EnableBlocks

    // TODO: DisableBlocks

    // TODO: InstantRespawn

    // TODO: SetReducedDebug

    // TODO: SetHandCrafting

}

/// `PLAYER_ACTION` / `Movement`
impl PlayerSel {

    // TODO: Teleport

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
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
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
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
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
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_RollbackBlocks(time_seconds);
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/SendToPlot)]
    #[inline(always)]
    pub fn send_to_plot(&self, plot_id : UInt) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SendToPlot(String::from(plot_id));
        DF_ACTION__SelectObject_Reset();
    } }

    #[lldf_bind_proc::dfdoc(PlayerAction/Kick)]
    #[inline(always)]
    pub fn kick(&self) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
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

impl PlayerSel {

    #[inline(always)]
    pub fn uuids<'l>(&'l self) -> &'l List<String> { &self.uuids }

}


unsafe impl DFSel for PlayerSel {}





#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_ACTION__SelectObject_PlayerName( target : List<String> ) -> ();
    fn DF_ACTION__SelectObject_Reset( ) -> ();


    fn DF_ACTION__PlayerAction_GiveItems( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetItemInSlot( item : Item, slot : UInt ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_MainHand( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_OffHand( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Head( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Chest( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Legs( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Feet( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SetCursorItem( item : Item ) -> ();
    fn DF_ACTION__PlayerAction_SaveInv( ) -> ();
    fn DF_ACTION__PlayerAction_LoadInv( ) -> ();
    fn DF_ACTION__PlayerAction_SetItemCooldown( item : Item, cooldown_ticks : UInt ) -> ();
    fn DF_ACTION__PlayerAction_GetItemCooldown( item : Item ) -> UInt;

    fn DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False( message : DFOpaqueValue ) -> ();
    fn DF_ACTION__PlayerAction_SendTitle_TextValueMerging_NoSpaces_InheritStyles_False( title : DFOpaqueValue, subtitle : DFOpaqueValue, fade_in_ticks : UInt, hold_ticks : UInt, fade_out_ticks : UInt ) -> ();
    fn DF_ACTION__PlayerAction_ActionBar( message : DFOpaqueValue ) -> ();

    fn DF_ACTION__PlayerAction_SetTickRate( ticks_per_second : UInt ) -> ();

    fn DF_ACTION__PlayerAction_SetNameColor_NameColor_DynamicNone( name_color : String ) -> ();

    fn DF_ACTION__PlayerAction_RollbackBlocks( time_seconds : UInt ) -> ();
    fn DF_ACTION__PlayerAction_SendToPlot( plot_id : String ) -> ();
    fn DF_ACTION__PlayerAction_Kick( ) -> ();

}
