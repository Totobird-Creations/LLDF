use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


#[repr(transparent)]
pub struct PlayerSel {
    pub(crate) uuids : List<String>
}

impl PlayerSel {

    #[doc(hidden)]
    pub unsafe fn from_uuids(uuids : DFOpaqueValue) -> Self { unsafe {
        Self { uuids : DF_TRANSMUTE__ListString(uuids) }
    } }

}

impl PlayerSel {

    #[inline(always)]
    pub fn uuids<'l>(&'l self) -> &'l List<String> { &self.uuids }

    #[inline(always)]
    pub fn names(&self) -> &List<Text> { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        let names = crate::bind::gamevalue::DF_GAMEVALUE__SelectionTargetNames_Default();
        DF_ACTION__SelectObject_Reset();
        transmute_unchecked(&DF_TRANSMUTE__ListText(names))
    } }

}

/// `PLAYER_ACTION` / `Item Management`
impl PlayerSel {

    #[inline(always)]
    pub fn give_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_GiveItems(item);
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SetHotbar

    // TODO: SetInventory

    #[inline(always)]
    pub fn set_item_in_slot(&self, slot : UInt, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetItemInSlot(item, slot);
        DF_ACTION__SelectObject_Reset();
    } }

    #[inline(always)]
    pub fn set_mainhand_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_MainHand(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[inline(always)]
    pub fn set_offhand_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_OffHand(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[inline(always)]
    pub fn set_head_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Head(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[inline(always)]
    pub fn set_chest_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Chest(item);
        DF_ACTION__SelectObject_Reset();
    } }
    #[inline(always)]
    pub fn set_legs_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SetEquipment_EquipmentSlot_Legs(item);
        DF_ACTION__SelectObject_Reset();
    } }
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

    // TODO: SetCursorItem

    // TODO: SaveInv

    // TODO: LoadInv

    // TODO: SetItemCooldown

    // TODO: GetItemCooldown

}

/// `PLAYER_ACTION` / `Communication`
impl PlayerSel {

    #[inline(always)]
    pub fn send_message<T : DFValue>(&self, text : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False(text.to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

    // TODO: SendMessageSeq

    // TODO: SendTitle

    // TODO: ActionBar

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

    // TODO: SetTickRate

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

    // TODO: SetNameColour

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

    // TODO: RollbackBlocks

    // TODO: SendToPlot

    // TODO: Kick

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


unsafe impl DFSel for PlayerSel {}





#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__ListString( from : DFOpaqueValue ) -> List<String>;
    fn DF_TRANSMUTE__ListText( from : DFOpaqueValue ) -> List<Text>;

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

    fn DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False( message : DFOpaqueValue ) -> ();

}
