#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerJoin)]
fn player_join(default : PlayerSel) {
    // Welcome
    /*default.send_message( Text::new() );
    default.send_message( Text::from_minimsg("<bold><underlined><light_purple>LLDF Example: Fibonacci") );
    default.send_message( Text::from_minimsg("<bold><#ff7fbf>LLDF<reset> is a project which aims to allow any programming language that can target <#bfffbf>LLVM<reset> to be used on <aqua>Diamond<gold>Fire<reset>.") );
    default.send_message( Text::from_minimsg("Some notable languages that can be used are <#ffbf7f>C<reset>, <#ffbf7f>C++<reset>, <#ffbf7f>Rust<reset>, and <#ffbf7f>Kotlin<reset>.") );
    default.send_message( Text::from_minimsg("This plot is written in <#ffbf7f>Rust<reset>.") );
    default.send_message( Text::new() );*/

    // Items and effects
    //default.give_item( Item::spyglass().with_name(Text::from_minimsg("<b><rainbow>Lookatdaboard!")) );

    //default.give_potion( Potion::invisibility(), Flag::False(), PotionParticles::None() );
    //                                           ^^^^^^^^^^^^^-Icon
    //default.set_feet_item( Item::leather_boots().with_colour(Colour::from_rgb(127usize, 191usize, 255usize)) );

    // Fibonacci
    let mut a = 1usize;
    let mut b = 1usize;
    let mut i = 0usize;
    loop {
        let c = a + b;
        default.send_actionbar( a.to_string() + " + " + b.to_string() + " = " + c.to_string() );
        a = b;
        b = c;
        i = i + 1;
        Thread::sleep(20usize)
    }

}

