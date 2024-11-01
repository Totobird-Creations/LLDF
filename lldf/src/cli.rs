use std::env;
use std::path::PathBuf;
use std::sync::atomic::{ AtomicBool, Ordering };

pub use clap::Parser;
use clap::{ ColorChoice, Subcommand };
use supports_color as supports_colour;


#[derive(Parser)]
#[command(styles=get_styles())]
#[command(arg_required_else_help = true)]
#[command(about, long_about = None)]
#[clap(disable_version_flag = true)]
pub struct Cli {

    /// Colour mode.
    #[arg(long="colour", require_equals=true)]
    #[arg(value_name="WHEN")]
    #[arg(default_value_t=ColorChoice::Auto)]
    pub colour: ColorChoice,

    /// Disable decompiler output.
    #[arg(short='q', long="quiet")]
    pub quiet: bool,

    #[command(subcommand)]
    command : Command

}
#[derive(Subcommand)]
pub enum Command {

    /// Decompiles LLVM to DiamondFire code and sends it to the given endpoint.
    Build {

        /// The path to an LLVM bitcode file, LLVM ir file, or Rust project root.
        #[arg(requires = "endpoint")]
        path : PathBuf,

        /// Dumps the DiamondFire code to the console.
        #[arg(long, group = "endpoint")]
        dump : bool,

        /// Send the DiamondFire code to CCAPI as a placement order. https://modrinth.com/mod/codeclient
        #[arg(long, group = "endpoint")]
        ccapi : bool

    }

}


static COLOUR_CACHED  : AtomicBool = AtomicBool::new(false);
static COLOUR_ENABLED : AtomicBool = AtomicBool::new(false);
/// Return whether or not to use colours.
/// 
/// - If `--colour=never` is passed, `false`.
/// - If `--colour==always` is passed, `true`.
/// - Else, automatically detect from the terminal.
fn enable_colour() -> bool {
    if (COLOUR_CACHED.load(Ordering::Relaxed)) {
        COLOUR_ENABLED.load(Ordering::Relaxed)
    } else {
        let c = if (env::args().any(|a| a == "--colour=never"  )) { false }
            else if (env::args().any(|a| a == "--colour=always" )) { true }
            else if let Some(supports_colour::ColorLevel { has_basic : true, .. }) = supports_colour::on(supports_colour::Stream::Stdout) { true }
            else { false };
        COLOUR_ENABLED.store(c, Ordering::Relaxed);
        COLOUR_CACHED.store(true, Ordering::Relaxed);
        c
    }
    
}
fn get_styles() -> clap::builder::Styles {
    use clap::builder::styling::{Style, Color, AnsiColor};
    let mut c = clap::builder::Styles::plain();
    if (enable_colour()) {c = c
        .header(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .error(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
        .usage(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .literal(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .valid(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .invalid(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Yellow))));
    }
    c
}
pub(crate) macro c {
    ($($code:tt),*) => {
        if (crate::cli::enable_colour()) {
            ::const_str::concat!($("\x1b[", stringify!($code), "m"),*)
        } else { "" }
    },
    (@ $($code:expr),*) => {
        if (crate::cli::enable_colour()) {
            let mut s = String::new();
            $( s += &format!("\x1b[{}m", $code) ; )*
            s
        } else { String::new() }
    }
}


impl Cli { pub fn run(&self) -> () {
    match (&self.command) {

        Command::Build { path, dump, ccapi } => { match ((dump, ccapi)) {

            // Dump
            (true, false) => crate::build::run(path, |templates| {
                for template in templates {
                    println!();
                    for (i, codeblock) in template.blocks.iter().enumerate() {
                        if (i > 0) { print!("  "); }
                        else { print!("{}", c!(1)) }
                        println!("{}{:?}{}", c!(96), codeblock, c!(0));
                    }
                }
                Ok(())
            }),

            // CCAPI
            (false, true) => crate::build::run(path, |templates| crate::build::ccapi_submit_templates(&templates)),

            _ => unreachable!()
        } }

    }
} }
