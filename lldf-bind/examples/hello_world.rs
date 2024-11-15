#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerJoin)]
fn player_join(default : PlayerSel) {
    default.send_message(
        test(Sound::custom("minecraft:entity.allay.hurt")) // Does not have a name because it's a custom sound.
    );
    default.send_message(
        test(Sound::allay_hurt()) // Does have a name.
    );
    
}

#[no_mangle]
fn test(sound : Sound) -> String {
    match (sound.kind()) { // The name of the sound
        Some(kind) => kind.to_string(),
        None       => "No sound kind found".to_string()
    }
}
