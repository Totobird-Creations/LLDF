pub mod codegen;
pub mod parse;


use std::fs;
use std::path::Path;
use std::process::{ self, Command };
use std::error::Error;

use codegen::CodeLine;
use llvm_ir::Module;

use parse::ParsedModule;
use serde::Deserialize as Deser;
use toml;
use tungstenite as ws;


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


pub fn run<P : AsRef<Path>, F : Fn(Vec<CodeLine>) -> Result<(), Box<dyn Error>>>(path : P, submit : F) -> () {
    // Get the module.
    let module = match (load_module(path)) {
        Err(err) => { eprintln!("error: {}", err); process::exit(1) },
        Ok(module) => module
    };

    let modules = vec![ module ];

    let modules = match (build_modules(&modules)) {
        Err(err) => { eprintln!("error: {}", err); process::exit(1) },
        Ok(modules) => modules
    };

    let templates = match (build_templates(modules)) {
        Err(err) => { eprintln!("error: {}", err); process::exit(1) },
        Ok(modules) => modules
    };

    match (submit(templates)) {
        Err(err) => { eprintln!("error: {}", err); process::exit(1) },
        Ok(_) => { }
    };
}



pub fn build_modules(modules : &Vec<Module>) -> Result<Vec<ParsedModule>, Box<dyn Error>> {
    let mut parsed = Vec::new();
    for module in modules {
        parsed.push(parse::parse_module(module)?);
    }
    Ok(parsed)
}



pub fn build_templates(modules : Vec<ParsedModule>) -> Result<Vec<CodeLine>, Box<dyn Error>> {
    Ok(modules.into_iter().map(|module| module.functions).flatten().map(|function| function.line).collect::<Vec<_>>())
}



pub fn ccapi_submit_templates(templates : &Vec<CodeLine>) -> Result<(), Box<dyn Error>> {

    eprint!("Connecting to CCAPI... ");
    let Ok((mut sock, _)) = ws::connect("ws://localhost:31375") else { return Err("Failed to connect to CCAPI".into()) };
    eprintln!("Done");

    // Request required codeclient permissions.
    eprint!("Requesting CCAPI required permissions... ");
    let Ok(_) = sock.send(ws::Message::Text("scopes read_plot write_code clear_plot".to_string())) else { return Err("Failed to request required CCAPI permissions".into()) };
    let Ok(ws::Message::Text(resp)) = sock.read() else { return Err("Required CCAPI permissions rejected".into()) };
    if (resp != "auth") { return Err("Required CCAPI permission rejected".into()) }
    eprintln!("Done");

    // Get plot size.
    eprint!("Getting plot size... ");
    let Ok(_) = sock.send(ws::Message::Text("size".to_string())) else { return Err("Failed to get plot size".into()) };
    let Ok(ws::Message::Text(size)) = sock.read() else { return Err("Failed to get plot size".into()) };
    let _size = match (size.as_str()) {
        "BASIC"   => 50,
        "LARGE"   => 100,
        "MASSIVE" => 300,
        "MEGA"    => 1000,
        _ => { return Err("Failed to get plot size".into()) }
    };
    eprintln!("Done");

    // Clear codespace.
    eprint!("Clearing the codespace... ");
    let Ok(_) = sock.send(ws::Message::Text("clear".to_string())) else { return Err("Failed to clear plot".into()) };
    eprintln!("Done");

    // Place templaces.
    eprint!("Queueing templates... ");
    for template in templates {
        // TODO: split template to plot size.
        let template = template.to_b64();
        //println!("{}", template);
        let Ok(_) = sock.send(ws::Message::Text(format!("place {}", template))) else { return Err("Failed to queue template".into()) };
    }
    eprintln!("Done");
    eprint!("Placing templates... ");
    let Ok(_) = sock.send(ws::Message::Text("place go".to_string())) else { return Err("Failed to place templates".into()) };
    let Ok(ws::Message::Text(resp)) = sock.read() else { return Err("Failed to place templates".into()) };
    if (resp != "place done") { return Err("Failed to place templates".into()) }
    eprintln!("Done");

    Ok(())
}
