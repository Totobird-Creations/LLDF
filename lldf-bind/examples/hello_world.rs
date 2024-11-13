#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {


    let a = "Hello, World!";
    let b = Item::beacon();
    let c = AdvancementFrame::Goal;
    default.send_advancement(a, b, c);


    default.set_gamemode_creative();


    let mut a = List::new();
    a.push(String::from("Hello,"));
    let mut b = a.clone();
    b.push(String::from("World!"));

    default.send_message(a);
    default.send_message(b);


    let a = unsafe{ Colour::from_hexcode_unchecked("#ff7f00") }.rgb();
    default.send_message(a.0);
    default.send_message(a.1);
    default.send_message(a.2);


    let s = Sound::anvil_land()
        .with_pitch(2.0)
        .with_volume(1000000.0);
    default.play_sound(Location::new(0.0, 0.0, 0.0, 0.0, 0.0), s, SoundChannel::Blocks);


    let v = Vector::new([1.0.into(), 2.0.into(), 3.0.into()]);
    default.send_message(v.name().to_string());


}

