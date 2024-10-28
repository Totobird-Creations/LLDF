pub mod codegen;
pub mod parse;


use std::fs;
use std::path::Path;
use std::process::{ self, Command };
use std::error::Error;

use llvm_ir::Module;

use serde::Deserialize as Deser;
use toml;


#[derive(Deser)]
struct CargoToml {
    package : CargoTomlPackage
}
#[derive(Deser)]
struct CargoTomlPackage {
    name : String
}


pub fn load_module<P : AsRef<Path>>(path : P) -> Result<Module, String> {
    let path = path.as_ref();
    if (path.is_file()) {

        // Try LLVM textual IR.
        if let Ok(module) = Module::from_ir_path(path) {
            return Ok(module);
        }

        // Try LLVM bitcode.
        return Module::from_bc_path(path);

    } else if (path.is_dir()) {

        // Try Rust project.
        let cargo_toml_path = path.join("Cargo.toml");
        if (cargo_toml_path.is_file()) {
            let cargo_toml             = fs::read_to_string(cargo_toml_path).map_err(|e| e.to_string())?;
            let cargo_toml : CargoToml = toml::from_str(&cargo_toml).map_err(|e| e.to_string())?;
            let mut cmd = Command::new("cargo")
                .args(["build", "--target=wasm32-unknown-unknown", "--release"])
                .current_dir(path)
                .env("RUSTFLAGS", "--emit=llvm-bc")
                .spawn().map_err(|e| e.to_string())?;
            let code = cmd.wait().map_err(|e| e.to_string())?;
            code.exit_ok().map_err(|e| e.to_string())?;
            return Module::from_bc_path(path.join("target/wasm32-unknown-unknown/release/deps").join(cargo_toml.package.name).with_extension("bc"));
        }

    }

    Err("Target module is not a valid LLVM IR file or recognised project format".to_string())
}


pub fn run<P : AsRef<Path>>(path : P) -> () {
    // Get the module.
    let module = match (crate::build::load_module(path)) {
        Err(err) => { eprintln!("error: {}", err); process::exit(1) },
        Ok(module) => module
    };

    if let Err(e) = build_modules(&vec![ module ]) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}



pub fn build_modules(modules : &Vec<Module>) -> Result<(), Box<dyn Error>> {
    for module in modules {
        parse::parse_module(module)?;
    }
    Ok(())
}
