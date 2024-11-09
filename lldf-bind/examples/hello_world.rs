#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    default.send_advancement("Hello, World!", Item::allay_spawn_egg(), AdvancementFrame::Goal);
    default.play_sound(default.eye_location(), Sound::entity_allay_death().with_pitch(0.0).with_volume(2.0), SoundChannel::Jukebox);
    default.set_gamemode_creative();
}

