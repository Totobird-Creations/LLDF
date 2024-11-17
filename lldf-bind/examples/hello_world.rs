#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerJoin)]
fn player_join(default : PlayerSel) { // Enums aren't working
    default.send_message( Sound::allay_death()                   .kind().to_string());
    default.send_message( Sound::custom("lldftest:custom_sound") .kind().to_string());
}


/*#[event(PlayerJoin)]
fn player_join(default : PlayerSel) { // Fibonacci
    let mut a = 1usize;
    let mut b = 1usize;
    let mut i = 0usize;
    while (i < 10) {
        default.send_message(UInt::from(a));
        let c = a + b;
        a = b;
        b = c;
        i = i + 1;
    }
}*/


/*#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) { // Ear pain
    default.send_message("Hello is me holben".to_string());
    default.send_message("holbeanz".to_string());
    default.send_message("ok die now".to_string());

    let loc     = Location::new(0.0, 0.0, 0.0, 0.0, 0.0);
    let sound   = Sound::ender_dragon_death().with_volume(1000000.0);
    let channel = unsafe{ SoundChannel::from_string_unchecked("Master".to_string()) };
    loop {
        default.play_sound(loc.clone(), sound.with_pitch(0.0), channel);
        default.play_sound(loc.clone(), sound.with_pitch(0.75), channel);
        default.play_sound(loc.clone(), sound.with_pitch(1.0), channel);
        default.play_sound(loc.clone(), sound.with_pitch(1.5), channel);
        default.play_sound(loc.clone(), sound.with_pitch(2.0), channel);
        Game::sleep(1usize)
    }
}*/
