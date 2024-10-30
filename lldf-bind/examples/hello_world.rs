#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerJoin)]
fn player_join() {
    let a = String::from("Test");
    let b = Text::from(a);

    let all_players = Game::all_players();

    //let uuids = all_players.uuids();

    all_players.send_message(b);
    //all_players.send_message(Text::from(&uuids[0usize]));
    //all_players.send_message(Text::from(&uuids[1usize]));

}
