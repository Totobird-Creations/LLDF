use crate::prelude::*;


/// A selection consisting of any number of non-player entities.
#[repr(transparent)]
pub struct EntitySel {
    pub(in super::super) uuids : List<String>
}

#[allow(deprecated)]
impl EntitySel {}

/// `ENTITY_ACTION` / `Statistics`
impl EntitySel {

    // TODO: Heal

    // TODO: SetHealth

    // TODO: SetAbsorption

    // TODO: SetMaxHealth

    // TODO: Damage

    // TODO: SetFireTicks

    // TODO: SetFreezeTicks

    // TODO: SetInvulTicks

    // TODO: GivePotion

    // TODO: RemovePotion

    // TODO: ClearPotions

    // TODO: SetAge

    // TODO: SetFallDistance

    // TODO: SetCreeperFuse

    // TODO: SetCreeperPower

    // TODO: SetCloudRadius

    // TODO: SetVillagerExp

    // TODO: SetWitherInvul

    // TODO: SetHorseJump

    // TODO: SetPickupDelay

    // TODO: SetFishingTime

    // TODO: SetWardenAnger

    // TODO: SetPandaSadTicks

    // TODO: SetArrowDamage

    // TODO: SetArrowPierce

    // TODO: HealthAttribute

    // TODO: CombatAttribute

    // TODO: KBAttribute

    // TODO: MovementAttribute

    // TODO: FallingAttribute

    // TODO: MiscAttribute

}

/// `ENTITY_ACTION` / `Appearance`
impl EntitySel {

    // TODO: MobDisguise

    // TODO: PlayerDisguise

    // TODO: BlockDisguise

    // TODO: Undisguise

    // TODO: SetGlowing

    // TODO: SetDyeColour

    // TODO: SetFishPattern

    // TODO: SetRabbitType

    // TODO: SetCatType

    // TODO: MooshroomType

    // TODO: SetFoxType

    // TODO: SetParrotColour

    // TODO: SetHorsePattern

    // TODO: SetAxolotlColour

    // TODO: SetLlamaColour

    // TODO: SetFrogType

    // TODO: SetVillagerBiome

    // TODO: SnowmanPumpkin

    // TODO: SetEndermanBlock

    // TODO: SetMinecartBlock

    // TODO: ArmourStandParts

    // TODO: SetBeeNectar

    // TODO: ProjectileItem

    // TODO: SetVisualFire

    // TODO: SendAnimation

    // TODO: AttackAnimation

    // TODO: ArmourStandPose

    // TODO: SetPose

    // TODO: SetFoxLeaping

    // TODO: SetArmsRaised

    // TODO: SetCatResting

    // TODO: SetGlowSquidDark

    // TODO: SetShulkerPeek

}

/// `ENTITY_ACTION` / `Movement`
impl EntitySel {

    // TODO: Teleport

    // TODO: LaunchUp

    // TODO: LaunchFwd

    // TODO: LaunchToward

    // TODO: SetGliding

    // TODO: SetGravity

    // TODO: RideEntity

    // TODO: AttachLead

    // TODO: SetRotation

    // TODO: FaceLocation

    // TODO: SetVelocity

    // TODO: SetFriction

}

/// `ENTITY_ACTION` / `Settings`
impl EntitySel {

    // TODO: SetName

    // TODO: SetNameVisible

    // TODO: SetNameColour

    // TODO: SetAI

    // TODO: SetSilenced

    // TODO: SetDeathDrops

    // TODO: SetCollidable

    // TODO: SetInvulnerable

    // TODO: SetMobSitting

    // TODO: SetBaby

    // TODO: SetSize

    // TODO: SetSheepSheared

    // TODO: SetSaddle

    // TODO: SetCarryingChest

    // TODO: ArmourStandSlots

    // TODO: SetMarker

    // TODO: SetAngry

    // TODO: SetRearing

    // TODO: SetRiptiding

    // TODO: CreeperCharged

    // TODO: SetInvisible

    // TODO: SetGoatScreaming

    // TODO: SetGoatHorns

    // TODO: Tame

    // TODO: EndCrystalBeam

    // TODO: SetPandaGene

    // TODO: SetPandaOnBack

    // TODO: SetProfession

    // TODO: SetProjSource

    // TODO: SetPersistent

    // TODO: InteractionSize

    // TODO: InteractResponse

    // TODO: SetBeeStinger

    // TODO: SetArrowNoClip

}

/// `ENTITY_ACTION` / `AI`
impl EntitySel {

    // TODO: SetCelebrating

    // TODO: SetTarget

    // TODO: MoveToLoc

    // TODO: Jump

    // TODO: Ram

    // TODO: FrogEat

    // TODO: SheepEat

    // TODO: IgniteCreeper

    // TODO: Explode

    // TODO: FoxSleeping

    // TODO: SetDragonPhase

    // TODO: SetBulletTarget

    // TODO: UseItem

    // TODO: SetAllayDancing

    // TODO: SetVexCharging

    // TODO: SnifferState

    // TODO: SetPandaRolling

}

/// `ENTITY_ACTION` / `Display Entity`
impl EntitySel {

    // TODO: DisplayViewRange

    // TODO: DisplayBillboard

    // TODO: DisplayShadow

    // TODO: DisplayBrightness

    // TODO: DispInterpolation

    // TODO: DisplayCullingSize

    // TODO: DispTPDuration

    // TODO: TDisplayText

    // TODO: TDisplayLineWidth

    // TODO: TDisplayOpacity

    // TODO: TDisplayAlign

    // TODO: TDisplayShadow

    // TODO: TDisplaySeeThru

    // TODO: TDisplayBackground

    // TODO: DisplayGlowColour

    // TODO: IDisplayItem

    // TODO: IDisplayModelType

    // TODO: BDisplayBlock

    // TODO: DisplayMatrix

}

/// `ENTITY_ACTION` / `Miscellaneous`
impl EntitySel {

    // TODO: Remove

    // TODO: SetEquipment

    // TODO: SetArmour

    // TODO: LaunchProj

    // TODO: ShearSheep

    // TODO: SetCustomTag

    // TODO: GetCustomTag

    // TODO: GetAllEntityTags

    // TODO: RemoveCustomTag

    // TODO: SetItem

    // TODO: SetArrowHitSound

    // TODO: SetDigging

}

/// `IF_ENTITY`
impl EntitySel {

    // TODO: IsType

    // TODO: NameEquals

    // TODO: StandingOn

    // TODO: IsGrounded

    // TODO: IsNear

    // TODO: IsRiding

    // TODO: HasPotion

    // TODO: IsMob

    // TODO: IsProj

    // TODO: IsVehicle

    // TODO: IsItem

    // TODO: Exists

    // TODO: HasCustomTag

    // TODO: IsSheared

}

/// `SELECT_OBJECT` / `Creating Selections`
impl EntitySel {

    // TODO: EntityName

    // TODO: AllEntities

    // TODO: Invert

}

/// `SELECT_OBJECT` / `Selection Filters`
impl EntitySel {

    // TODO: FilterRandom

    // TODO: FilterDistance

    // TODO: FilterSort

    // TODO: FilterRay

}

/// `Game Value` / `Statistical Values`
impl EntitySel {

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

    // TODO: item_usage_progress

    // TODO: flight_speed

    // TODO: walk_speed

    // TODO: width

    // TODO: height

}

/// `Game Value` / `Locational Values`
impl EntitySel {

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
impl EntitySel {

    // TODO: mainhand_item

    // TODO: offhand_item

    // TODO: armour_items

    // TODO: hotbar_items

    // TODO: inventory_items

    // TODO: saddle_item

    // TODO: entity_item

}

/// `Game Value` / `Informational Values`
impl EntitySel {

    // TODO: name

    // TODO: uuid

    // TODO: entity_type

    // TODO: potions

    // TODO: vehicle_uuid

    // TODO: passenger_uuids

    // TODO: leading_uuid

    // TODO: lead_uuids

    // TODO: targeted_uuid

    // TODO: shooter_uuid

    // TODO: pose

}

/// `Game Value` / `Plot Values`
impl PlayerSel {

    // TODO: len

    // TODO: uuids

    // TODO: names

}


unsafe impl DFSel for EntitySel {}
