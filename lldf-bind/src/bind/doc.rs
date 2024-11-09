pub macro dfdoc {
    ( { PlayerEvent / ClickContainerSlot $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ClickContainerSlot`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// clicks a slot in a container.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / CloseInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: CloseInv`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player closes an inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerEvent / StartFly $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: StartFly`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player starts flying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / BreakBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: BreakBlock`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player breaks a block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / StartSprint $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: StartSprint`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player starts sprinting.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Cancelling this event will
        /// only result in the sprint
        /// particles being hidden.
        /// 
        /// ##### Additional Info
        /// Client side particles are
        /// not hidden!
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / MobKillPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: MobKillPlayer`
        /// 
        /// ##### Description
        /// Executes code when
        /// a mob kills a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Teleport $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Teleport`
        /// 
        /// ##### Description
        /// Executes code when
        /// a player is teleported.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ShootBow $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ShootBow`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// fires an arrow with a bow.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / StopFly $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: StopFly`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player stops flying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / TameEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: TameEntity`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// tames a mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / LeftClick $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: LeftClick`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// left clicks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Does not execute on left
        /// clicks where a player deals
        /// damage.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / PlayerTakeDmg $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PlayerTakeDmg`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player takes damage.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ProjHit $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ProjHit`
        /// 
        /// ##### Description
        /// Executes code when a projectile
        /// launched by a player hits a block,
        /// an entity, or another player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / KillPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: KillPlayer`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// kills another player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / VehicleJump $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: VehicleJump`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player presses the jump key
        /// while riding a vehicle
        /// or other entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ClickInvSlot $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ClickInvSlot`
        /// 
        /// ##### Description
        /// Executes code when a player clicks
        /// a slot inside their inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Respawn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Respawn`
        /// 
        /// ##### Description
        /// Executes code when
        /// a player respawns.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / SwapHands $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: SwapHands`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// swaps an item or items between
        /// their main hand and off hand.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / PackLoad $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PackLoad`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player finishes loading a
        /// plot resource pack.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerEvent / DamageEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: DamageEntity`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player damages an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Sneak $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Sneak`
        /// 
        /// ##### Description
        /// Executes code when
        /// a player sneaks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Cancelling this event will
        /// not update what the sneaking
        /// player sees!
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / PlayerHeal $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PlayerHeal`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// regains health from any
        /// source.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ClickPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ClickPlayer`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// right clicks another player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Consume $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Consume`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// eats or drinks an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Death $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Death`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// dies, not as a result of another
        /// player or entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / PlaceBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PlaceBlock`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player places a block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Walk $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Walk`
        /// 
        /// ##### Description
        /// Executes code while
        /// a player is walking.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / PickUpItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PickUpItem`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player picks up an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Dismount $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Dismount`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player dismounts a vehicle
        /// or other entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / CloudImbuePlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: CloudImbuePlayer`
        /// 
        /// ##### Description
        /// Executes code when an area of
        /// effect cloud applies its potion
        /// effect(s) to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Area of effect clouds that do
        /// not apply any potion effects
        /// will not trigger this event.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Leave $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Leave`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player leaves the plot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / DropItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: DropItem`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player drops an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / LeftClickPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: LeftClickPlayer`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// left clicks another player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ChangeSlot $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ChangeSlot`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// changes their hotbar slot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ClickEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ClickEntity`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// right clicks an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / HorseJump $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: HorseJump`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// causes a horse to jump.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ShootProjectile $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ShootProjectile`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// throws a projectile such
        /// as snowballs or eggs.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Move $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Move`
        /// 
        /// ##### Description
        /// Executes code when
        /// a player moves.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Unsneak $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Unsneak`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player stops sneaking.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Cancelling this event will
        /// not update what the sneaking
        /// player sees!
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Fish $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Fish`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// fishes an entity,
        /// player, or nothing.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / FallDamage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: FallDamage`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerEvent / BreakItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: BreakItem`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player breaks an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / LoopEvent $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: LoopEvent`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerEvent / PlayerResurrect $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PlayerResurrect`
        /// 
        /// ##### Description
        /// Executes code when
        /// a player resurrects with
        /// a totem of undying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / RightClick $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: RightClick`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// right clicks while looking at a
        /// block or holding an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Executes multiple times if a
        /// player holds down right click.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ClickMenuSlot $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ClickMenuSlot`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// clicks a slot in an inventory
        /// menu.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerEvent / Riptide $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Riptide`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// throws a riptide trident.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / KillMob $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: KillMob`
        /// 
        /// ##### Description
        /// Executes code when
        /// a player kills a mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Join $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Join`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player joins the plot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / EntityDmgPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: EntityDmgPlayer`
        /// 
        /// ##### Description
        /// Executes code when an
        /// entity damages a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / StopSprint $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: StopSprint`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player stops sprinting.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Cancelling this event will
        /// only result in the sprint
        /// particles being shown.
        /// 
        /// ##### Additional Info
        /// Client side particles are
        /// not shown!
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Jump $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Jump`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player jumps.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / LoadCrossbow $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: LoadCrossbow`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// loads a crossbow.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / ProjDmgPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: ProjDmgPlayer`
        /// 
        /// ##### Description
        /// Executes code when a projectile
        /// damages a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Command $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Command`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// types a command on the plot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Examples
        /// - "@command"
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerEvent / LeftClickEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: LeftClickEntity`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// left clicks an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / Exhaustion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: Exhaustion`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player gains exhaustion.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerEvent / PackDecline $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PackDecline`
        /// 
        /// ##### Description
        /// Executes code when a
        /// player declines a plot
        /// resource pack prompt.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerEvent / PlayerDmgPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerEvent: PlayerDmgPlayer`
        /// 
        /// ##### Description
        /// Executes code when a player
        /// damages another player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Function / Dynamic $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Function: Dynamic`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { Repeat / Adjacent $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Adjacent`
        /// 
        /// ##### Description
        /// Repeats code once for each
        /// block adjacent to a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / Path $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Path`
        /// 
        /// ##### Description
        /// Repeats code once for
        /// each interpolated point in
        /// a path of locations.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / Multiple $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Multiple`
        /// 
        /// ##### Description
        /// Repeats code multiple times.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / Grid $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Grid`
        /// 
        /// ##### Description
        /// Repeats code once for each
        /// block in a region in order:
        /// X → Z → Y. Iterates from the
        /// first to the second location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / While $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: While`
        /// 
        /// ##### Description
        /// Repeats code as long as a
        /// condition is true.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / Range $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Range`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { Repeat / ForEach $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: ForEach`
        /// 
        /// ##### Description
        /// Repeats code once for each
        /// index of a list.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / Sphere $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Sphere`
        /// 
        /// ##### Description
        /// Repeats code once for every
        /// evenly distributed sphere point.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / Forever $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Forever`
        /// 
        /// ##### Description
        /// Repeats code indefinitely.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The Control: Wait block can
        /// be used for a delay.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / Range $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: Range`
        /// 
        /// ##### Description
        /// Repeats code once for each
        /// number on a number line.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// To iterate in the negative direction,
        /// you must use a negative step size.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Repeat / ForEachEntry $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Repeat: ForEachEntry`
        /// 
        /// ##### Description
        /// Repeats code once per entry in
        /// a dictionary
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DispRotationEuler $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DispRotationEuler`
        /// 
        /// ##### Description
        /// Sets the left or right rotation of a
        /// display entity from 3 angles on
        /// each axis.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetParrotColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetParrotColor`
        /// 
        /// ##### Description
        /// Sets a parrot's color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Parrot
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / DispTranslation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DispTranslation`
        /// 
        /// ##### Description
        /// Sets the translation values
        /// of a display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / Remove $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Remove`
        /// 
        /// ##### Description
        /// Deletes an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetVelocity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetVelocity`
        /// 
        /// ##### Description
        /// Sets an entity's movement
        /// velocity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / TdispBackground $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: TdispBackground`
        /// 
        /// ##### Description
        /// Sets the background color
        /// and opacity of a text display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Text Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / DisplayCullingSize $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayCullingSize`
        /// 
        /// ##### Description
        /// Sets the culling width
        /// and height of a
        /// display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetGlowSquidDark $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetGlowSquidDark`
        /// 
        /// ##### Description
        /// Sets the number of ticks a
        /// glow squid will stop glowing for.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Glow Squid
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / HideName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: HideName`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetFrogType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFrogType`
        /// 
        /// ##### Description
        /// Sets a frog's color type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Frog
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / DispRotAxisAngle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DispRotAxisAngle`
        /// 
        /// ##### Description
        /// Sets the left or right rotation of
        /// a display entity from axis-angle
        /// rotation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / Damage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Damage`
        /// 
        /// ##### Description
        /// Damages a mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetSheepSheared $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetSheepSheared`
        /// 
        /// ##### Description
        /// Sets whether a sheep
        /// has its wool.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Sheep
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetMobSitting $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetMobSitting`
        /// 
        /// ##### Description
        /// Sets whether an entity
        /// is sitting.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Camel
        /// - Cat
        /// - Fox
        /// - Parrot
        /// - Wolf
        /// 
        /// ##### Additional Info
        /// Works on both tamed
        /// and untamed mobs.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetAxolotlColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetAxolotlColor`
        /// 
        /// ##### Description
        /// Sets an axolotl's color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Axolotl
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SendAnimation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SendAnimation`
        /// 
        /// ##### Description
        /// Makes a mob perform
        /// an animation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / GetAllEntityTags $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: GetAllEntityTags`
        /// 
        /// ##### Description
        /// Gets all tags registered
        /// on an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DisableGlowing $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisableGlowing`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetWardenAnger $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetWardenAnger`
        /// 
        /// ##### Description
        /// Sets the anger level
        /// of a Warden.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Warden
        /// 
        /// ##### Additional Info
        /// If the anger level reaches
        /// 80, the warden will actively
        /// purse the target.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetHorsePattern $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetHorsePattern`
        /// 
        /// ##### Description
        /// Sets a horse's color and pattern.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Horse
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / Heal $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Heal`
        /// 
        /// ##### Description
        /// Restores a mob's health.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetPandaSadTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetPandaSadTicks`
        /// 
        /// ##### Description
        /// Makes a panda sad for
        /// the specified duration.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Panda
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetAI $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetAI`
        /// 
        /// ##### Description
        /// Sets whether an entity is
        /// sentient and/or affected
        /// by physics.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / Kbattribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Kbattribute`
        /// 
        /// ##### Description
        /// Sets one of the entity's knockback-related
        /// attributes such as knockback resistance.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / MovementAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: MovementAttribute`
        /// 
        /// ##### Description
        /// Sets one of the entity's movement-related
        /// attributes, such as walking speed
        /// and jump height.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetRiptiding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetRiptiding`
        /// 
        /// ##### Description
        /// Sets whether an entity
        /// is riptiding.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetArrowNoClip $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetArrowNoClip`
        /// 
        /// ##### Description
        /// Sets whether an arrow
        /// will pass through blocks
        /// and through entities.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Arrows, Tridents
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / FallingAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: FallingAttribute`
        /// 
        /// ##### Description
        /// Sets one of the entity's falling-related
        /// attributes, such as gravity
        /// and fall damage multiplier.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Additional Info
        /// This is a temporary code action
        /// to allow early migration.
        /// Type /migration for more info.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetProjSource $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetProjSource`
        /// 
        /// ##### Description
        /// Sets the projectile source of
        /// a projectile (or removes it).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Projectile
        /// - Area Effect Cloud
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetFoxLeaping $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFoxLeaping`
        /// 
        /// ##### Description
        /// Sets whether a fox appears
        /// to be leaping.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Fox
        /// 
        /// ##### Additional Info
        /// Does not affect movement.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetItemOwner $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetItemOwner`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetPandaGene $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetPandaGene`
        /// 
        /// ##### Description
        /// Sets the gene of a panda.
        /// This affects their behavior
        /// and appearance.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Panda
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetDyeColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetDyeColor`
        /// 
        /// ##### Description
        /// Sets a mob's dye color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Sheep
        /// - Shulker
        /// - Dog (collar)
        /// - Cat (collar)
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / LaunchUp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: LaunchUp`
        /// 
        /// ##### Description
        /// Launches an entity up or down.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A negative launch power will
        /// launch the entity down.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetMaxHealth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetMaxHealth`
        /// 
        /// ##### Description
        /// Sets an entity's maximum
        /// health.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetAge $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetAge`
        /// 
        /// ##### Description
        /// Sets an animal's age.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Breedable
        /// 
        /// ##### Additional Info
        /// 0: Target is an adult that
        /// can breed. Age stays at 0.
        /// 
        /// ##### Additional Info
        /// -1 and below: Target is a
        /// baby. Age goes up each tick.
        /// 
        /// ##### Additional Info
        /// 1 and above: Target is an
        /// adult that can't breed. Age
        /// goes down each tick.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetFishingTime $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFishingTime`
        /// 
        /// ##### Description
        /// Sets the time until a fish
        /// starts to approach a
        /// fishing hook.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Fishing Hook
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / EndCrystalBeam $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: EndCrystalBeam`
        /// 
        /// ##### Description
        /// Sets the location an end
        /// crystal points its beam at.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// To remove the beam, do not
        /// specify a location.
        /// 
        /// ##### Additional Info
        /// The target location is
        /// rounded to the nearest
        /// block.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetName`
        /// 
        /// ##### Description
        /// Sets an entity's custom name.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / FrogEat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: FrogEat`
        /// 
        /// ##### Description
        /// Makes a frog try to eat the
        /// specified mob or player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Frog
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DisplayBrightness $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayBrightness`
        /// 
        /// ##### Description
        /// Sets the brightness
        /// of a display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetProfession $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetProfession`
        /// 
        /// ##### Description
        /// Sets a villager's profession.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Villager
        /// - Zombie Villager
        /// 
        /// ##### Additional Info
        /// If the villager has no
        /// experience yet, it is given
        /// 1 experience to prevent
        /// unemployment.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / NoGravity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: NoGravity`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetArmsRaised $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetArmsRaised`
        /// 
        /// ##### Description
        /// Sets whether a mob has its
        /// arms raised.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Zombie
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / ClearPotions $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ClearPotions`
        /// 
        /// ##### Description
        /// Removes all active potion
        /// effects from an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / ArmorStandParts $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ArmorStandParts`
        /// 
        /// ##### Description
        /// Sets whether an armor stand has
        /// arms and a base plate.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Armor Stand
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetMoveSpeed $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetMoveSpeed`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetInvulnerable $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetInvulnerable`
        /// 
        /// ##### Description
        /// Sets whether an entity is
        /// invulnerable to damage.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// An invulnerable entity does not
        /// trigger damage events.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetNameVisible $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetNameVisible`
        /// 
        /// ##### Description
        /// Sets whether an entity's
        /// custom name is always
        /// displayed above them.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetFriction $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFriction`
        /// 
        /// ##### Description
        /// Changes the type of friction
        /// an entity experiences.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / ProjColl $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ProjColl`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / ArmorStandTags $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ArmorStandTags`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetPickupDelay $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetPickupDelay`
        /// 
        /// ##### Description
        /// Sets the number of ticks a
        /// dropped item cannot be
        /// picked up for.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If set to 0 upon spawning,
        /// the item can be picked up
        /// immediately.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetTarget $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetTarget`
        /// 
        /// ##### Description
        /// Instructs a mob's AI to target
        /// a specific mob or player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { EntityAction / DropItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DropItems`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / MiscAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: MiscAttribute`
        /// 
        /// ##### Description
        /// Sets one of the entity's miscellaneous
        /// attributes such as follow range and
        /// spawn reinforcements.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / TdisplayShadow $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: TdisplayShadow`
        /// 
        /// ##### Description
        /// Sets whether the text in
        /// a text display has
        /// shadow or not.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Text Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetCreeperPower $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCreeperPower`
        /// 
        /// ##### Description
        /// Sets a creeper's explosion power.
        /// This affects the damage and area
        /// of effect.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Creeper
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetMarker $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetMarker`
        /// 
        /// ##### Description
        /// Sets whether an armor stand
        /// is a marker.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / RemoveCustomTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: RemoveCustomTag`
        /// 
        /// ##### Description
        /// Removes a custom tag
        /// from an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetNameVisible $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetNameVisible`
        /// 
        /// ##### Description
        /// Sets whether an entity's
        /// custom name is always
        /// displayed above them.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A mob's custom name is
        /// always visible when the
        /// cursor is placed on them.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetInvulTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetInvulTicks`
        /// 
        /// ##### Description
        /// Sets the currently
        /// remaining ticks until an
        /// entity can next be hurt.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// This value is set to 10
        /// upon taking damage.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetShulkerPeek $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetShulkerPeek`
        /// 
        /// ##### Description
        /// Sets how far a shulker
        /// should peek up to.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Shulker
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetAbsorption $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetAbsorption`
        /// 
        /// ##### Description
        /// Sets an entity's absorption
        /// health (golden hearts).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Additional Info
        /// The target does not need to
        /// have an absorption effect.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetPose $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetPose`
        /// 
        /// ##### Description
        /// Changes the pose of an entity.
        /// This affects their animations
        /// and/or hitbox, depending on
        /// the pose and entity type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetRearing $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetRearing`
        /// 
        /// ##### Description
        /// Sets whether a horse is
        /// standing on its hind legs.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Horse
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / CreeperCharged $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: CreeperCharged`
        /// 
        /// ##### Description
        /// Sets whether a creeper
        /// has the charged effect.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Creeper
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetFireTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFireTicks`
        /// 
        /// ##### Description
        /// Sets the remaining time an entity is on
        /// fire for.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// 0 ticks mean the target is not on fire.
        /// 
        /// ##### Additional Info
        /// Ghasts, magma cubes, vexes, withers,
        /// wither skulls, tnt, falling blocks, and
        /// shulker bullets cannot be set on fire.
        /// 
        /// ##### Additional Info
        /// Blazes do not rely on fire ticks to
        /// display the fire effect.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetCloudRadius $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCloudRadius`
        /// 
        /// ##### Description
        /// Sets an area of effect cloud's
        /// radius and shrinking speed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / CombatAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: CombatAttribute`
        /// 
        /// ##### Description
        /// Sets one of the entity's combat-related
        /// attributes such as attack damage
        /// and attack speed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetGravity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetGravity`
        /// 
        /// ##### Description
        /// Sets whether an entity
        /// is affected by gravity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetName`
        /// 
        /// ##### Description
        /// Sets an entity's custom name.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / Jump $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Jump`
        /// 
        /// ##### Description
        /// Causes a mob to jump.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DispTPDuration $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DispTPDuration`
        /// 
        /// ##### Description
        /// Sets how long a display entity takes
        /// to visually move to its destination
        /// when it teleports.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetWitherInvul $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetWitherInvul`
        /// 
        /// ##### Description
        /// Sets the remaining ticks of
        /// invulnerability a wither has.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Wither
        /// 
        /// ##### Additional Info
        /// If AI is enabled, this value
        /// decreases over time. At 0,
        /// an explosion occurs.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / BdisplayBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: BdisplayBlock`
        /// 
        /// ##### Description
        /// Sets the displayed block
        /// of a block display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Block Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetFreezeTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFreezeTicks`
        /// 
        /// ##### Description
        /// Sets an entity's current
        /// freeze ticks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / Silence $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Silence`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / TdisplaySeeThru $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: TdisplaySeeThru`
        /// 
        /// ##### Description
        /// Sets whether a text display
        /// is visible through walls
        /// or not.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Text Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetArrowPierce $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetArrowPierce`
        /// 
        /// ##### Description
        /// Sets how many targets an
        /// arrow can pierce through.
        /// A pierce of 1 can hit
        /// up to 2 entities.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Arrows
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetGliding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetGliding`
        /// 
        /// ##### Description
        /// Sets whether an entity
        /// is gliding.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Additional Info
        /// The entity must be
        /// wearing an elytra.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetRotation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetRotation`
        /// 
        /// ##### Description
        /// Changes an entity's pitch and
        /// yaw without teleporting it.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DisplayShadow $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayShadow`
        /// 
        /// ##### Description
        /// Sets the shadow properties
        /// of a display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / InteractResponse $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: InteractResponse`
        /// 
        /// ##### Description
        /// Sets whether an interaction
        /// entity has response when
        /// interacting with it.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Interaction Entity
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetPandaRolling $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetPandaRolling`
        /// 
        /// ##### Description
        /// Sets whether a panda is
        /// rolling or not.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Panda
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / UseItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: UseItem`
        /// 
        /// ##### Description
        /// Forces a mob to use held items
        /// such as bow or spyglass.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetFishPattern $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFishPattern`
        /// 
        /// ##### Description
        /// Sets a tropical fish's
        /// color and pattern.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Tropical Fish
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / RideEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: RideEntity`
        /// 
        /// ##### Description
        /// Mounts an entity on top of
        /// another entity or player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / DisplayMatrix $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayMatrix`
        /// 
        /// ##### Description
        /// Sets the affine transformation
        /// matrix of a display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / NoDrops $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: NoDrops`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / DispInterpolation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DispInterpolation`
        /// 
        /// ##### Description
        /// Sets the interpolation
        /// properties of a display
        /// entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SnifferState $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SnifferState`
        /// 
        /// ##### Description
        /// Forces a sniffer to perform
        /// a specific action.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Sniffer
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetHandItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetHandItem`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / EnableGlowing $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: EnableGlowing`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetEndermanBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetEndermanBlock`
        /// 
        /// ##### Description
        /// Set an enderman's held block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Enderman
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / Teleport $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Teleport`
        /// 
        /// ##### Description
        /// Teleports an entity to a
        /// specified location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / LaunchToward $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: LaunchToward`
        /// 
        /// ##### Description
        /// Launches an entity toward or away
        /// from a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A negative launch power will launch
        /// the entity away from the location.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetArmor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetArmor`
        /// 
        /// ##### Description
        /// Sets a mob's armor items.
        /// Place the armor in slots 1-4
        /// of the chest, with 1 being the
        /// helmet and 4 being the boots.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Additional Info
        /// Any block or item can render
        /// on the target's head if placed
        /// in the 'helmet' slot.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DisplayGlowColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayGlowColor`
        /// 
        /// ##### Description
        /// Sets the glowing color
        /// of a display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Item Display
        /// - Block Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetVisualFire $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetVisualFire`
        /// 
        /// ##### Description
        /// Sets whether an entity
        /// should appear on fire.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The affected entity's
        /// fire ticks won't change
        /// and won't take any damage.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetAgeSize $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetAgeSize`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / GetCustomTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: GetCustomTag`
        /// 
        /// ##### Description
        /// Gets the value of a custom
        /// entity tag.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / InteractionSize $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: InteractionSize`
        /// 
        /// ##### Description
        /// Sets the hitbox size of
        /// an interaction entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Interaction Entity
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / FaceLocation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: FaceLocation`
        /// 
        /// ##### Description
        /// Rotates an entity to look
        /// toward a location without
        /// teleporting them.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / LSetArmor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: LSetArmor`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetCatType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCatType`
        /// 
        /// ##### Description
        /// Sets a cat's skin type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Cat
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetArrowDamage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetArrowDamage`
        /// 
        /// ##### Description
        /// Sets the base damage
        /// dealt by an arrow or trident.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Arrows, Tridents
        /// 
        /// ##### Additional Info
        /// Base Arrow Damage: 2
        /// 
        /// ##### Additional Info
        /// Base Thrown Trident Damage: 8
        /// 
        /// ##### Additional Info
        /// Total Arrow Damage: ||velocity|| x base
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetSaddle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetSaddle`
        /// 
        /// ##### Description
        /// Sets whether a mob wears
        /// a saddle.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Camel
        /// - Pig
        /// - Strider
        /// - Any Horse
        /// 
        /// ##### Additional Info
        /// For a custom saddle item,
        /// use 'Set Equipment Item'.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetBulletTarget $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetBulletTarget`
        /// 
        /// ##### Description
        /// Causes a shulker bullet to start
        /// targeting the provided entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DisplayScale $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayScale`
        /// 
        /// ##### Description
        /// Sets the scale of
        /// a display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / TdisplayLineWidth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: TdisplayLineWidth`
        /// 
        /// ##### Description
        /// Sets the maximum line width
        /// of a text display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Text Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / TdisplayAlign $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: TdisplayAlign`
        /// 
        /// ##### Description
        /// Sets the text alignment
        /// of a text display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Text Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / LaunchProj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: LaunchProj`
        /// 
        /// ##### Description
        /// Launches a projectile from a mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetDragonPhase $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetDragonPhase`
        /// 
        /// ##### Description
        /// Sets the behavior phase
        /// of an Ender Dragon.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Ender Dragon
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetLlamaColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetLlamaColor`
        /// 
        /// ##### Description
        /// Sets a llama's fur color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Llama
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetVillagerBiome $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetVillagerBiome`
        /// 
        /// ##### Description
        /// Sets the biome type of a
        /// villager. This affects their
        /// appearance only.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Villager
        /// - Zombie villager
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetCreeperFuse $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCreeperFuse`
        /// 
        /// ##### Description
        /// Sets the starting amount
        /// of ticks it takes for a
        /// creeper to explode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Creeper
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / EnableAI $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: EnableAI`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SetBaby $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetBaby`
        /// 
        /// ##### Description
        /// Sets whether an entity
        /// is a baby (permanently).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Breedable
        /// - Any Zombie
        /// - Piglin
        /// - Armor Stand
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / MooshroomType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: MooshroomType`
        /// 
        /// ##### Description
        /// Sets a mooshroom's skin
        /// type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Mooshroom
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetInvisible $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetInvisible`
        /// 
        /// ##### Description
        /// Sets whether an entity
        /// is invisible.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// - Item Frame
        /// - Armor Stand
        /// - Interaction Entity
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / DisplayBillboard $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayBillboard`
        /// 
        /// ##### Description
        /// Sets how a display entity
        /// is rotated with a
        /// player's view.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / NoProjColl $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: NoProjColl`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / SheepEat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SheepEat`
        /// 
        /// ##### Description
        /// Causes a sheep to
        /// eat grass.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Sheep
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetCatResting $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCatResting`
        /// 
        /// ##### Description
        /// Sets whether a cat appears
        /// to be lying down.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Cat
        /// 
        /// ##### Additional Info
        /// Does not affect movement.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / GivePotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: GivePotion`
        /// 
        /// ##### Description
        /// Gives one or more potion
        /// effects to an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetGoatHorns $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetGoatHorns`
        /// 
        /// ##### Description
        /// Sets which goat horns
        /// are shown or hidden.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Goat
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / Tame $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Tame`
        /// 
        /// ##### Description
        /// Tames and sets the owner
        /// of a tameable mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Wolf
        /// - Cat
        /// - Parrot
        /// - Llama
        /// - Any Horse
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetGlowing $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetGlowing`
        /// 
        /// ##### Description
        /// Sets whether this entity has
        /// a glowing outline that can
        /// be seen through blocks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetGoatScreaming $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetGoatScreaming`
        /// 
        /// ##### Description
        /// Sets whether a goat
        /// screams or not.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Goat
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetPandaOnBack $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetPandaOnBack`
        /// 
        /// ##### Description
        /// Sets whether a panda is
        /// laying on its back or not.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Panda
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetBeeStinger $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetBeeStinger`
        /// 
        /// ##### Description
        /// Sets whether a bee
        /// has its stinger.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Bee
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / IdisplayModelType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: IdisplayModelType`
        /// 
        /// ##### Description
        /// Sets the model type
        /// of an item display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Item Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetHealth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetHealth`
        /// 
        /// ##### Description
        /// Sets an entity's current
        /// health.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / MobDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: MobDisguise`
        /// 
        /// ##### Description
        /// Disguises an entity as a mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { EntityAction / BlockDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: BlockDisguise`
        /// 
        /// ##### Description
        /// Disguises an entity as a block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { EntityAction / SetMinecartBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetMinecartBlock`
        /// 
        /// ##### Description
        /// Sets the block shown inside
        /// a minecart. This does not
        /// affect its functionality.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Minecart
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / FoxSleeping $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: FoxSleeping`
        /// 
        /// ##### Description
        /// Causes a fox to start
        /// or stop sleeping.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Fox
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetCollidable $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCollidable`
        /// 
        /// ##### Description
        /// Sets whether a mob is able
        /// to collide with other entities.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Additional Info
        /// To disable collisions between
        /// mobs and players, both must
        /// have their collisions disabled.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / ArmorStandPose $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ArmorStandPose`
        /// 
        /// ##### Description
        /// Sets the rotation of an armor
        /// stand part.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Armor Stand
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / LaunchFwd $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: LaunchFwd`
        /// 
        /// ##### Description
        /// Launches an entity forward
        /// or backward.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A negative launch power will
        /// launch the entity backward.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetFallDistance $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFallDistance`
        /// 
        /// ##### Description
        /// Sets an entity's fall distance,
        /// affecting fall damage upon
        /// landing.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / MoveToLoc $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: MoveToLoc`
        /// 
        /// ##### Description
        /// Instructs a mob's AI to always
        /// pathfind to a certain location
        /// at a certain speed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { EntityAction / TdisplayOpacity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: TdisplayOpacity`
        /// 
        /// ##### Description
        /// Sets the text opacity
        /// of a text display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Text Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / IdisplayItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: IdisplayItem`
        /// 
        /// ##### Description
        /// Sets the displayed item
        /// of an item display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Item Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetEquipment $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetEquipment`
        /// 
        /// ##### Description
        /// Sets the item in one of the
        /// equipment slots (including
        /// horse items) of an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / AttackAnimation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: AttackAnimation`
        /// 
        /// ##### Description
        /// Makes a mob perform
        /// an attack animation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetSilenced $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetSilenced`
        /// 
        /// ##### Description
        /// Sets whether an entity will
        /// produce sound effects.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetBeeNectar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetBeeNectar`
        /// 
        /// ##### Description
        /// Sets if a bee has nectar
        /// on its body.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Bee
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / AttachLead $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: AttachLead`
        /// 
        /// ##### Description
        /// Attaches a lead to the target,
        /// held by an entity or lead knot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Additional Info
        /// Max. attach range is 10 blocks.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SnowmanPumpkin $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SnowmanPumpkin`
        /// 
        /// ##### Description
        /// Sets whether a snow golem
        /// is wearing a pumpkin.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Snow Golem
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetCustomTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCustomTag`
        /// 
        /// ##### Description
        /// Sets the value of or creates
        /// a custom tag value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / RemovePotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: RemovePotion`
        /// 
        /// ##### Description
        /// Removes one or more potion
        /// effects from an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ##### Additional Info
        /// Only the potion's type needs
        /// to match; amplifier and duration
        /// are disregarded.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / Gravity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Gravity`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / ShearSheep $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ShearSheep`
        /// 
        /// ##### Description
        /// Causes a sheep to
        /// be sheared.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Sheep
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / ArmorStandSlots $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ArmorStandSlots`
        /// 
        /// ##### Description
        /// Sets the possible interactions, such
        /// as adding or removing items, of an
        /// armor stand's slot(s).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Armor Stand
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetAllayDancing $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetAllayDancing`
        /// 
        /// ##### Description
        /// Sets whether an allay is
        /// dancing or not.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Allay
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetRabbitType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetRabbitType`
        /// 
        /// ##### Description
        /// Sets a rabbit's skin type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Rabbit
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / DisplayViewRange $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: DisplayViewRange`
        /// 
        /// ##### Description
        /// Sets the view range of a
        /// display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Display Entity
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetSize $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetSize`
        /// 
        /// ##### Description
        /// Sets the size of an entity.
        /// This may also affect its
        /// health and strength.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Slime
        /// - Magma Cube
        /// - Phantom
        /// - Pufferfish
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / NoAI $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: NoAI`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / ShowName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ShowName`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / PlayerDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: PlayerDisguise`
        /// 
        /// ##### Description
        /// Disguises an entity as a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// To get a skin use /item head &lt;player name&ht;
        /// OR /item mshead
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { EntityAction / SetAngry $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetAngry`
        /// 
        /// ##### Description
        /// Sets whether a mob is
        /// angry at players.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Bee
        /// - Wolf
        /// - Zombified Piglin
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetItem`
        /// 
        /// ##### Description
        /// Sets the item of
        /// an item entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Dropped Item
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / Explode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Explode`
        /// 
        /// ##### Description
        /// Causes an entity
        /// to explode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Creeper
        /// - Primed TNT
        /// - Firework
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetDigging $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetDigging`
        /// 
        /// ##### Description
        /// Makes a warden emerge
        /// or dig into the ground.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Warden
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / MoveTo $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: MoveTo`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / Undisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Undisguise`
        /// 
        /// ##### Description
        /// Removes an entity's disguise.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { EntityAction / SetArrowHitSound $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetArrowHitSound`
        /// 
        /// ##### Description
        /// Sets the sound an arrow
        /// plays whenever it lands.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Arrows, Tridents
        /// 
        /// ##### Additional Info
        /// The pitch and volume of
        /// the sound are ignored.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetDeathDrops $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetDeathDrops`
        /// 
        /// ##### Description
        /// Sets whether a mob drops
        /// their items when dead.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetPersistent $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetPersistent`
        /// 
        /// ##### Description
        /// Sets whether an item
        /// or a falling block will
        /// never despawn.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Dropped Item
        /// - Falling Block
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetVexCharging $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetVexCharging`
        /// 
        /// ##### Description
        /// Sets whether a vex is
        /// charging or not.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Vex
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / SetVillagerExp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetVillagerExp`
        /// 
        /// ##### Description
        /// Sets a villager's experience
        /// points, which affects their level.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Villager
        /// 
        /// ##### Additional Info
        /// Level requirements:
        /// ■ 0: Novice
        /// ■ 10: Apprentice
        /// ■ 70: Journeyman
        /// ■ 150: Expert
        /// ■ 250: Master
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / IgniteCreeper $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: IgniteCreeper`
        /// 
        /// ##### Description
        /// Ignites a creeper, causing
        /// it to explode after a fuse
        /// period.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Creeper
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetCelebrating $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCelebrating`
        /// 
        /// ##### Description
        /// Causes a mob to start
        /// or stop celebrating.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Raider
        /// - Piglin
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / ProjectileItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: ProjectileItem`
        /// 
        /// ##### Description
        /// Sets the item a projectile
        /// displays as.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Snowball
        /// - Egg
        /// - Small Fireball
        /// - Ghast Fireball
        /// - Ender Pearl
        /// - Experience Bottle
        /// - Eye of Ender
        /// 
        /// ##### Additional Info
        /// Does not work with air.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / TdisplayText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: TdisplayText`
        /// 
        /// ##### Description
        /// Sets the displayed text
        /// of a text display.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Text Display
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { EntityAction / SetHorseJump $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetHorseJump`
        /// 
        /// ##### Description
        /// Sets a horse's jump strength.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Horse
        /// - Donkey
        /// - Mule
        /// - Llama
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetNameColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetNameColor`
        /// 
        /// ##### Description
        /// Sets the color an entity's
        /// name tag appears in.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { EntityAction / Unsilence $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Unsilence`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityAction / HealthAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: HealthAttribute`
        /// 
        /// ##### Description
        /// Sets one of the entity's health-related
        /// attributes such as max health
        /// and armor defense points.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Any Mob
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetCarryingChest $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetCarryingChest`
        /// 
        /// ##### Description
        /// Sets whether a mob carries
        /// a chest, which allows its
        /// inventory to be accessed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Donkey
        /// - Mule
        /// - Llama
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / Ram $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: Ram`
        /// 
        /// ##### Description
        /// Makes a goat ram the
        /// specified mob or player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Goat
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityAction / SetFoxType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityAction: SetFoxType`
        /// 
        /// ##### Description
        /// Sets a fox's fur type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Fox
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { SetVariable / String $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: String`
        /// 
        /// ##### Description
        /// Sets a variable to a string, or combines
        /// multiple values into one string.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// All values will be converted to string.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleType`
        /// 
        /// ##### Description
        /// Sets a particle effect's type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemEnchants $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemEnchants`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / ClearItemTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ClearItemTag`
        /// 
        /// ##### Description
        /// Removes all item custom tags.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / PurgeVars $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: PurgeVars`
        /// 
        /// ##### Description
        /// Clears all variables with names
        /// that match the given text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftAllAxes $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftAllAxes`
        /// 
        /// ##### Description
        /// Shifts a location's coordinates
        /// on the X, Y, and Z axes.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetParticleMat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleMat`
        /// 
        /// ##### Description
        /// Gets a particle effect's particle
        /// display material.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleSprd $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleSprd`
        /// 
        /// ##### Description
        /// Sets a particle effect's horizontal
        /// and vertical spread.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AbsoluteValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AbsoluteValue`
        /// 
        /// ##### Description
        /// Makes a number positive
        /// if it is negative.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AppendValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AppendValue`
        /// 
        /// ##### Description
        /// Adds a value to the end of a list
        /// variable.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Appending a list will result in
        /// the list itself being appended
        /// to the list (a nested list).
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: `
        /// 
        /// ##### Description
        /// Sets a variable to the remainder
        /// after dividing two numbers with
        /// a whole quotient.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftOnVector $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftOnVector`
        /// 
        /// ##### Description
        /// Shifts a location along a
        /// vector.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemAttribute`
        /// 
        /// ##### Description
        /// Get an attribute's
        /// multiplier for a specific slot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ClearDict $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ClearDict`
        /// 
        /// ##### Description
        /// Removes all entries from
        /// a dictionary.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: `
        /// 
        /// ##### Description
        /// Sets a variable to the sum of
        /// the given numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftRotation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftRotation`
        /// 
        /// ##### Description
        /// Rotates a location by shifting its pitch
        /// (up/down) or yaw (left/right) value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: `
        /// 
        /// ##### Description
        /// Sets a variable to the difference
        /// between the given numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemName`
        /// 
        /// ##### Description
        /// Gets an item's name.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If the item is unnamed, a &lt;translate&ht;
        /// component is used.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemRarity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemRarity`
        /// 
        /// ##### Description
        /// Gets an item's rarity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Item rarity determines the
        /// item's default name color.
        /// 
        /// ##### Additional Info
        /// An item's rarity is determined
        /// by its enchantments and type.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / MultiplyVector $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: MultiplyVector`
        /// 
        /// ##### Description
        /// Multiplies a vector's length
        /// by a number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: `
        /// 
        /// ##### Description
        /// Sets a variable to the quotient
        /// of the given numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetSignText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetSignText`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / Bitwise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Bitwise`
        /// 
        /// ##### Description
        /// Sets a variable to the result
        /// of a bitwise operation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Input values are treated
        /// as whole numbers. Decimal
        /// value is ignored.
        /// 
        /// ##### Additional Info
        /// Numbers are stored as
        /// 64-bit signed integers.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetLecternPage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetLecternPage`
        /// 
        /// ##### Description
        /// Gets the displayed page
        /// number of a Lectern.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ParseX $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ParseX`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / ShiftOnAxis $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftOnAxis`
        /// 
        /// ##### Description
        /// Shifts the X, Y, or Z coordinate
        /// of a location on its axis.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ParseY $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ParseY`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / VectorBetween $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: VectorBetween`
        /// 
        /// ##### Description
        /// Sets a variable to the vector
        /// between two locations.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ParseZ $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ParseZ`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetVectorComp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetVectorComp`
        /// 
        /// ##### Description
        /// Gets a vector's X, Y, or Z
        /// component.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: `
        /// 
        /// ##### Description
        /// Sets a variable to a value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RmText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RmText`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / AddItemAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AddItemAttribute`
        /// 
        /// ##### Description
        /// Adds an attribute modifier to the
        /// item, which is active in a certain
        /// equipment slot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetCenterLoc $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetCenterLoc`
        /// 
        /// ##### Description
        /// Finds an average position (center)
        /// of the given locations.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AlignLoc $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AlignLoc`
        /// 
        /// ##### Description
        /// Aligns a location to the center
        /// or corner of the block it is in.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetSoundVolume $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetSoundVolume`
        /// 
        /// ##### Description
        /// Gets a sound's volume.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Sound volume decreases based
        /// on distance to the sound's origin.
        /// 
        /// ##### Additional Info
        /// The maximum distance to hear a
        /// sound is equal to 16.0 × volume.
        /// 
        /// ##### Additional Info
        /// At volumes below 1.0, the sound's
        /// loudness at its origin decreases.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RandomNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RandomNumber`
        /// 
        /// ##### Description
        /// Sets a variable to a random
        /// number between two other
        /// numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ContainerName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ContainerName`
        /// 
        /// ##### Description
        /// Gets a container's name at
        /// a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Raycast $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Raycast`
        /// 
        /// ##### Description
        /// Raycasts from a location
        /// to the first intersection.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RotateAroundVec $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RotateAroundVec`
        /// 
        /// ##### Description
        /// Rotates a vector around
        /// another vector by an angle.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetParticleMotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleMotion`
        /// 
        /// ##### Description
        /// Gets a particle effect's particle
        /// motion.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleMotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleMotion`
        /// 
        /// ##### Description
        /// Sets a particle effect's particle
        /// motion and motion variation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Average $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Average`
        /// 
        /// ##### Description
        /// Sets a variable to the average
        /// of the given numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / WrapNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: WrapNumber`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SetY $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetY`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SetMapTexture $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetMapTexture`
        /// 
        /// ##### Description
        /// Sets a map item's texture to the
        /// image at the given URL.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Cannot be used more than 10
        /// times without delay. An additional
        /// usage is given every 5 seconds.
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { SetVariable / GetBlockData $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetBlockData`
        /// 
        /// ##### Description
        /// Gets a block state tag
        /// value at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Sets the variable to "0" if the
        /// tag is not present on the block.
        /// 
        /// ##### Additional Info
        /// Always returns a string value.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetX $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetX`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SortDict $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SortDict`
        /// 
        /// ##### Description
        /// Sorts a dictionary
        /// by its keys or values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetLecternBook $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetLecternBook`
        /// 
        /// ##### Description
        /// Gets the book on the
        /// Lectern at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetCustomSound $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetCustomSound`
        /// 
        /// ##### Description
        /// Gets the key of a custom sound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / CrossProduct $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: CrossProduct`
        /// 
        /// ##### Description
        /// Sets a variable to the cross
        /// product of two vectors.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / X $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: X`
        /// 
        /// ##### Description
        /// Sets a variable to the product
        /// of the given numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetParticleRoll $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleRoll`
        /// 
        /// ##### Description
        /// Gets a particle effect's roll.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ParseYaw $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ParseYaw`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / DotProduct $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: DotProduct`
        /// 
        /// ##### Description
        /// Sets a variable to the dot
        /// product of two vectors.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetZ $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetZ`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SetArmorTrim $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetArmorTrim`
        /// 
        /// ##### Description
        /// Sets the trim of an
        /// armor item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / PopListValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: PopListValue`
        /// 
        /// ##### Description
        /// Gets a list variable's value at
        /// an index and removes it.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// List indices start at 1.
        /// 
        /// ##### Additional Info
        /// If the index is outside the
        /// given list, 0 is returned.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Noise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Noise`
        /// 
        /// ##### Description
        /// Gets a seeded noise value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / MinNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: MinNumber`
        /// 
        /// ##### Description
        /// Sets a variable to the lowest
        /// number in a set.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetPotionType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetPotionType`
        /// 
        /// ##### Description
        /// Gets a potion effect's type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemName`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / ListLength $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ListLength`
        /// 
        /// ##### Description
        /// Gets the number of values
        /// a list has.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Sine $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Sine`
        /// 
        /// ##### Description
        /// Sets a variable to the trigonometric
        /// sine function of a number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Use the variant tag to swap between
        /// sin, asin and sinh functions.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / DirectionName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: DirectionName`
        /// 
        /// ##### Description
        /// Sets a variable to the name
        /// of the direction of a vector.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RepeatString $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RepeatString`
        /// 
        /// ##### Description
        /// Repeats a string the given number
        /// of times.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemLore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemLore`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / JoinString $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: JoinString`
        /// 
        /// ##### Description
        /// Combines a list of strings.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The joining string will be added
        /// between each of the joined strings.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ReverseList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ReverseList`
        /// 
        /// ##### Description
        /// Reverses the order of a
        /// list variable's values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / DedupList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: DedupList`
        /// 
        /// ##### Description
        /// Removes list elements that appear
        /// more than once.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Only the first appearance of
        /// a value is kept.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / CreateDict $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: CreateDict`
        /// 
        /// ##### Description
        /// Creates a dictionary with the
        /// given keys and values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RoundNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RoundNumber`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / FaceLocation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: FaceLocation`
        /// 
        /// ##### Description
        /// Sets a location's rotation to
        /// face another location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemLoreLine $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemLoreLine`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SetVectorLength $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetVectorLength`
        /// 
        /// ##### Description
        /// Sets a vector's length. This
        /// affects all components.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Vectors with an X, Y, and Z
        /// of zero will not be affected.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetPotionDur $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetPotionDur`
        /// 
        /// ##### Description
        /// Sets a potion effect's duration.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / BlockResistance $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: BlockResistance`
        /// 
        /// ##### Description
        /// Gets a block's blast resistance.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SplitString $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SplitString`
        /// 
        /// ##### Description
        /// Splits a string into a list of strings.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Leading and trailing spaces
        /// are removed from the result.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / NormalRandom $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: NormalRandom`
        /// 
        /// ##### Description
        /// Sets a variable to a normally distributed
        /// random number. Values closer to μ are
        /// more likely to be chosen.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Also known as a Gaussian distribution.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetPotionType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetPotionType`
        /// 
        /// ##### Description
        /// Sets a potion effect's type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AlignVector $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AlignVector`
        /// 
        /// ##### Description
        /// Aligns a vector to the
        /// nearest axis.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemDura $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemDura`
        /// 
        /// ##### Description
        /// Sets the given item's durability.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetBreakability $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetBreakability`
        /// 
        /// ##### Description
        /// Sets whether an item is
        /// unbreakable.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetSignText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetSignText`
        /// 
        /// ##### Description
        /// Gets a sign's text at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RaycastEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RaycastEntity`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SetDictValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetDictValue`
        /// 
        /// ##### Description
        /// Sets the given key to the value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Setting a value to an existing
        /// key, will overwrite the old value.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetAllCoords $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetAllCoords`
        /// 
        /// ##### Description
        /// Sets a location's coordinates or
        /// creates a new location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Rgbcolor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Rgbcolor`
        /// 
        /// ##### Description
        /// Creates a color hex based on red,
        /// green, and blue channels.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetCanDestroy $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetCanDestroy`
        /// 
        /// ##### Description
        /// Sets which blocks an item
        /// can break in Adventure Mode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Hslcolor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Hslcolor`
        /// 
        /// ##### Description
        /// Creates a color based on hue,
        /// saturation, and lightness.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetDirection $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetDirection`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetItemLore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemLore`
        /// 
        /// ##### Description
        /// Gets an item's lore list.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RemoveListIndex $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RemoveListIndex`
        /// 
        /// ##### Description
        /// Removes a list variable's value
        /// at an index and shifts all values
        /// after it to the left.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / CellularNoise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: CellularNoise`
        /// 
        /// ##### Description
        /// Gets cellular noise: A type of noise
        /// based on distance from cell origins.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Logarithm $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Logarithm`
        /// 
        /// ##### Description
        /// Finds the logarithm of a number.
        /// A logarithm is the power the
        /// base must be raised to to get
        /// the given input.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If the result of the logarithm
        /// is undefined, 0 will be returned.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemTag`
        /// 
        /// ##### Description
        /// Sets the value of or creates
        /// a custom stored tag value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / TrimString $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: TrimString`
        /// 
        /// ##### Description
        /// Trims a string, starting and ending
        /// at the given positions.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ParseMiniMessageExpr $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ParseMiniMessageExpr`
        /// 
        /// ##### Description
        /// Parses a MiniMessage expression from
        /// a string value into a styled text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemAmount $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemAmount`
        /// 
        /// ##### Description
        /// Gets an item's stack size.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetPotionAmp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetPotionAmp`
        /// 
        /// ##### Description
        /// Sets a potion effect's amplifier.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetCanDestroy $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetCanDestroy`
        /// 
        /// ##### Description
        /// Gets which blocks an item
        /// can break in Adventure Mode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Output is in arbitrary order.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RotateAroundAxis $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RotateAroundAxis`
        /// 
        /// ##### Description
        /// Rotates a vector around an
        /// axis by an angle.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemName`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetItemDura $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemDura`
        /// 
        /// ##### Description
        /// Gets the given item's current or
        /// maximum durability.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftInDirection $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftInDirection`
        /// 
        /// ##### Description
        /// Shifts a location forward, upward,
        /// or sideways based on its rotation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / WrapNum $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: WrapNum`
        /// 
        /// ##### Description
        /// Checks if a number is inside
        /// the bounds and if not, wraps
        /// it around the farthest bound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Examples
        /// - WrapNum(10, 2, 8) = 4
        /// - WrapNum(4, 1, 4) = 1
        /// - WrapNum(-2, 0, 7) = 5
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ReplaceString $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ReplaceString`
        /// 
        /// ##### Description
        /// Searches for part of a string
        /// and replaces it.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemName`
        /// 
        /// ##### Description
        /// Sets an item's name.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetLodestoneLoc $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetLodestoneLoc`
        /// 
        /// ##### Description
        /// Sets a compass's lodestone location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / FlattenList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: FlattenList`
        /// 
        /// ##### Description
        /// Sets a variable to a list with
        /// its sub-lists spread out
        /// into individual elements.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Examples
        /// - \[\[1, 2\], 5, \[3, 4\]\] becomes \[1, 2, 5, 3, 4\]
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / BlockHardness $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: BlockHardness`
        /// 
        /// ##### Description
        /// Gets a block's hardness value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetPotionAmp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetPotionAmp`
        /// 
        /// ##### Description
        /// Gets a potion effect's amplifier.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetParticleAmount $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleAmount`
        /// 
        /// ##### Description
        /// Gets a particle effect's particle
        /// amount.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetDictSize $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetDictSize`
        /// 
        /// ##### Description
        /// Gets the number of entries
        /// in a dictionary.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemAmount $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemAmount`
        /// 
        /// ##### Description
        /// Sets an item's stack size.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SubtractVectors $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SubtractVectors`
        /// 
        /// ##### Description
        /// Sets a variable to the difference
        /// between the given vectors.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetCase $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetCase`
        /// 
        /// ##### Description
        /// Sets a string's capitalization
        /// (eg. to uppercase).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleColor`
        /// 
        /// ##### Description
        /// Sets a particle effect's particle
        /// color and color variation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetLight $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetLight`
        /// 
        /// ##### Description
        /// Gets the light level at
        /// a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetBookText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetBookText`
        /// 
        /// ##### Description
        /// Gets a book's text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetDictValues $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetDictValues`
        /// 
        /// ##### Description
        /// Gets the list of values
        /// in this dictionary.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Vector $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Vector`
        /// 
        /// ##### Description
        /// Sets a variable to a vector.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Distance $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Distance`
        /// 
        /// ##### Description
        /// Sets a variable to the distance
        /// between two locations.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemLore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemLore`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / Root $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Root`
        /// 
        /// ##### Description
        /// Takes the root of a number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleAmount $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleAmount`
        /// 
        /// ##### Description
        /// Sets a particle effect's particle
        /// amount.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AddItemEnchant $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AddItemEnchant`
        /// 
        /// ##### Description
        /// Adds an enchantment to an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemType`
        /// 
        /// ##### Description
        /// Gets an item's material.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetDirection $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetDirection`
        /// 
        /// ##### Description
        /// Gets a location's rotation
        /// (pitch and yaw) as a direction.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetLoreLine $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetLoreLine`
        /// 
        /// ##### Description
        /// Gets a single line from
        /// an item's lore.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetParticleType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleType`
        /// 
        /// ##### Description
        /// Gets a particle effect's type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RemoveString $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RemoveString`
        /// 
        /// ##### Description
        /// Searches for part of a string and
        /// removes all instances of it.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetAllBlockData $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetAllBlockData`
        /// 
        /// ##### Description
        /// Gets the block state tags
        /// at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Returns "0" if the block
        /// has no tags.
        /// 
        /// ##### Additional Info
        /// Always returns a string value.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / MaxNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: MaxNumber`
        /// 
        /// ##### Description
        /// Sets a variable to the highest
        /// number in a set.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetDictKeys $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetDictKeys`
        /// 
        /// ##### Description
        /// Gets the list of keys
        /// in this dictionary.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / TrimStyledText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: TrimStyledText`
        /// 
        /// ##### Description
        /// Trims the content of styled text,
        /// leaving all formatting in place.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleMat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleMat`
        /// 
        /// ##### Description
        /// Sets a particle effect's particle
        /// display material.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetCoord $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetCoord`
        /// 
        /// ##### Description
        /// Gets a location's X, Y, Z, pitch,
        /// or yaw coordinate.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RemoveItemTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RemoveItemTag`
        /// 
        /// ##### Description
        /// Removes a custom item tag.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleSize $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleSize`
        /// 
        /// ##### Description
        /// Sets a particle effect's particle
        /// size and size variation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetPotionDur $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetPotionDur`
        /// 
        /// ##### Description
        /// Gets a potion effect's duration.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RandomLoc $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RandomLoc`
        /// 
        /// ##### Description
        /// Sets the variable to a random
        /// location between two locations.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetSoundType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetSoundType`
        /// 
        /// ##### Description
        /// Sets a sound's type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetLodestoneLoc $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetLodestoneLoc`
        /// 
        /// ##### Description
        /// Gets a compass's lodestone location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftDirection $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftDirection`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetContainerName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetContainerName`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetParticleSprd $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleSprd`
        /// 
        /// ##### Description
        /// Gets a particle effect's horizontal
        /// or vertical spread.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ReflectVector $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ReflectVector`
        /// 
        /// ##### Description
        /// Reflects a vector off a
        /// surface defined by another
        /// vector.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetHeadOwner $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetHeadOwner`
        /// 
        /// ##### Description
        /// Gets a player head's owner
        /// name or UUID.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemEnchants $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemEnchants`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / AppendDict $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AppendDict`
        /// 
        /// ##### Description
        /// Adds all entries from one
        /// dictionary into the other.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetMaxItemAmount $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetMaxItemAmount`
        /// 
        /// ##### Description
        /// Gets an item's maximum stack size.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetColorChannels $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetColorChannels`
        /// 
        /// ##### Description
        /// Gets a color's RGB/HSB/HSL
        /// number values as a list.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetDirection $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetDirection`
        /// 
        /// ##### Description
        /// Sets a location's rotation
        /// (pitch and yaw) to a direction.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetListValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetListValue`
        /// 
        /// ##### Description
        /// Sets a list variable's value at
        /// an index.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// List indices start at 1.
        /// 
        /// ##### Additional Info
        /// Setting an index beyond the
        /// bounds of the list will
        /// clamp the index to the
        /// list bounds.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemEnchants $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemEnchants`
        /// 
        /// ##### Description
        /// Sets an item's enchantment list.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Refer to the format of the
        /// 'Get Item Enchantments' action.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetBookText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetBookText`
        /// 
        /// ##### Description
        /// Sets a book's text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RandomValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RandomValue`
        /// 
        /// ##### Description
        /// Sets a variable to a random
        /// value from a set.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemType`
        /// 
        /// ##### Description
        /// Sets an item's material.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetSoundType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetSoundType`
        /// 
        /// ##### Description
        /// Gets the given sound's type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetListValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetListValue`
        /// 
        /// ##### Description
        /// Gets a list variable's value at
        /// an index.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// List indices start at 1.
        /// 
        /// ##### Additional Info
        /// If the index is outside the
        /// given list, 0 is returned.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / BounceNum $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: BounceNum`
        /// 
        /// ##### Description
        /// Checks if a number is inside
        /// the bounds and if not, bounces
        /// it towards the other bound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Examples
        /// - BounceNum(10, 2, 8) = 6
        /// - BounceNum(4, 1, 4) = 4
        /// - BounceNum(-9, 0, 7) = 5
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Tangent $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Tangent`
        /// 
        /// ##### Description
        /// Sets a variable to the trigonometric
        /// tangent function of a number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Use the variant tag to swap between
        /// tan, atan and tanh functions.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / VoronoiNoise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: VoronoiNoise`
        /// 
        /// ##### Description
        /// Gets a Voronoi noise value: A cellular
        /// noise in which the value of an entire
        /// cell is calculated.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetDirection $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetDirection`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / Hsbcolor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Hsbcolor`
        /// 
        /// ##### Description
        /// Creates a color based on hue,
        /// saturation, and brightness.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: `
        /// 
        /// ##### Description
        /// Increments a number variable
        /// by 1 or more other numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetSoundVariant $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetSoundVariant`
        /// 
        /// ##### Description
        /// Gets the variant of a sound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemColor`
        /// 
        /// ##### Description
        /// Gets a colorable item's color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Leather Armor
        /// - Potions
        /// - Tipped Arrows
        /// - Filled Maps
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ClearFormatting $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ClearFormatting`
        /// 
        /// ##### Description
        /// Clears all formatting from the
        /// given styled text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / InsertListValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: InsertListValue`
        /// 
        /// ##### Description
        /// Inserts a value into a list
        /// variable at an index, shifting
        /// all values at and after it to
        /// the right.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetSoundVolume $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetSoundVolume`
        /// 
        /// ##### Description
        /// Sets a sound's volume.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Sound volume decreases based
        /// on distance to the sound's origin.
        /// 
        /// ##### Additional Info
        /// The maximum distance to hear a
        /// sound is equal to 16.0 × volume.
        /// 
        /// ##### Additional Info
        /// At volumes below 1.0, the sound's
        /// loudness at its origin decreases.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetCoord $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetCoord`
        /// 
        /// ##### Description
        /// Sets a location's X, Y, Z, pitch,
        /// or yaw coordinate.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AddVectors $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AddVectors`
        /// 
        /// ##### Description
        /// Sets a variable to the sum
        /// of the given vectors.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetPitch $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetPitch`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetParticleFade $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleFade`
        /// 
        /// ##### Description
        /// Gets a particle effect's particle
        /// fade color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RaycastBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RaycastBlock`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetItemEnchants $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemEnchants`
        /// 
        /// ##### Description
        /// Gets an item's enchantments.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetHeadTexture $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetHeadTexture`
        /// 
        /// ##### Description
        /// Sets a player head's texture
        /// using an owning player
        /// or custom texture.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / PerlinNoise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: PerlinNoise`
        /// 
        /// ##### Description
        /// Gets a Perlin noise value: A type
        /// of fractal gradient noise.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / WorleyNoise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: WorleyNoise`
        /// 
        /// ##### Description
        /// Gets a Worley noise value: A cellular
        /// noise in which the distance between
        /// two cells' nuclei is calculated.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemColor`
        /// 
        /// ##### Description
        /// Sets a colorable item's color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Leather Armor
        /// - Potions
        /// - Tipped Arrows
        /// - Filled Maps
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetParticleColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleColor`
        /// 
        /// ##### Description
        /// Gets a particle effect's particle
        /// color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetSoundPitch $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetSoundPitch`
        /// 
        /// ##### Description
        /// Sets a sound's pitch or note
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RoundNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RoundNumber`
        /// 
        /// ##### Description
        /// Rounds a number to a whole
        /// number or multiple.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetCanPlaceOn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetCanPlaceOn`
        /// 
        /// ##### Description
        /// Gets which blocks an item
        /// can be placed on in Adventure Mode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Output is in arbitrary order.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SortList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SortList`
        /// 
        /// ##### Description
        /// Sorts a list variable's values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Numbers, Text, Lists
        /// 
        /// ##### Additional Info
        /// Sub-lists are sorted by
        /// their first index.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetCustomSound $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetCustomSound`
        /// 
        /// ##### Description
        /// Sets the key of a custom sound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RemoveDictEntry $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RemoveDictEntry`
        /// 
        /// ##### Description
        /// Removes the dictionary entry
        /// with the given key.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / FormatTime $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: FormatTime`
        /// 
        /// ##### Description
        /// Converts a numerical timestamp to
        /// a human-readable time/date text
        /// using a date format.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Use Game Value: Timestamp to
        /// generate timestamps.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemFlags $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemFlags`
        /// 
        /// ##### Description
        /// Sets which components of an item
        /// are visible, similar to /hideflags.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / StringLength $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: StringLength`
        /// 
        /// ##### Description
        /// Gets the number of characters
        /// a string has.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemEffects $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemEffects`
        /// 
        /// ##### Description
        /// Gets the potion effects applied by
        /// an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Potions
        /// - Tipped Arrows
        /// - Suspicious Stew
        /// 
        /// ##### Additional Info
        /// Durations of effects applied by
        /// Tipped Arrows and Instant Potions
        /// are multiplied by 8.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / StyledText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: StyledText`
        /// 
        /// ##### Description
        /// Sets a variable to a styled text, or combines
        /// multiple values into one styled text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// All values will be converted to styled text.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetMiniMessageExpr $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetMiniMessageExpr`
        /// 
        /// ##### Description
        /// Gets the MiniMessage expression for
        /// a styled text value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// This value will often NOT match the
        /// source expression! Meta tags such as
        /// "&lt;rainbow&ht;" will be broken down into
        /// several adjacent "&lt;color&ht;" tags.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetYaw $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetYaw`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SetItemLore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemLore`
        /// 
        /// ##### Description
        /// Sets an item's lore list.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetItemEffects $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetItemEffects`
        /// 
        /// ##### Description
        /// Sets the potion effects applied by
        /// an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Potions
        /// - Tipped Arrows
        /// - Suspicious Stew
        /// 
        /// ##### Additional Info
        /// Durations of effects applied by
        /// Tipped Arrows and Instant Potions
        /// are divided by 8.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: `
        /// 
        /// ##### Description
        /// Decrements a number variable
        /// by 1 or more other numbers.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetItemTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetItemTag`
        /// 
        /// ##### Description
        /// Gets the value of a custom
        /// item tag.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Returns 0 if the tag is
        /// not present.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / CreateList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: CreateList`
        /// 
        /// ##### Description
        /// Creates a list from the given
        /// values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AppendList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AppendList`
        /// 
        /// ##### Description
        /// Adds a list to the end of
        /// another list variable.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetContainerItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetContainerItems`
        /// 
        /// ##### Description
        /// Gets a container's contents
        /// at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftToward $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftToward`
        /// 
        /// ##### Description
        /// Shifts a location toward another
        /// location by the given distance.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / TrimList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: TrimList`
        /// 
        /// ##### Description
        /// Trims a list, starting and ending
        /// at the given indices.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GradientNoise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GradientNoise`
        /// 
        /// ##### Description
        /// Gets gradient noise: A type of noise
        /// based on a lattice of random gradients.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetBlockDrops $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetBlockDrops`
        /// 
        /// ##### Description
        /// Gets the items dropped by a
        /// block if mined by a given tool.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Setting the tool to air will
        /// use no tool (player's fist).
        /// 
        /// ##### Additional Info
        /// Leaving the tool unset will
        /// use the best tool to mine the
        /// block.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ClearEnchants $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ClearEnchants`
        /// 
        /// ##### Description
        /// Removes enchantments from an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Cosine $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Cosine`
        /// 
        /// ##### Description
        /// Sets a variable to the trigonometric
        /// cosine function of a number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Use the variant tag to swap between
        /// cos, acos and cosh functions.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetParticleFade $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleFade`
        /// 
        /// ##### Description
        /// Sets a particle effect's particle
        /// fade color.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetVectorComp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetVectorComp`
        /// 
        /// ##### Description
        /// Sets a vector's X, Y, or Z
        /// component.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ParseNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ParseNumber`
        /// 
        /// ##### Description
        /// Converts a string to a number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If the string is not a valid
        /// number, the result is 0.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Exponent $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Exponent`
        /// 
        /// ##### Description
        /// Raises a number to the power
        /// of an exponent.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftAllDirs $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftAllDirs`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetValueIndex $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetValueIndex`
        /// 
        /// ##### Description
        /// Searches for a value in a list
        /// variable and gets the index if
        /// found.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Returns 0 if the value is not
        /// found in the list.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RemItemEnchant $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RemItemEnchant`
        /// 
        /// ##### Description
        /// Removes an enchantment from an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / AddItemLore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: AddItemLore`
        /// 
        /// ##### Description
        /// Adds lines to an item's lore.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetBookText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetBookText`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / SetParticleRoll $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetParticleRoll`
        /// 
        /// ##### Description
        /// Sets a particle effect's roll.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetSoundVariant $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetSoundVariant`
        /// 
        /// ##### Description
        /// Sets the variant of a sound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftLocation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftLocation`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / RandomizeList $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RandomizeList`
        /// 
        /// ##### Description
        /// Randomizes the order of a
        /// list variable's values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ClampNumber $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ClampNumber`
        /// 
        /// ##### Description
        /// Checks if a number is between
        /// a minimum and maximum value, and
        /// if not, sets it to the nearest.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / Round $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: Round`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetSoundPitch $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetSoundPitch`
        /// 
        /// ##### Description
        /// Gets a sound's pitch or
        /// note.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / TranslateColors $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: TranslateColors`
        /// 
        /// ##### Description
        /// Converts legacy color codes written in
        /// "&" or hex format to
        /// functional color codes, or vice versa.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetBlockGrowth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetBlockGrowth`
        /// 
        /// ##### Description
        /// Gets a block's current
        /// growth at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetAllItemTags $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetAllItemTags`
        /// 
        /// ##### Description
        /// Gets all tags registered
        /// on an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / RemoveListValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: RemoveListValue`
        /// 
        /// ##### Description
        /// Removes all matching values
        /// from a list variable.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ShiftAllDirections $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ShiftAllDirections`
        /// 
        /// ##### Description
        /// Shifts a location in multiple directions
        /// based on its rotation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ValueNoise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ValueNoise`
        /// 
        /// ##### Description
        /// Gets value noise: A type of noise
        /// based on a lattice of random values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetCanPlaceOn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetCanPlaceOn`
        /// 
        /// ##### Description
        /// Sets which blocks an item
        /// can be placed on in Adventure Mode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetBlockType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetBlockType`
        /// 
        /// ##### Description
        /// Gets a block's material
        /// at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ParsePitch $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ParsePitch`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetDictValue $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetDictValue`
        /// 
        /// ##### Description
        /// Get a dictionary variable's
        /// value for a key.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If the dictionary does not
        /// contain the key, 0 is returned.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ContainerLock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ContainerLock`
        /// 
        /// ##### Description
        /// Gets a container's lock key at a
        /// location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Returns "none" if the container
        /// is not locked.
        /// 
        /// ##### Additional Info
        /// Returns 0 if the container
        /// is not lockable.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetBlockPower $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetBlockPower`
        /// 
        /// ##### Description
        /// Gets a block's redstone
        /// power level.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / GetVectorLength $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetVectorLength`
        /// 
        /// ##### Description
        /// Gets a vector's length.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / ContentLength $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: ContentLength`
        /// 
        /// ##### Description
        /// Gets the number of characters
        /// a styled text has, ignoring all
        /// formatting tags.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetModelData $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetModelData`
        /// 
        /// ##### Description
        /// Sets the model value
        /// used in resource packs.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SetVariable / SetCoords $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: SetCoords`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SetVariable / GetParticleSize $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SetVariable: GetParticleSize`
        /// 
        /// ##### Description
        /// Gets a particle effect's particle
        /// size.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsLookingAt $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsLookingAt`
        /// 
        /// ##### Description
        /// Checks if a player is looking at
        /// the given block or location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / InWorldBorder $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: InWorldBorder`
        /// 
        /// ##### Description
        /// Checks if a player (or a location)
        /// is within their world border.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { IfPlayer / IsInGameMode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsInGameMode`
        /// 
        /// ##### Description
        /// Checks if a player is in
        /// a specific game mode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / HasRoomForItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: HasRoomForItem`
        /// 
        /// ##### Description
        /// Checks if a player's inventory
        /// has room for one or more
        /// items to be given.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsHoldingOff $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsHoldingOff`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / UsingPack $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: UsingPack`
        /// 
        /// ##### Description
        /// Checks if a player is
        /// using a plot resource
        /// pack.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { IfPlayer / NoItemCooldown $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: NoItemCooldown`
        /// 
        /// ##### Description
        /// Checks if a player does not have a
        /// cooldown applied to an item type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The check will succeed if any
        /// of the given items are not on
        /// cooldown.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsUsingItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsUsingItem`
        /// 
        /// ##### Description
        /// Checks if a player is currently
        /// using an item (eg. bow).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / HasAllItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: HasAllItems`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / IsSwimming $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsSwimming`
        /// 
        /// ##### Description
        /// Checks if a player
        /// is in water or lava.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Use 'Is Sprinting'
        /// to check if a player
        /// is swimming with the
        /// swimming animation.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / HasItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: HasItem`
        /// 
        /// ##### Description
        /// Checks if a player has an item
        /// in their inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / BlockEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: BlockEquals`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / IsWearing $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsWearing`
        /// 
        /// ##### Description
        /// Checks if a player is wearing
        /// an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsNear $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsNear`
        /// 
        /// ##### Description
        /// Checks if a player is within a
        /// range of a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsRiding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsRiding`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / StandingOn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: StandingOn`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / CmdEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: CmdEquals`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / StandingOn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: StandingOn`
        /// 
        /// ##### Description
        /// Checks if a player is standing on
        /// the given block or location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsGrounded $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsGrounded`
        /// 
        /// ##### Description
        /// Checks if a player is
        /// supported by a block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / CursorItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: CursorItem`
        /// 
        /// ##### Description
        /// Checks if the item that is being moved
        /// with a player's cursor is the given item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If multiple items are in the chest,
        /// the target can have any of them on
        /// their cursor.
        /// 
        /// ##### Additional Info
        /// When used on the Player Click Item
        /// in Own Inventory Event, 'Cursor Item
        /// Equals' checks the previous cursor
        /// item, not the item that was clicked.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / SlotEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: SlotEquals`
        /// 
        /// ##### Description
        /// Checks if a player's currently
        /// selected hotbar slot equals the
        /// given slot ID.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / ItemEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: ItemEquals`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / IsHoldingMain $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsHoldingMain`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfPlayer / IsHolding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsHolding`
        /// 
        /// ##### Description
        /// Checks if a player is holding
        /// an item in their hand.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / MenuSlotEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: MenuSlotEquals`
        /// 
        /// ##### Description
        /// Checks if a player's currently
        /// open inventory menu contains
        /// an item in the given slot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsBlocking $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsBlocking`
        /// 
        /// ##### Description
        /// Checks if a player is
        /// blocking with a shield.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / HasPermission $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: HasPermission`
        /// 
        /// ##### Description
        /// Checks if a player has a certain
        /// level of access on this plot, such
        /// as builder or owner.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsRiding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsRiding`
        /// 
        /// ##### Description
        /// Checks if a player is riding
        /// another entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / MainHandEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: MainHandEquals`
        /// 
        /// ##### Description
        /// Checks if a player's main hand
        /// is their left or right hand.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsSneaking $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsSneaking`
        /// 
        /// ##### Description
        /// Checks if a player is sneaking.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsFlying $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsFlying`
        /// 
        /// ##### Description
        /// Checks if a player is flying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / HasPotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: HasPotion`
        /// 
        /// ##### Description
        /// Checks if a player has a
        /// potion effect of the given
        /// type active.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / NameEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: NameEquals`
        /// 
        /// ##### Description
        /// Checks if a player's username is
        /// equal to one of the given
        /// usernames (case insensitive).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Works with UUIDs.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / InvOpen $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: InvOpen`
        /// 
        /// ##### Description
        /// Checks if a player has a
        /// certain inventory type open.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Does not work with special
        /// screens such as the death
        /// screen or the player's own
        /// inventory.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / HasSlotItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: HasSlotItem`
        /// 
        /// ##### Description
        /// Checks if a player has an item
        /// in the given inventory slot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsSprinting $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsSprinting`
        /// 
        /// ##### Description
        /// Checks if a player is sprinting
        /// or using the sprint key to swim.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / IsGliding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: IsGliding`
        /// 
        /// ##### Description
        /// Checks if a player is
        /// gliding with elytra.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfPlayer / CmdArgEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfPlayer: CmdArgEquals`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { Process / Dynamic $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Process: Dynamic`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { EntityEvent / EntityKillEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: EntityKillEntity`
        /// 
        /// ##### Description
        /// Executes code when an entity
        /// kills another entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / BlockFall $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: BlockFall`
        /// 
        /// ##### Description
        /// Executes code when a block
        /// affected by gravity turns
        /// into a falling block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / ProjKillEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: ProjKillEntity`
        /// 
        /// ##### Description
        /// Executes code when a
        /// projectile kills an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / EntityDmgEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: EntityDmgEntity`
        /// 
        /// ##### Description
        /// Executes code when an entity
        /// damages another entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / FallingBlockLand $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: FallingBlockLand`
        /// 
        /// ##### Description
        /// Executes code when a falling
        /// block lands on the ground.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / EntityResurrect $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: EntityResurrect`
        /// 
        /// ##### Description
        /// Executes code when
        /// an entity resurrects with
        /// a totem of undying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / ItemMerge $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: ItemMerge`
        /// 
        /// ##### Description
        /// Executes code when dropped items
        /// try to merge into a stack.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / EntityHeal $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: EntityHeal`
        /// 
        /// ##### Description
        /// Executes code when an
        /// entity regains health.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / Teleport $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: Teleport`
        /// 
        /// ##### Description
        /// Executes code when
        /// an entity is teleported.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / ShootBow $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: ShootBow`
        /// 
        /// ##### Description
        /// Executes code when an
        /// entity shoots a bow.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / EntityDmg $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: EntityDmg`
        /// 
        /// ##### Description
        /// Executes code when
        /// an entity takes damage.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / ProjDmgEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: ProjDmgEntity`
        /// 
        /// ##### Description
        /// Executes code when a projectile
        /// damages an entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / EntityExplode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: EntityExplode`
        /// 
        /// ##### Description
        /// Executes code when an
        /// entity explodes.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / EntityDeath $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: EntityDeath`
        /// 
        /// ##### Description
        /// Executes code when an entity
        /// dies by natural causes.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / VehicleDamage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: VehicleDamage`
        /// 
        /// ##### Description
        /// Executes code when a vehicle
        /// entity (minecart or boat) is
        /// damaged.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / Transform $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: Transform`
        /// 
        /// ##### Description
        /// Executes code when an
        /// entity turns into another or
        /// group of others for any reason.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { EntityEvent / RegrowWool $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `EntityEvent: RegrowWool`
        /// 
        /// ##### Description
        /// Executes code when a sheep
        /// regrows its wool.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / SignHasTxt $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: SignHasTxt`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfGame / HasRoomForItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: HasRoomForItem`
        /// 
        /// ##### Description
        /// Checks if the container at a
        /// location has room for one or
        /// more items to be given.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / EventBlockEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: EventBlockEquals`
        /// 
        /// ##### Description
        /// Checks if the block in a block
        /// related event is the given block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Block Events
        /// - Click Events
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / CommandEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: CommandEquals`
        /// 
        /// ##### Description
        /// Checks if the command entered
        /// in the Command Event is equal
        /// to the given string.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Command Event
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { IfGame / EventItemEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: EventItemEquals`
        /// 
        /// ##### Description
        /// Checks if the item in a item
        /// related event is the given item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Item Events
        /// - Right Click Event
        /// - Left Click Event
        /// - Change Slot Event
        /// - Place Block Event
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / SignHasTxt $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: SignHasTxt`
        /// 
        /// ##### Description
        /// Checks if the sign at a location
        /// has the given text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / AttackIsCrit $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: AttackIsCrit`
        /// 
        /// ##### Description
        /// Checks if an event attack
        /// is critical.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / ContainerHas $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: ContainerHas`
        /// 
        /// ##### Description
        /// Checks if the container at a
        /// location has the given item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If multiple items are in the
        /// chest, the container only
        /// needs to have one of them.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / BlockEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: BlockEquals`
        /// 
        /// ##### Description
        /// Checks if the block at a location
        /// is the given block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / InBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: InBlock`
        /// 
        /// ##### Description
        /// Checks if a location collides with
        /// the hitbox of the nearest block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / BlockPowered $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: BlockPowered`
        /// 
        /// ##### Description
        /// Checks if the block at a location
        /// is powered by redstone.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / HasPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: HasPlayer`
        /// 
        /// ##### Description
        /// Checks if there is currently
        /// a player in the game with the
        /// given name or UUID.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / ContainerHasAll $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: ContainerHasAll`
        /// 
        /// ##### Description
        /// Checks if the container at a
        /// location has all of the given
        /// items.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / CmdArgEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: CmdArgEquals`
        /// 
        /// ##### Description
        /// Checks if a part of the command
        /// entered in the Command Event
        /// is equal to the given string.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Command Event
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { IfGame / EventCancelled $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: EventCancelled`
        /// 
        /// ##### Description
        /// Checks if the current
        /// event is cancelled.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfGame / IsChunkLoaded $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfGame: IsChunkLoaded`
        /// 
        /// ##### Description
        /// Checks if the chunk at a location
        /// is currently loaded.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / FillContainer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: FillContainer`
        /// 
        /// ##### Description
        /// Fills the container at a location
        /// with items.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / BreakBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: BreakBlock`
        /// 
        /// ##### Description
        /// Breaks the block at a location
        /// as if it was broken by a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / LPfxSpiral $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: LPfxSpiral`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / ParticleSphere $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleSphere`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / ChangeSign $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ChangeSign`
        /// 
        /// ##### Description
        /// Changes a line of text
        /// on a sign.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / WebRequest $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: WebRequest`
        /// 
        /// ##### Description
        /// Sends a web request to a URL.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { GameAction / ClearScBoard $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ClearScBoard`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / HideSidebar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: HideSidebar`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SpawnItemDisplay $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnItemDisplay`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / WriteTransaction $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: WriteTransaction`
        /// 
        /// ##### Description
        /// Adds blocks to the next transaction; a method
        /// of queuing up block operations so that
        /// they can be sent simultaneously.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Requires the Game Action: Apply Transaction
        /// in order to apply changes to the world.
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { GameAction / ParticleSpiral $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleSpiral`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetBlockData $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetBlockData`
        /// 
        /// ##### Description
        /// Sets a data tag value of
        /// the block at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / Firework $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: Firework`
        /// 
        /// ##### Description
        /// Launches a firework
        /// rocket at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetEventDamage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetEventDamage`
        /// 
        /// ##### Description
        /// Sets the damage dealt in
        /// this event.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Damage Events
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnItem`
        /// 
        /// ##### Description
        /// Spawns an item at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SignColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SignColor`
        /// 
        /// ##### Description
        /// Changes the text color
        /// of a sign.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ShulkerBullet $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ShulkerBullet`
        /// 
        /// ##### Description
        /// Spawns a shulker bullet at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / FireworkEffect $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: FireworkEffect`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetContainer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetContainer`
        /// 
        /// ##### Description
        /// Sets the contents of the container
        /// at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnInteraction $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnInteraction`
        /// 
        /// ##### Description
        /// Spawns an invisible hitbox
        /// with the specified size.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Detect interactions with:
        /// ⏵ Player Damage Entity Event
        /// ⏵ Player Right Click Entity Event
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetItemInSlot $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetItemInSlot`
        /// 
        /// ##### Description
        /// Sets the item in a slot of the
        /// container at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / CloneRegion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: CloneRegion`
        /// 
        /// ##### Description
        /// Copies a region of blocks to another
        /// region, including air.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { GameAction / UncancelEvent $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: UncancelEvent`
        /// 
        /// ##### Description
        /// Uncancels the initial event that
        /// triggered this line of code.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Events cannot be uncancelled
        /// after a Control: Wait block.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetLecternBook $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetLecternBook`
        /// 
        /// ##### Description
        /// Sets the book and the
        /// displayed page of a Lectern.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnArmorStand $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnArmorStand`
        /// 
        /// ##### Description
        /// Spawns an armor stand at a
        /// location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Options to set the pose and tags
        /// are in Entity Action » Appearance.
        /// 
        /// ##### Additional Info
        /// Equipment goes from the bottom
        /// left corner towards the right:
        /// 
        /// Helmet, Chestplate, Leggings,
        /// Boots, Right Hand, Left Hand
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { GameAction / SpawnBlockDisp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnBlockDisp`
        /// 
        /// ##### Description
        /// Spawns a block display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Notable defaults:
        /// ⏵ Billboard = Fixed
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { GameAction / ClearContainer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ClearContainer`
        /// 
        /// ##### Description
        /// Empties a container at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / CancelEvent $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: CancelEvent`
        /// 
        /// ##### Description
        /// Cancels the initial event that
        /// triggered this line of code.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Events cannot be cancelled
        /// after a Control: Wait block.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ParticleEffect $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleEffect`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SpawnFangs $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnFangs`
        /// 
        /// ##### Description
        /// Spawns evoker fangs at a
        /// location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Evoker Fangs deal damage
        /// and remove themselves a
        /// second later.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetEventSound $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetEventSound`
        /// 
        /// ##### Description
        /// Sets the sound to play for
        /// this event, replacing the
        /// original sound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Entity Death Events
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetEventXP $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetEventXP`
        /// 
        /// ##### Description
        /// Sets the amount of experience
        /// this event should drop.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Entity Death Events
        /// - Fish Event
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / LockContainer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: LockContainer`
        /// 
        /// ##### Description
        /// Sets the lock key of the container
        /// at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Lock keys determine the name of
        /// the item used to open the container.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / RemoveScore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: RemoveScore`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / CreateHologram $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: CreateHologram`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetExhaustion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetExhaustion`
        /// 
        /// ##### Description
        /// Sets the exhaustion 
        /// gained in this event.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Exhaustion Events
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ParticleCircle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleCircle`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / PfxLineA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: PfxLineA`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / ClearItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ClearItems`
        /// 
        /// ##### Description
        /// Removes all of an item from
        /// the container at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / StartLoop $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: StartLoop`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetFurnaceSpeed $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetFurnaceSpeed`
        /// 
        /// ##### Description
        /// Sets the amount of ticks it
        /// takes for a furnace block
        /// to cook an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Furnace
        /// - Blast Furnace
        /// - Smoker
        /// 
        /// ##### Additional Info
        /// By default, a furnace cooks
        /// in 200 ticks.
        /// 
        /// ##### Additional Info
        /// Fuel duration is unaffected
        /// by cooking time.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / BlockDropsOn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: BlockDropsOn`
        /// 
        /// ##### Description
        /// Enables blocks dropping
        /// as items when broken.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / BoneMeal $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: BoneMeal`
        /// 
        /// ##### Description
        /// Applies bone meal to a block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { GameAction / DebugStackTrace $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: DebugStackTrace`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Dev</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(feature = "rank_dev")]
        #[doc(cfg(feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / FallingBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: FallingBlock`
        /// 
        /// ##### Description
        /// Spawns a falling block at a
        /// location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Falling blocks automatically
        /// disappear after 30 seconds.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / DiscordWebhook $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: DiscordWebhook`
        /// 
        /// ##### Description
        /// Sends a message to a Discord
        /// webhook.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { GameAction / TickBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: TickBlock`
        /// 
        /// ##### Description
        /// Causes a block to get "random
        /// ticked", which could cause a
        /// block update.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ReplaceItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ReplaceItems`
        /// 
        /// ##### Description
        /// Replaces items in the container
        /// at a location with the given item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetEventProj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetEventProj`
        /// 
        /// ##### Description
        /// Replaces the projectile fired in
        /// the Shoot Bow Event.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Shoot Bow Event
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / Explosion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: Explosion`
        /// 
        /// ##### Description
        /// Creates an explosion at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnMob $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnMob`
        /// 
        /// ##### Description
        /// Spawns a mob at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Equipment goes from the bottom
        /// left corner towards the right:
        /// 
        /// Main Hand, Helmet, Chestplate,
        /// Leggings, Boots, Off Hand
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetBrushableItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetBrushableItem`
        /// 
        /// ##### Description
        /// Sets the item buried in a
        /// suspicious sand or gravel.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ParticleLineA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleLineA`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SpawnEnderEye $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnEnderEye`
        /// 
        /// ##### Description
        /// Spawns an eye of ender at a
        /// location, which (if specified) will
        /// float towards its destination.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If the destination is further
        /// than 12 blocks away, the eye's
        /// path always goes upwards.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ShowSidebar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ShowSidebar`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SpawnPotionCloud $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnPotionCloud`
        /// 
        /// ##### Description
        /// Spawns a lingering potion cloud
        /// at a location that imbues effects
        /// onto entities who enter it.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { GameAction / LaunchProj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: LaunchProj`
        /// 
        /// ##### Description
        /// Launches a projectile.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetBlock`
        /// 
        /// ##### Description
        /// Sets the block at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Will cause block updates. Use SetRegion
        /// to not update nearby blocks and
        /// save CPU.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnItemDisp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnItemDisp`
        /// 
        /// ##### Description
        /// Spawns an item display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Notable defaults:
        /// ⏵ Billboard = Fixed
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { GameAction / SetBlockGrowth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetBlockGrowth`
        /// 
        /// ##### Description
        /// Sets the growth stage of the block
        /// (eg. carrots) at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Most crops have growth stages
        /// from 0 to 7.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / Wait $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: Wait`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetContainerName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetContainerName`
        /// 
        /// ##### Description
        /// Sets the name of the container
        /// at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetHead $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetHead`
        /// 
        /// ##### Description
        /// Sets the block at a location
        /// to a player head.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / RemoveHologram $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: RemoveHologram`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / RemoveItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: RemoveItems`
        /// 
        /// ##### Description
        /// Removes items from the container
        /// at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnRngItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnRngItem`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetRegion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetRegion`
        /// 
        /// ##### Description
        /// Fills a region with a type of block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Can set up to 100,000 blocks per
        /// action.
        /// 
        /// ##### Additional Info
        /// Will not cause block updates. A 1x1x1
        /// SetRegion may save CPU compared to
        /// SetBlock in certain situations.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ParticleCircleA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleCircleA`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SpawnTNT $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnTNT`
        /// 
        /// ##### Description
        /// Spawns primed TNT at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnExpOrb $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnExpOrb`
        /// 
        /// ##### Description
        /// Spawns an experience orb at
        /// a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetBiome $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetBiome`
        /// 
        /// ##### Description
        /// Sets the biome of a region.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Can set up to 100,000 blocks per
        /// action.
        /// 
        /// ##### Additional Info
        /// Players may have to reload the chunks
        /// on their client to see a change.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { GameAction / SetEventHeal $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetEventHeal`
        /// 
        /// ##### Description
        /// Sets the amount of health
        /// regained in this event.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Player Heal Event
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / PfxPath $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: PfxPath`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / ApplyTransaction $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ApplyTransaction`
        /// 
        /// ##### Description
        /// Applies the current transaction
        /// and generates a new one.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Can set up to 1,000,000 blocks
        /// per transaction.
        /// 
        /// ##### Additional Info
        /// Only 1 transaction can be active at a time.
        /// 
        /// ##### Additional Info
        /// Transactions are asynchronous, and
        /// may override blocks set after them.
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { GameAction / ParticleRay $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleRay`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / GenerateTree $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: GenerateTree`
        /// 
        /// ##### Description
        /// Generates a tree at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The location is the northwest
        /// corner of 2x2 trees.
        /// 
        /// ##### Additional Info
        /// Trees will not grow on blocks
        /// that do not normally support
        /// tree growth.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / StopLoop $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: StopLoop`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetScObj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetScObj`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SpawnCrystal $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnCrystal`
        /// 
        /// ##### Description
        /// Spawns an end crystal at a
        /// location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SetCampfireItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetCampfireItem`
        /// 
        /// ##### Description
        /// Sets the item being cooked in
        /// one of a campfire's slots.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// After the cooking time, the
        /// item drops from the campfire
        /// (unless the campfire is unlit).
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / SpawnTextDisplay $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnTextDisplay`
        /// 
        /// ##### Description
        /// Spawns a text display entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Notable defaults:
        /// ⏵ Line Width = 200
        /// ⏵ Text Alignment = Center
        /// ⏵ Billboard = Center
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { GameAction / SpawnVehicle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SpawnVehicle`
        /// 
        /// ##### Description
        /// Spawns a vehicle at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / Lightning $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: Lightning`
        /// 
        /// ##### Description
        /// Strikes lightning at a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The lightning's damage can be
        /// detected using the "lightning"
        /// damage event cause.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ParticleSpiralA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleSpiralA`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / SetScore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: SetScore`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / ParticleCluster $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleCluster`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { GameAction / BlockDropsOff $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: BlockDropsOff`
        /// 
        /// ##### Description
        /// Disables blocks dropping
        /// as items when broken.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { GameAction / ParticleLine $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `GameAction: ParticleLine`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetHotbar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetHotbar`
        /// 
        /// ##### Description
        /// Sets items in a player's
        /// hotbar.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetReducedDebug $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetReducedDebug`
        /// 
        /// ##### Description
        /// When enabled, a player won't be
        /// able to see their coordinates,
        /// block info, or other info.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / CloseInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: CloseInv`
        /// 
        /// ##### Description
        /// Closes a player's inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / GiveItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GiveItems`
        /// 
        /// ##### Description
        /// Gives a player all of the
        /// items in the chest.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / NoKeepInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: NoKeepInv`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetHandCrafting $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetHandCrafting`
        /// 
        /// ##### Description
        /// Sets if a player is
        /// allowed to interact with
        /// their hand-crafting menu.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / BossBar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: BossBar`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ParticleSphere $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleSphere`
        /// 
        /// ##### Description
        /// Displays a sphere of particles
        /// at a location to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetVelocity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetVelocity`
        /// 
        /// ##### Description
        /// Sets a player's movement
        /// velocity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / Particle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Particle`
        /// 
        /// ##### Description
        /// Displays a particle effect
        /// to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / AddInvRow $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: AddInvRow`
        /// 
        /// ##### Description
        /// Adds a row to the bottom of
        /// a player's current inventory
        /// menu.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / NoNatRegen $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: NoNatRegen`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / DisplayLightning $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayLightning`
        /// 
        /// ##### Description
        /// Displays a lightning strike
        /// effect to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / Damage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Damage`
        /// 
        /// ##### Description
        /// Damages a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SendAnimation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SendAnimation`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetXPProg $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetXPProg`
        /// 
        /// ##### Description
        /// Sets the XP progress bar
        /// to a certain percentage.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetInventory $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetInventory`
        /// 
        /// ##### Description
        /// Sets items in a player's
        /// upper inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / TpSequence $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: TpSequence`
        /// 
        /// ##### Description
        /// Teleports a player to multiple
        /// locations, with a delay between
        /// each teleport.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / Heal $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Heal`
        /// 
        /// ##### Description
        /// Restores a player's health.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Triggers the Heal Event.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetSpawnPoint $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetSpawnPoint`
        /// 
        /// ##### Description
        /// Sets the location a player will
        /// spawn when they die and respawn.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetInventoryKept $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetInventoryKept`
        /// 
        /// ##### Description
        /// Sets whether a player's inventory
        /// is kept after death.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / LaunchUp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: LaunchUp`
        /// 
        /// ##### Description
        /// Launches a player up or down.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A negative launch power will
        /// launch the player down.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / GetTargetEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GetTargetEntity`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ForceFlight $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ForceFlight`
        /// 
        /// ##### Description
        /// Forces a player to start
        /// or stop flying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / LoadInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: LoadInv`
        /// 
        /// ##### Description
        /// Loads a player's inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If no saved inventory exists,
        /// the player's inventory will be
        /// cleared.
        /// 
        /// ##### Additional Info
        /// Inventories loaded with 'Load
        /// Saved Inventory' take several
        /// ticks to load. Use 'Control:
        /// Wait' if you need to check the
        /// inventory's contents after it is
        /// loaded.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / ChatStyle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ChatStyle`
        /// 
        /// ##### Description
        /// Sets a player's chat color or
        /// decoration.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Examples
        /// - "&lt;#D4D4FF&ht;"
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / Kick $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Kick`
        /// 
        /// ##### Description
        /// Kicks a player from
        /// the plot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / ProjColl $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ProjColl`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / MiscAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: MiscAttribute`
        /// 
        /// ##### Description
        /// Sets one of the player's miscellaneous
        /// attributes such as luck.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SpectateTarget $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SpectateTarget`
        /// 
        /// ##### Description
        /// Makes a player spectate
        /// another player or entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The player must be in
        /// spectator mode.
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / HurtAnimation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: HurtAnimation`
        /// 
        /// ##### Description
        /// Makes a player perform
        /// a hurt animation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SurvivalMode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SurvivalMode`
        /// 
        /// ##### Description
        /// Sets a player's game
        /// mode to Survival.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisplayBellRing $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayBellRing`
        /// 
        /// ##### Description
        /// Displays a bell ring animation
        /// at a location to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetStatus $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetStatus`
        /// 
        /// ##### Description
        /// Sets the player's game status,
        /// which is used to display information
        /// about what the player is doing
        /// in the game.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A player's game status be seen
        /// when using /locate.
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / SetCursorItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetCursorItem`
        /// 
        /// ##### Description
        /// Sets the item on a
        /// player's cursor.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetAbsorption $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetAbsorption`
        /// 
        /// ##### Description
        /// Sets a player's absorption
        /// health (golden hearts).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The target does not need to
        /// have an absorption effect.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetFireTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetFireTicks`
        /// 
        /// ##### Description
        /// Sets the remaining time a
        /// player is on fire for.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// 0 ticks mean the target is
        /// not on fire.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / CombatAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: CombatAttribute`
        /// 
        /// ##### Description
        /// Sets one of the player's combat-related
        /// attributes such as attack damage
        /// and attack speed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetGamemode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetGamemode`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / RemoveInvRow $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RemoveInvRow`
        /// 
        /// ##### Description
        /// Removes the given number of
        /// rows from the bottom of a player's
        /// current inventory menu.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / WakeUpAnimation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: WakeUpAnimation`
        /// 
        /// ##### Description
        /// Displays the wake up (fade in)
        /// animation to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / DisableBlocks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisableBlocks`
        /// 
        /// ##### Description
        /// Prevents a player from placing
        /// and breaking certain blocks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetScoreObj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetScoreObj`
        /// 
        /// ##### Description
        /// Sets the objective name of the
        /// scoreboard sidebar.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / LSetHealth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: LSetHealth`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ParticleEffect $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleEffect`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ClearInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ClearInv`
        /// 
        /// ##### Description
        /// Empties a player's inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetFreezeTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetFreezeTicks`
        /// 
        /// ##### Description
        /// Sets how long a player
        /// is frozen for.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetGliding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetGliding`
        /// 
        /// ##### Description
        /// Sets whether a player
        /// is gliding with elytra.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The player must be
        /// wearing an elytra.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetRotation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetRotation`
        /// 
        /// ##### Description
        /// Changes a player's pitch and
        /// yaw.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ClearItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ClearItems`
        /// 
        /// ##### Description
        /// Removes all of an item from
        /// a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetFlying $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetFlying`
        /// 
        /// ##### Description
        /// Sets whether a player
        /// is flying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisplayBlockOpen $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayBlockOpen`
        /// 
        /// ##### Description
        /// Displays a container block
        /// at a location as being open
        /// or closed to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Chest
        /// - Trapped Chest
        /// - Ender Chest
        /// - Shulker Box
        /// - Barrel
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetHandItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetHandItem`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SendAdvancement $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SendAdvancement`
        /// 
        /// ##### Description
        /// Displays a custom advancement
        /// popup to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / ClearChat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ClearChat`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetMenuItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetMenuItem`
        /// 
        /// ##### Description
        /// Sets the item in a slot
        /// of a player's current
        /// inventory menu.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / LaunchToward $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: LaunchToward`
        /// 
        /// ##### Description
        /// Launches a player toward or away
        /// from a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A negative launch power will launch
        /// the player away from the location.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetArmor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetArmor`
        /// 
        /// ##### Description
        /// Sets a player's armor items.
        /// Place the armor in slots 1-4
        /// of the chest, with 1 being the
        /// helmet and 4 being the boots.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Any block or item can render
        /// on the target's head if placed
        /// in the 'helmet' slot.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisplayGateway $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayGateway`
        /// 
        /// ##### Description
        /// Displays a vertical beam on
        /// an end gateway to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / GiveSaturation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GiveSaturation`
        /// 
        /// ##### Description
        /// Adds saturation to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisplayEquipment $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayEquipment`
        /// 
        /// ##### Description
        /// Displays equipment on an entity
        /// to a player. Equipment goes from
        /// slots 2-7 in order of Helmet,
        /// Chestplate, Leggings, Boots,
        /// Main Hand, Off Hand.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Any block or item can render
        /// on the entity's head if placed
        /// in the 'helmet' slot.
        /// 
        /// ##### Additional Info
        /// This equipment is client-side.
        /// The actual equipment is not changed.
        /// 
        /// ##### Additional Info
        /// If the equipment slot is updated,
        /// it will be reset.
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / GiveExp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GiveExp`
        /// 
        /// ##### Description
        /// Adds experience points or
        /// levels to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / FaceLocation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: FaceLocation`
        /// 
        /// ##### Description
        /// Rotates a player to look
        /// toward a location without
        /// teleporting them.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ClearScoreboard $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ClearScoreboard`
        /// 
        /// ##### Description
        /// Removes all scores from
        /// the scoreboard.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ActionBar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ActionBar`
        /// 
        /// ##### Description
        /// Displays text directly above
        /// a player's hotbar.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Multiple variables (of any type)
        /// will be merged into one message.
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / SetChatTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetChatTag`
        /// 
        /// ##### Description
        /// Sets a player's chat tag.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Multiple text items will be
        /// merged into one chat tag.
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / ShiftWorldBorder $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ShiftWorldBorder`
        /// 
        /// ##### Description
        /// Changes a player's world border
        /// size if they have one active.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / DisplaySignText $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplaySignText`
        /// 
        /// ##### Description
        /// Displays text on a sign
        /// to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Displaying a sign text will
        /// wipe any existing text on the
        /// sign for the player.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetSpeed $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetSpeed`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ExpandInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ExpandInv`
        /// 
        /// ##### Description
        /// Adds 3 more rows to a player's
        /// current inventory menu using the
        /// contents of the chest.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / LaunchProj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: LaunchProj`
        /// 
        /// ##### Description
        /// Launches a projectile from
        /// a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / NoProjColl $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: NoProjColl`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ShowDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ShowDisguise`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ParticleCuboidA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleCuboidA`
        /// 
        /// ##### Description
        /// Displays an animated particle
        /// cuboid to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / PlaySound $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: PlaySound`
        /// 
        /// ##### Description
        /// Plays a sound for a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetCompass $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetCompass`
        /// 
        /// ##### Description
        /// Sets the location compasses
        /// point to for a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / RngTeleport $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RngTeleport`
        /// 
        /// ##### **__Deprecated__**
        /// Use 'Set Variable: Set to Random
        /// Value' instead. This will likely
        /// be removed in 5.4.
        /// 
        /// ##### Description
        /// Teleports a player to a random
        /// location in the chest.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / MobDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: MobDisguise`
        /// 
        /// ##### Description
        /// Disguises a player as a mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / EnableBlocks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: EnableBlocks`
        /// 
        /// ##### Description
        /// Allows a player to place
        /// and break certain blocks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / OpenBlockInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: OpenBlockInv`
        /// 
        /// ##### Description
        /// Opens a container's inventory.
        /// Also works with crafting tables.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / ParticleCircleA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleCircleA`
        /// 
        /// ##### Description
        /// Displays an animated circle
        /// of particles to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / RemoveBossBar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RemoveBossBar`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetEquipment $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetEquipment`
        /// 
        /// ##### Description
        /// Sets the item in one of the
        /// equipment slots (armor and
        /// held items) of a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / GiveRngItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GiveRngItem`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetDropsEnabled $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetDropsEnabled`
        /// 
        /// ##### Description
        /// Sets whether a player drops
        /// their items when dead.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SendToPlot $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SendToPlot`
        /// 
        /// ##### Description
        /// Sends a player to another plot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The player will be prompted to
        /// be sent to the specified plot.
        /// The prompt may be denied by the player.
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / RemovePotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RemovePotion`
        /// 
        /// ##### Description
        /// Removes one or more potion
        /// effects from a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Only the potion's type needs
        /// to match; amplifier and duration
        /// are disregarded.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisplayFracture $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayFracture`
        /// 
        /// ##### Description
        /// Displays a block fracture
        /// effect at a location to a
        /// player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Disappears after 20s.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetEntityHidden $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetEntityHidden`
        /// 
        /// ##### Description
        /// Sets if an entity is hidden
        /// to a target.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / SetSidebar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetSidebar`
        /// 
        /// ##### Description
        /// Sets whether the scoreboard
        /// sidebar is visible to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / AllowDrops $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: AllowDrops`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / Vibration $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Vibration`
        /// 
        /// ##### Description
        /// Displays a Sculk Sensor
        /// vibration to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetSlot $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetSlot`
        /// 
        /// ##### Description
        /// Sets a player's selected
        /// hotbar slot.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ParticleRay $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleRay`
        /// 
        /// ##### Description
        /// Displays a ray of particles
        /// to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ParticleCuboid $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleCuboid`
        /// 
        /// ##### Description
        /// Displays a particle cuboid as a
        /// solid, hollow or wireframe
        /// shape to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SendMessageSeq $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SendMessageSeq`
        /// 
        /// ##### Description
        /// Sends a series of messages
        /// in chat to a player, with a
        /// delay after each message.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetNamePrefix $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetNamePrefix`
        /// 
        /// ##### Description
        /// Sets the prefix or suffix
        /// for the player's name.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / ClearDispBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ClearDispBlock`
        /// 
        /// ##### Description
        /// Displays the real block at a
        /// location to a player, effectively
        /// removing any client-side blocks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetRainLevel $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetRainLevel`
        /// 
        /// ##### Description
        /// Sets the heaviness of rain and
        /// storm visible to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The player's weather must be
        /// set to Downfall to take effect.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / Undisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Undisguise`
        /// 
        /// ##### Description
        /// Removes a player's disguise.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / ParticleSpiralA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleSpiralA`
        /// 
        /// ##### Description
        /// Displays an animated spiral of
        /// particles to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / InstantRespawn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: InstantRespawn`
        /// 
        /// ##### Description
        /// Sets if a player is instantly 
        /// respawned upon dying.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Also respawns a player if
        /// they are dead.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetScore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetScore`
        /// 
        /// ##### Description
        /// Sets a score on the
        /// scoreboard.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetNameColor $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetNameColor`
        /// 
        /// ##### Description
        /// Sets the color a player's
        /// name tag appears in.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetAtkSpeed $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetAtkSpeed`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / DisablePvp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisablePvp`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetTickRate $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetTickRate`
        /// 
        /// ##### Description
        /// Changes the tick rate of a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Lower tick rate makes the player
        /// move in slower speed, and makes
        /// various animations slower.
        /// 
        /// ##### Additional Info
        /// Tick rate of 0 does not affect
        /// the player's movement and entities
        /// will appear to be broken.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / PlayEntitySound $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: PlayEntitySound`
        /// 
        /// ##### Description
        /// Plays a sound that follows a
        /// moving entity or player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Some sounds are not supported
        /// to be played from an entity.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ReplaceProj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ReplaceProj`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetExp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetExp`
        /// 
        /// ##### Description
        /// Sets a player's experience
        /// level, points or progress.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / Kbattribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Kbattribute`
        /// 
        /// ##### Description
        /// Sets one of the player's knockback-related
        /// attributes such as knockback resistance.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / MovementAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: MovementAttribute`
        /// 
        /// ##### Description
        /// Sets one of the player's movement-related
        /// attributes, such as walking speed
        /// and jump height.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ParticleSpiral $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleSpiral`
        /// 
        /// ##### Description
        /// Displays a spiral of particles
        /// at a location to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / FallingAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: FallingAttribute`
        /// 
        /// ##### Description
        /// Sets one of the player's falling-related
        /// attributes, such as gravity
        /// and fall damage multiplier.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// This is a temporary code action
        /// to allow early migration.
        /// Type /migration for more info.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetAllowFlight $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetAllowFlight`
        /// 
        /// ##### Description
        /// Sets whether a player
        /// is able to enter and exit
        /// flight mode by double
        /// tapping jump.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetMaxHealth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetMaxHealth`
        /// 
        /// ##### Description
        /// Sets a player's maximum
        /// health.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / RemoveBossBar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RemoveBossBar`
        /// 
        /// ##### Description
        /// Removes a boss health bar
        /// from a player's screen.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / SetFogDistance $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetFogDistance`
        /// 
        /// ##### Description
        /// Sets how far the fog is
        /// displayed to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Fog distance cannot be higher
        /// than the client's View Distance.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / AdventureMode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: AdventureMode`
        /// 
        /// ##### Description
        /// Sets a player's game
        /// mode to Adventure.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SpectatorMode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SpectatorMode`
        /// 
        /// ##### Description
        /// Sets a player's game
        /// mode to Spectator.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / DispHeadTexture $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DispHeadTexture`
        /// 
        /// ##### Description
        /// Changes a head's texture at
        /// a location for a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The texture is client-side but
        /// will NOT revert if interacted with.
        /// 
        /// ##### Additional Info
        /// Requires a player head to
        /// already be placed.
        /// 
        /// ##### Additional Info
        /// Works on client-side player heads.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / ClearPotions $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ClearPotions`
        /// 
        /// ##### Description
        /// Removes all active potion
        /// effects from a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetTabListInfo $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetTabListInfo`
        /// 
        /// ##### Description
        /// Sets the text to be displayed
        /// above or below a player's player
        /// list shown when pressing Tab.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Multiple variables (of any type)
        /// will be merged into one message.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / EnablePvp $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: EnablePvp`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / HideDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: HideDisguise`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ScoreLineFormat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ScoreLineFormat`
        /// 
        /// ##### Description
        /// Sets the number format of a
        /// single line in the player's
        /// scoreboard.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Put an appropriate value in the chest
        /// depending on the number format type.
        /// 
        /// ##### Additional Info
        /// Line number format overrides the default
        /// number format of the whole scoreboard.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetBossBar $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetBossBar`
        /// 
        /// ##### Description
        /// Creates or modifies a custom boss
        /// health bar at the top of a player's
        /// screen.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Boss bar positions are relative;
        /// unused positions are not shown.
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / SetSkin $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetSkin`
        /// 
        /// ##### Description
        /// Sets the player's skin.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// To get a skin use
        /// /item head &lt;player name&ht;
        /// OR /item mshead
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / SpectatorCollision $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SpectatorCollision`
        /// 
        /// ##### Description
        /// Toggles whether a player
        /// collides with blocks in
        /// spectator mode.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / SetNameVisible $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetNameVisible`
        /// 
        /// ##### Description
        /// Sets whether a player's
        /// name tag is visible.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetInvulTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetInvulTicks`
        /// 
        /// ##### Description
        /// Sets the currently
        /// remaining ticks until a
        /// player can next be hurt.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// This value is set to 10
        /// upon taking damage.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / EnableFlight $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: EnableFlight`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetStingsStuck $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetStingsStuck`
        /// 
        /// ##### Description
        /// Sets the amount of bee stings
        /// sticking out of a player's
        /// character model.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / RemoveScore $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RemoveScore`
        /// 
        /// ##### Description
        /// Removes a score from
        /// the scoreboard.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisallowDrops $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisallowDrops`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetExhaustion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetExhaustion`
        /// 
        /// ##### Description
        /// Sets a player's exhaustion level.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ParticleCircle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleCircle`
        /// 
        /// ##### Description
        /// Displays a circle of particles
        /// to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / DisplayBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayBlock`
        /// 
        /// ##### Description
        /// Displays a block at a location to
        /// a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// This block is client-side. The
        /// actual block is not changed.
        /// 
        /// ##### Additional Info
        /// Interacting with a client-side
        /// block causes it to update.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / RideEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RideEntity`
        /// 
        /// ##### Description
        /// Mounts a player on top of
        /// another player or entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / WeatherRain $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: WeatherRain`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / RmWorldBorder $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RmWorldBorder`
        /// 
        /// ##### Description
        /// Removes a player's world border.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / ResourcePack $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ResourcePack`
        /// 
        /// ##### Description
        /// Send a resource pack to the player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Resource packs may only be
        /// sent once per player.
        /// 
        /// ##### Additional Info
        /// Resource packs are limited to
        /// 10 MiB in size.
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / SetInvName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetInvName`
        /// 
        /// ##### Description
        /// Renames a player's current
        /// inventory menu.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / GiveExhaustion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GiveExhaustion`
        /// 
        /// ##### Description
        /// Adds exhaustion to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / Teleport $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Teleport`
        /// 
        /// ##### Description
        /// Teleports a player to
        /// a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetAllowPVP $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetAllowPVP`
        /// 
        /// ##### Description
        /// Sets whether a player can
        /// hurt or be hurt by other
        /// players.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisableFlight $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisableFlight`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetVisualFire $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetVisualFire`
        /// 
        /// ##### Description
        /// Sets whether a player
        /// should appear on fire.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The affected player's
        /// fire ticks won't change
        /// and won't take any damage.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetDisguiseVisible $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetDisguiseVisible`
        /// 
        /// ##### Description
        /// Sets a player's ability to
        /// see their own disguise. It
        /// is recommended that it is
        /// almost always hidden.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / SetArrowsStuck $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetArrowsStuck`
        /// 
        /// ##### Description
        /// Sets the amount of arrows
        /// sticking out of a player's
        /// character model.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// These arrows cannot be
        /// used or picked up.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / GetItemCooldown $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GetItemCooldown`
        /// 
        /// ##### Description
        /// Gets the remaining cooldown
        /// on an item type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The cooldown applies to all items
        /// of the given type.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetItems`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / KeepInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: KeepInv`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ReplaceItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ReplaceItems`
        /// 
        /// ##### Description
        /// Replaces items in a player's
        /// inventory with the given item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SendMessage $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SendMessage`
        /// 
        /// ##### Description
        /// Sends a chat message to a
        /// player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Multiple values (of any type)
        /// will be merged together.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetSlotItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetSlotItem`
        /// 
        /// ##### Description
        /// Sets the item in a slot
        /// of a player's inventory.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / PlaySoundSeq $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: PlaySoundSeq`
        /// 
        /// ##### Description
        /// Plays a sequence of sounds
        /// to a player, with a delay
        /// between each sound.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ParticleLineA $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleLineA`
        /// 
        /// ##### Description
        /// Displays an animated line of
        /// particles between two locations
        /// to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / Respawn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: Respawn`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetInvName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetInvName`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetItemCooldown $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetItemCooldown`
        /// 
        /// ##### Description
        /// Applies a cooldown visual effect
        /// to an item type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The cooldown applies to all items
        /// of the given type.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetPlayerWeather $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetPlayerWeather`
        /// 
        /// ##### Description
        /// Sets the type of weather
        /// visible to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SendHover $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SendHover`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SetShoulder $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetShoulder`
        /// 
        /// ##### Description
        /// Displays a parrot on the targets'
        /// shoulders.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Parrot will not dismount
        /// automatically.
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetAirTicks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetAirTicks`
        /// 
        /// ##### Description
        /// Sets a player's remaining
        /// breath ticks.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Each bubble is equal
        /// to 30 air ticks.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DisplayPickup $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayPickup`
        /// 
        /// ##### Description
        /// Displays a pickup animation
        /// of one entity being collected
        /// by another entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Mythic</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_mythic"))]
        $($item)*
    },
    ( { PlayerAction / SetWorldBorder $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetWorldBorder`
        /// 
        /// ##### Description
        /// Creates a world border only
        /// visible to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / SetPlayerTime $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetPlayerTime`
        /// 
        /// ##### Description
        /// Sets the time of day visible
        /// to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Add 24000 ticks to add
        /// one moon phase.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / GiveFood $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GiveFood`
        /// 
        /// ##### Description
        /// Adds food to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / NatRegen $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: NatRegen`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / GivePotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: GivePotion`
        /// 
        /// ##### Description
        /// Gives one or more potion
        /// effects to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / RemoveItems $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RemoveItems`
        /// 
        /// ##### Description
        /// Removes items from a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / BoostElytra $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: BoostElytra`
        /// 
        /// ##### Description
        /// Boosts a player's elytra
        /// using a firework rocket.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The player must be
        /// gliding with an elytra.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SaveInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SaveInv`
        /// 
        /// ##### Description
        /// Saves a player's inventory.
        /// It can be loaded later with
        /// 'Load Saved Inventory'.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / OpenBook $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: OpenBook`
        /// 
        /// ##### Description
        /// Opens a written book
        /// menu for a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Opened book and quills
        /// cannot be edited.
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / SetHealth $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetHealth`
        /// 
        /// ##### Description
        /// Sets a player's current
        /// health.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Does not trigger a heal
        /// or damage event.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / BlockDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: BlockDisguise`
        /// 
        /// ##### Description
        /// Disguises a player as a block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / RollbackBlocks $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: RollbackBlocks`
        /// 
        /// ##### Description
        /// Undoes the interactions with
        /// blocks by a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The rollback time argument
        /// specifies how far back
        /// in time block changes should
        /// be reverted.
        /// 
        /// ##### Additional Info
        /// Please note that the rollback
        /// time argument is in
        /// SECONDS!
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / NoDeathDrops $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: NoDeathDrops`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / WalkSpeed $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: WalkSpeed`
        /// 
        /// ##### Description
        /// Sets a player's walk
        /// speed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / SetCollidable $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetCollidable`
        /// 
        /// ##### Description
        /// Sets whether a player is
        /// able to collide with other
        /// entities.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Useful in conjunction
        /// with projectiles.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / LaunchFwd $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: LaunchFwd`
        /// 
        /// ##### Description
        /// Launches a player forward
        /// or backward.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A negative launch power will
        /// launch the player backward.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / SetFallDistance $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetFallDistance`
        /// 
        /// ##### Description
        /// Sets a player's fall distance,
        /// affecting fall damage upon
        /// landing.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / CreativeMode $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: CreativeMode`
        /// 
        /// ##### Description
        /// Sets a player's game
        /// mode to Creative.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / AttackAnimation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: AttackAnimation`
        /// 
        /// ##### Description
        /// Makes a player perform
        /// an attack animation.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Noble</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_noble"))]
        $($item)*
    },
    ( { PlayerAction / DisplayHologram $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DisplayHologram`
        /// 
        /// ##### Description
        /// Displays a floating name tag
        /// at a location to a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If the player already has
        /// a hologram displayed at the
        /// location, its text is replaced.
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / SetAbsorption $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetAbsorption`
        /// 
        /// ##### Description
        /// Sets a player's absorption
        /// health (golden hearts).
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// The target does not need to
        /// have an absorption effect.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / DeathDrops $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: DeathDrops`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / ShowInv $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ShowInv`
        /// 
        /// ##### Description
        /// Opens a custom inventory
        /// for a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / SetFoodLevel $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetFoodLevel`
        /// 
        /// ##### Description
        /// Sets a player's food hunger level.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / PlayerDisguise $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: PlayerDisguise`
        /// 
        /// ##### Description
        /// Disguises a player as another
        /// player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// To get a skin use /item head &lt;player name&ht;
        /// OR /item mshead
        /// 
        /// ##### Requires Rank: **<u>Overlord</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_overlord"))]
        $($item)*
    },
    ( { PlayerAction / SetSaturation $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SetSaturation`
        /// 
        /// ##### Description
        /// Sets a player's saturation
        /// level.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / WeatherClear $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: WeatherClear`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { PlayerAction / SendTitle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: SendTitle`
        /// 
        /// ##### Description
        /// Displays text in the center
        /// of a player's screen.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Requires Rank: **<u>Emperor</u>**
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[doc(cfg(feature = "rank_emperor"))]
        $($item)*
    },
    ( { PlayerAction / ScoreDefFormat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ScoreDefFormat`
        /// 
        /// ##### Description
        /// Sets the default number format
        /// of the player's scoreboard.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Put an appropriate value in the chest
        /// depending on the number format type.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / StopSound $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: StopSound`
        /// 
        /// ##### Description
        /// Stops all or specific sounds
        /// for a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Stopping sounds from the
        /// Master source stops every
        /// sound effect.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / HealthAttribute $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: HealthAttribute`
        /// 
        /// ##### Description
        /// Sets one of the player's health-related
        /// attributes such as max health
        /// and armor defense points.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { PlayerAction / ParticleLine $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `PlayerAction: ParticleLine`
        /// 
        /// ##### Description
        /// Displays a line of particles
        /// between two locations to
        /// a player.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / LastMob $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: LastMob`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / RandomPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: RandomPlayer`
        /// 
        /// ##### Description
        /// Creates a selection using
        /// one or more random
        /// players in the game.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / LastEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: LastEntity`
        /// 
        /// ##### Description
        /// Creates a selection using
        /// the most recently spawned
        /// entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / Shooter $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: Shooter`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / AllMobs $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: AllMobs`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / EntityName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: EntityName`
        /// 
        /// ##### Description
        /// Creates a selection using all
        /// entities in the game whose
        /// name or UUID matches.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / FilterRandom $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: FilterRandom`
        /// 
        /// ##### Description
        /// Filters the selection by
        /// randomly picking one or
        /// more objects from it.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / DefaultEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: DefaultEntity`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / PlayerName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: PlayerName`
        /// 
        /// ##### Description
        /// Creates a selection using all
        /// players in the game whose
        /// name or UUID matches.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / AllEntities $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: AllEntities`
        /// 
        /// ##### Description
        /// Creates a selection of
        /// all entities in the game.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / Damager $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: Damager`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / FilterDistance $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: FilterDistance`
        /// 
        /// ##### Description
        /// Filters the selection to the
        /// objects that are nearest
        /// or farthest to a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / FilterRay $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: FilterRay`
        /// 
        /// ##### Description
        /// Filters the selected objects
        /// to the objects that intersect
        /// a ray.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / Reset $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: Reset`
        /// 
        /// ##### Description
        /// Deactivates the selection.
        /// Code that follows will run
        /// normally with event targets.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / EventTarget $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: EventTarget`
        /// 
        /// ##### Description
        /// Creates a selection using
        /// a target involved in this
        /// Event.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / Killer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: Killer`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / Victim $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: Victim`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / EntitiesCond $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: EntitiesCond`
        /// 
        /// ##### Description
        /// Creates a selection of
        /// all entities in the game
        /// that meet a condition.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Can be inverted with the
        /// NOT Arrow.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / AllPlayers $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: AllPlayers`
        /// 
        /// ##### Description
        /// Creates a selection of
        /// all players in the game.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / Invert $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: Invert`
        /// 
        /// ##### Description
        /// Creates a new selection by
        /// inverting the selection that
        /// is currently active.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If the current selection
        /// contains players, all other
        /// players are selected.
        /// 
        /// ##### Additional Info
        /// If the current selection
        /// contains entities, all other
        /// entities are selected.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / RandomEntity $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: RandomEntity`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / FilterCondition $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: FilterCondition`
        /// 
        /// ##### Description
        /// Filters the selection to the
        /// objects that meet a certain
        /// condition.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Can be inverted with the
        /// NOT Arrow.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / MobsCond $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: MobsCond`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / FilterSort $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: FilterSort`
        /// 
        /// ##### Description
        /// Filters the selection by
        /// sorting the value of each
        /// object in order.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / Projectile $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: Projectile`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / DefaultPlayer $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: DefaultPlayer`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { SelectObject / PlayersCond $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: PlayersCond`
        /// 
        /// ##### Description
        /// Creates a selection of
        /// all players in the game
        /// that meet a condition.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Can be inverted with the
        /// NOT Arrow.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { SelectObject / MobName $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `SelectObject: MobName`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfEntity / IsVehicle $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsVehicle`
        /// 
        /// ##### Description
        /// Checks if an entity
        /// is a boat or minecart.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsGrounded $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsGrounded`
        /// 
        /// ##### Description
        /// Checks if an entity is
        /// supported by a block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsType`
        /// 
        /// ##### Description
        /// Checks if an entity is the
        /// given type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsProj $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsProj`
        /// 
        /// ##### Description
        /// Checks if an entity
        /// is a projectile.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsMob $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsMob`
        /// 
        /// ##### Description
        /// Checks if an entity
        /// is a mob.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / HasCustomTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: HasCustomTag`
        /// 
        /// ##### Description
        /// Checks if an entity has a
        /// given custom tag, and (if
        /// provided) whether the tag
        /// matches the given value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsSheared $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsSheared`
        /// 
        /// ##### Description
        /// Checks if a sheep is
        /// sheared.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Sheep
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsItem $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsItem`
        /// 
        /// ##### Description
        /// Checks if an entity
        /// is an item.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsRiding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsRiding`
        /// 
        /// ##### Description
        /// Checks if an entity is riding
        /// another entity.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / Exists $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: Exists`
        /// 
        /// ##### Description
        /// Checks if an entity still
        /// exists in the world.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// Entities which have been removed
        /// still remain in selections.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsNear $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsNear`
        /// 
        /// ##### Description
        /// Checks if an entity is within a
        /// range of a location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / HasPotion $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: HasPotion`
        /// 
        /// ##### Description
        /// Checks if an entity has a
        /// potion effect of a certain
        /// type active.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / IsRiding $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: IsRiding`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfEntity / StandingOn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: StandingOn`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfEntity / NameEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: NameEquals`
        /// 
        /// ##### Description
        /// Checks if an entity's name or
        /// custom name is equal to the
        /// given text.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfEntity / StandingOn $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfEntity: StandingOn`
        /// 
        /// ##### Description
        /// Checks if an entity is standing on
        /// the given block or location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { CallFunction / Dynamic $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `CallFunction: Dynamic`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { Control / StopRepeat $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Control: StopRepeat`
        /// 
        /// ##### Description
        /// Stops a Repeat sequence and
        /// continues to the next code block.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Repeat
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Control / Return $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Control: Return`
        /// 
        /// ##### Description
        /// Skips the rest of a Function
        /// sequence and returns to the
        /// block it was called from.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// This occurs automatically
        /// at the end of a Function.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Control / ReturnNTimes $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Control: ReturnNTimes`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { Control / Skip $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Control: Skip`
        /// 
        /// ##### Description
        /// Skips the rest of this repeat
        /// statement's code and continues
        /// to the next repetition.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Repeat
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Control / End $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Control: End`
        /// 
        /// ##### Description
        /// Stops the current event
        /// thread. Any code after this
        /// block will not be executed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// This occurs automatically
        /// when a thread has no valid
        /// event targets or selection.
        /// A player target is invalid
        /// if they have left the game.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { Control / Wait $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `Control: Wait`
        /// 
        /// ##### Description
        /// Pauses the current code
        /// sequence for a duration of
        /// ticks, seconds, or minutes.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// It is not possible to wait
        /// fractions of a tick.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: `
        /// 
        /// ##### Description
        /// Checks if a number value is
        /// less than or equal to another
        /// number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / ItemHasEnchant $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: ItemHasEnchant`
        /// 
        /// ##### Description
        /// Checks if an item has a
        /// given enchantment, or,
        /// if no enchantment is specified,
        /// checks if it has any.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// A level of 0 will work
        /// for any level.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / ItemIsBlock $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: ItemIsBlock`
        /// 
        /// ##### Description
        /// Checks if an item is
        /// able to be placed.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / DictValueEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: DictValueEquals`
        /// 
        /// ##### Description
        /// Checks if a dictionary's value
        /// for the given key is equal to
        /// any of the given values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / ItemHasTag $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: ItemHasTag`
        /// 
        /// ##### Description
        /// Checks if an item has a
        /// given custom tag, and (if
        /// provided) whether the tag
        /// matches the given value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / StringMatches $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: StringMatches`
        /// 
        /// ##### Description
        /// Checks if a string value matches
        /// other values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / ListIsEmpty $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: ListIsEmpty`
        /// 
        /// ##### Description
        /// Checks if a list is empty.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / StartsWith $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: StartsWith`
        /// 
        /// ##### Description
        /// Checks if the first part of
        /// a string value matches a
        /// certain string.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / ListValueEq $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: ListValueEq`
        /// 
        /// ##### Description
        /// Checks if a list's value at an
        /// index is equal to a value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / VarIsType $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: VarIsType`
        /// 
        /// ##### Description
        /// Checks if a value is of a
        /// certain type.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / TextMatches $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: TextMatches`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfVariable / IsNear $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: IsNear`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfVariable / InRange $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: InRange`
        /// 
        /// ##### Description
        /// Checks if a number value is
        /// in between 2 other numbers or
        /// a location value is within the
        /// region of 2 other locations.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Works With
        /// - Number
        /// - Location
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / VarExists $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: VarExists`
        /// 
        /// ##### Description
        /// Checks if a variable exists.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / Legacy $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: Legacy`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfVariable / BlockIsSolid $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: BlockIsSolid`
        /// 
        /// ##### Description
        /// Checks if a material will collide
        /// with entities.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / ItemEquals $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: ItemEquals`
        /// 
        /// ##### Description
        /// Works the same as Value =
        /// but has a few extra options
        /// for item comparison.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / ListContains $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: ListContains`
        /// 
        /// ##### Description
        /// Checks if any of a list's contents
        /// match the given value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ##### Additional Info
        /// If multiple values are given, the
        /// condition will return true if one
        /// of the values are found.
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / Legacy $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: Legacy`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfVariable / InRange $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: InRange`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { IfVariable / LocIsNear $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: LocIsNear`
        /// 
        /// ##### Description
        /// Checks if a location is
        /// near another location.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / Contains $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: Contains`
        /// 
        /// ##### Description
        /// Checks if a string value
        /// contains another string.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: `
        /// 
        /// ##### Description
        /// Checks if a value does not
        /// equal another value.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: `
        /// 
        /// ##### Description
        /// Checks if a number value is
        /// less than another number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: `
        /// 
        /// ##### Description
        /// Checks if a value is equal
        /// to one of the given values.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: `
        /// 
        /// ##### Description
        /// Checks if a number value is
        /// greater than another number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / EndsWith $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: EndsWith`
        /// 
        /// ##### Description
        /// Checks if the last part of
        /// a string value matches a
        /// certain string.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable /  $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: `
        /// 
        /// ##### Description
        /// Checks if a number value
        /// is greater than or equal to
        /// another number.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { IfVariable / DictHasKey $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `IfVariable: DictHasKey`
        /// 
        /// ##### Description
        /// Checks if a dictionary has
        /// the given key.
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        $($item)*
    },
    ( { StartProcess / Dynamic $( { $($tagident:ident = $tagvalue:ident),* } )? } { $($item:tt)* } ) => {
        /// ### `StartProcess: Dynamic`
        /// 
        /// ##### **__Deprecated__**
        /// *Assumed.*
        /// 
        /// ##### Description
        /// *No description available.*
        $(
            /// 
            /// ##### Tags
            $(
                #[doc = crate::core::concat!("- `", crate::core::stringify!($tagident), "`: `", crate::core::stringify!($tagvalue), "`")]
            )*
        )?
        /// 
        /// ## 
        /// *\[2024-11-09 18:22:38 UTC\] Automatically generated from the action dump.*
        #[cfg(any(feature = "rank_none", feature = "rank_noble", feature = "rank_emperor", feature = "rank_mythic", feature = "rank_overlord", feature = "rank_dev"))]
        #[deprecated] 
        $($item)*
    },
    ( { $a:ident / $b:ident } { $($_:tt)* } ) => {
        compile_error!(concat!("Unknown target `", stringify!($a), "/", stringify!($b), "`"));
    }
}
