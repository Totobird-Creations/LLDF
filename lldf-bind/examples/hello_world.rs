#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {

    //let a = "Hello, World!";
    //let b = Item::allay_spawn_egg();
    //let c = AdvancementFrame::Goal;
    //default.send_advancement(
    //    a,
    //    b,
    //    c
    //);

    //default.play_sound(
    //    default.eye_location(),
    //    Sound::entity_allay_death().with_pitch(2.0).with_volume(2.0),
    //    SoundChannel::Jukebox
    //);

    //default.set_gamemode_creative();

    let a = Vector::<3>::new([1.0.into(), 2.0.into(), 3.0.into()]);
    let b = Vector::<5>::new([1.0.into(), 2.0.into(), 3.0.into(), 4.0.into(), 5.0.into()]);
    default.send_message(a.y());          // Prints 2
    default.send_message(a.lane(1usize)); // Prints 2
    default.send_message(b.lane(3usize)); // Prints 4

}

