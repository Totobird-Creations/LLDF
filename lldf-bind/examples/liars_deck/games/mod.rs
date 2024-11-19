use super::*;


use lldf_bind::prelude::*;
use core::mem::MaybeUninit;


// TODO: Fix Clone derive macro
pub struct Game {
    players : List<Uuid>,
    turn    : Uuid
}
impl Clone for Game {
    fn clone(&self) -> Self {
        Self { players : self.players.clone(), turn : self.turn.clone() }
    }
}


static mut RUNNING_GAMES : MaybeUninit<List<Game>> = MaybeUninit::uninit();


pub fn setup() {
    *unsafe{ RUNNING_GAMES.assume_init_mut() } = List::new();
}

pub fn remove(player : &PlayerSel) {
    // TODO
}
