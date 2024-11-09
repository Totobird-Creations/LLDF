use super::*;


#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Calling_an_event_trigger_is_not_allowed {}


pub macro event_trigger {


    // `PLAYER_EVENT` `Plot and Server Events`

    { PlayerJoin, $original_func_ident:ident } => { event_trigger_inner!{ DF_EVENT__Event_Join, $original_func_ident, {
        { unsafe {
            extern "C" {
                fn DF_ACTION__SelectObject_EventTarget_EventTarget_Default( ) -> ();
                fn DF_GAMEVALUE__SelectionTargetUUIDs_Default( ) -> ::lldf_bind::types::List<::lldf_bind::types::String>;
                fn DF_ACTION__SelectObject_Reset( ) -> ();
            }
            DF_ACTION__SelectObject_EventTarget_EventTarget_Default();
            let uuids = DF_GAMEVALUE__SelectionTargetUUIDs_Default();
            DF_ACTION__SelectObject_Reset();
            ::lldf_bind::types::PlayerSel::from_uuids(uuids)
        } }
    }} },

    { PlayerLeave, $original_func_ident:ident } => { event_trigger_inner!{ DF_EVENT__Event_Leave, $original_func_ident, {
        { unsafe {
            extern "C" {
                fn DF_ACTION__SelectObject_EventTarget_EventTarget_Default( ) -> ();
                fn DF_GAMEVALUE__SelectionTargetUUIDs_Default( ) -> ::lldf_bind::types::List<::lldf_bind::types::String>;
                fn DF_ACTION__SelectObject_Reset( ) -> ();
            }
            DF_ACTION__SelectObject_EventTarget_EventTarget_Default();
            let uuids = DF_GAMEVALUE__SelectionTargetUUIDs_Default();
            DF_ACTION__SelectObject_Reset();
            ::lldf_bind::types::PlayerSel::from_uuids(uuids)
        } }
    }} },

    // TODO: Command

    // TODO: PackLoad

    // TODO: PackDecline


    // `PLAYER_EVENT` `Click Events`

    // TODO: RightClick

    // TODO: LeftClick

    // TODO: ClickEntity

    // TODO: LeftClickEntity

    // TODO: ClickPlayer

    // TODO: LeftClickPlayer

    // TODO: LoadCrossbow

    // TODO: PlaceBlock

    // TODO: BreakBlock

    { PlayerSwapHands, $original_func_ident:ident } => { event_trigger_inner!{ DF_EVENT__Event_SwapHands, $original_func_ident, {
        { unsafe {
            extern "C" {
                fn DF_ACTION__SelectObject_EventTarget_EventTarget_Default( ) -> ();
                fn DF_GAMEVALUE__SelectionTargetUUIDs_Default( ) -> ::lldf_bind::types::List<::lldf_bind::types::String>;
                fn DF_ACTION__SelectObject_Reset( ) -> ();
            }
            DF_ACTION__SelectObject_EventTarget_EventTarget_Default();
            let uuids = DF_GAMEVALUE__SelectionTargetUUIDs_Default();
            DF_ACTION__SelectObject_Reset();
            ::lldf_bind::types::PlayerSel::from_uuids(uuids)
        } }
    }} },

    // TODO: ChangeSlot

    // TODO: TameEntity


    // `PLAYER_EVENT` `Movement Events`

    // TODO: Walk

    // TODO: Move

    // TODO: Jump

    // TODO: Sneak

    // TODO: Unsneak

    // TODO: StartSprint

    // TODO: StopSprint

    // TODO: StartFly

    // TODO: StopFly

    // TODO: Riptide

    // TODO: Dismount

    // TODO: HorseJump

    // TODO: VehicleJump

    // TODO: Teleport


    // `PLAYER_EVENT` `Item Events`

    // TODO: ClickMenuSlot

    // TODO: ClickInvSlot

    // TODO: ClickContainerSlot

    // TODO: PickUpItem

    // TODO: DropItem

    // TODO: Consume

    // TODO: BreakItem

    // TODO: CloseInv

    // TODO: Fish


    // `PLAYER_EVENT` `Damage Events`

    // TODO: PlayerTakeDmg

    // TODO: PlayerDmgPlayer

    // TODO: DamageEntity

    // TODO: EntityDmgPlayer

    // TODO: PlayerHeal

    // TODO: ShootBow

    // TODO: ShootProjectile

    // TODO: ProjHit

    // TODO: ProjDmgPlayer

    // TODO: CloudImbuePlayer

    // TODO: Exhaustion


    // `PLAYER_EVENT` `Death Events`

    // TODO: Death

    // TODO: KillPlayer

    // TODO: PlayerResurrect

    // TODO: KillMob

    // TODO: MobKillPlayer

    // TODO: Respawn


    // `ENTITY_EVENT`

    // TODO: EntityDmgEntity

    // TODO: EntityKillEntity

    // TODO: EntityDmg

    // TODO: ProjDmgEntity

    // TODO: ProjKillEntity

    // TODO: EntityDeath

    // TODO: EntityExplode

    // TODO: VehicleDamage

    // TODO: BlockFall

    // TODO: FallingBlockLand

    // TODO: EntityResurrect

    // TODO: RegrowWool

    // TODO: ItemMerge

    // TODO: ShootBow

    // TODO: EntityHeal

    // TODO: Transform

    // TODO: Teleport


    { $event:ident, $original_func_ident:ident } => {
        compile_error!(concat!("Unknown event `", stringify!($event), "`"));
        event_trigger_inner!{ $trigger_func_ident, $original_func_ident }
    }
}


macro event_trigger_inner {
    ( $trigger_func_ident:ident, $original_func_ident:ident, { $( $arg:expr ),* } ) => {

        #[no_mangle]
        extern "C" fn $trigger_func_ident(_ : ::lldf_bind::__private::Calling_an_event_trigger_is_not_allowed) {
            $original_func_ident(
                unsafe{ *(::lldf_bind::core::mem::transmute_unchecked::<_, &_>(&())) }
                $( , $arg )*
            );
        }

    }
}
