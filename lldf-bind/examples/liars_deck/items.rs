use lldf_bind::prelude::*;
use core::mem::MaybeUninit;


static mut QUEUE_ENABLED : MaybeUninit<Item> = MaybeUninit::uninit();
#[inline(always)]
pub fn queue_enabled() -> Item {
    unsafe{ QUEUE_ENABLED.assume_init_mut() }.clone()
}
#[inline(always)]
fn setup_queue_enabled() {
    *unsafe{ QUEUE_ENABLED.assume_init_mut() } = Item::tipped_arrow()
        .with_name(Text::from_minimsg("<#1fdf3f>Queued".to_string()))
        .with_colour(Colour::from_rgb(31usize, 223usize, 63usize));
}


static mut QUEUE_DISABLED : MaybeUninit<Item> = MaybeUninit::uninit();
#[inline(always)]
pub fn queue_disabled() -> Item {
    unsafe{ QUEUE_DISABLED.assume_init_mut() }.clone()
}
#[inline(always)]
fn setup_queue_disabled() {
    *unsafe{ QUEUE_DISABLED.assume_init_mut() } = Item::tipped_arrow()
        .with_name(Text::from_minimsg("<#df1f3f>Unqueued".to_string()))
        .with_colour(Colour::from_rgb(223usize, 31usize, 63usize));
}


pub fn setup() {
    setup_queue_enabled();
    setup_queue_disabled();
}
