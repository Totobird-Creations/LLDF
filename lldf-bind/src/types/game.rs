use crate::prelude::*;
use crate::bind::DFOpaqueValue;


pub struct Game {
    _unconstructable : ()
}

impl Game {

    // TODO: Move to `PlayerSel` as constructor.
    #[inline(always)]
    pub fn all_players() -> PlayerSel { unsafe {
        let uuids = DF_GAMEVALUE__PlotPlayerUUIDs_Default();
        PlayerSel { uuids }
    } }

    // TODO: Move to `PlayerSel` as constructor.
    #[inline(always)]
    pub fn player_by_uuid<T : AsRef<String>>(uuid : T) -> PlayerSel { unsafe {
        DF_ACTION__SelectObject_PlayerName(uuid.as_ref() as *const _ as *const _);
        let uuids = DF_GAMEVALUE__SelectionTargetUUIDs_Default();
        DF_ACTION__SelectObject_Reset();
        PlayerSel { uuids }
    } }

}

/// `SET_VARIABLE` / `Variable Setting`
impl Game {

    // TODO: SetPersistentData

    // TODO: GetPersistentData

    // TODO: RemovePersistentData

}

/// `GAME_ACTION` / `Entity Spawning`
impl Game {

    // TODO: SpawnMob

    // TODO: SpawnItem

    // TODO: SpawnVehicle

    // TODO: SpawnExpOrb

    // TODO: Explosion

    // TODO: SpawnTNT

    // TODO: SpawnFangs

    // TODO: Firework

    // TODO: LaunchProj

    // TODO: Lightning

    // TODO: SpawnPotionCloud

    // TODO: FallingBlock

    // TODO: SpawnArmourStand

    // TODO: SpawnCrystal

    // TODO: SpawnEnderEye

    // TODO: ShulkerBullet

    // TODO: SpawnTextDisplay

    // TODO: SpawnItemDisp

    // TODO: SpawnBlockDisp

    // TODO: SpawnInteraction

}

/// `GAME_ACTION` / `Block Manipulation`
impl Game {

    // TODO: SetBlock

    // TODO: set_block_and_update

    // TODO: SetRegion

    // TODO: SetBiome

    // TODO: CloneRegion

    // TODO: BreakBlock

    // TODO: SetBlockData

    // TODO: RandomTickBlock

    // TODO: BoneMealBlock

    // TODO: GenerateTree

    // TODO: SetBlockGrowth

    // TODO: FillContainer

    // TODO: SetContainer

    // TODO: SetItemInSlot

    // TODO: ReplaceItems

    // TODO: RemoveItems

    // TODO: ClearItems

    // TODO: ClearContainer

    // TODO: SetContainerName

    // TODO: LockContainer

    // TODO: ChangeSign

    // TODO: SignColour

    // TODO: SetHead

    // TODO: SetFurnaceSpeed

    // TODO: SetCampfireItem

    // TODO: SetLecternBook

    // TODO: SetBrushableItem

    // TODO: WriteTransaction

    // TODO: ApplyTransaction

}

/// `SET_VARIABLE` / `World Actions`
impl Game {

    // TODO: GetBlockMaterial

    // TODO: GetBlockData

    // TODO: GetAllBlockData

    // TODO: GetBlockGrowth

    // TODO: GetBlockPower

    // TODO: GetLightLevel

    // TODO: GetSignText

    // TODO: GetContainerName

    // TODO: GetContainerLock

    // TODO: GetContainerContents

    // TODO: GetLecternBook

    // TODO: GetLecternPage

    // TODO: GetBlockDropItems

}

/// `GAME_ACTION` / `Event Manipulation`
impl Game {

    // TODO: CancelEvent

    // TODO: UncancelEvent

    // TODO: SetEventDamage

    // TODO: SetEventHeal

    // TODO: SetEventXP

    // TODO: SetEventProj

    // TODO: SetEventSound

    // TODO: SetExhaustion

}

/// `GAME_ACTION` / `Settings`
impl Game {

    // TODO: BlockDropsOn

    // TODO: BlockDropsOff

}

/// `GAME_ACTION` / `Web Manipulation`
impl Game {

    // TODO: SendWebRequest

    // TODO: DiscordWebhook

}

/// `IF_GAME`
impl Game {

    // TODO: BlockEquals

    // TODO: BlockIsPowered

    // TODO: ContainerHasItem

    // TODO: ContainerHasAllItems

    // TODO: ContainerHasRoomForItem

    // TODO: SignContainsText

    // TODO: GameHasPlayer

    // TODO: EventBlockEquals

    // TODO: EventItemEquals

    // TODO: CommandEquals

    // TODO: CommandArgumentEquals

    // TODO: AttackIsCritical

    // TODO: EventIsCancalled

    // TODO: LocationIsInBlock

    // TODO: IsChunkLoaded

}

/// `Game Value` / `Plot Values`
impl Game {

    // TODO: player_count

    // TODO: plot_cpu

    // TODO: server_tps

    // TODO: now

    // TODO: plot_id

    // TODO: plot_size

    // TODO: elapsed

    // TODO: active_transactions

}





extern "C" {

    fn DF_GAMEVALUE__PlotPlayerUUIDs_Default( ) -> List<Uuid>;
    fn DF_GAMEVALUE__SelectionTargetUUIDs_Default( ) -> List<Uuid>;

    fn DF_ACTION__SelectObject_PlayerName( target : *const DFOpaqueValue ) -> ();

    fn DF_ACTION__SelectObject_Reset( ) -> ();

}
