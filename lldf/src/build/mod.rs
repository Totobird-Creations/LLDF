pub mod codegen;
pub mod parse;


use std::fs;
use std::path::Path;
use std::process::{ self, Command };
use std::error::Error;
use std::collections::HashMap;

use codegen::{CodeLine, CodeValue, Codeblock, ParameterType, VariableScope};
use llvm_ir::Module;

use parse::{ ParsedModule, name_to_local };
use serde::Deserialize as Deser;
use toml;
use tungstenite as ws;
use const_str::concat;


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
    let mut templates = HashMap::new();

    let init_template = "DF_INIT".to_string();

    for module in modules {
        if let Some(function) = module.init_function {
            if (! function.line.blocks.is_empty()) {
                templates.entry(init_template.clone()).or_insert_with(|| (vec![ ], CodeLine::new())).1.blocks.extend(function.line.blocks);
            }
        }
        for (name, function) in module.functions {
            if (! function.line.blocks.is_empty()) {
                let params = function.function.map(|function| function.parameters.iter().map(
                    |param| CodeValue::Parameter {
                        name        : name_to_local(&param.name),
                        typ         : ParameterType::Variable,
                        plural      : false,
                        optional    : false,
                        description : Some(format!("Type: {}", param.ty)),
                        note        : Some(format!("{}", param.name))
                    }
                ).collect()).unwrap_or_else(|| vec![ ]);
                templates.entry(name).or_insert_with(|| (params, CodeLine::new())).1.blocks.extend(function.line.blocks);
            }
        }
    }

    // Handle init template.
    if let Some(mut template) = templates.remove(&init_template) {
        let init_var = CodeValue::Variable {
            name  : concat!(crate::MODULE_NAME, ".init").to_string(),
            scope : VariableScope::Unsaved
        };
        let one      = CodeValue::Number("1".to_string());
        let params   = vec![ init_var, one ];
        template.1.blocks.insert(0, Codeblock::action("set_var", "=", params.clone(), vec![]));
        template.1.blocks.insert(0, Codeblock::OPEN_COND_BRACKET);
        template.1.blocks.insert(0, Codeblock::action("if_var", "=", params, vec![])); // TOOD: NOT and tags
        template.1.blocks.push(Codeblock::CLOSE_COND_BRACKET);
        templates.entry(String::from("DF_EVENT__Event_Join")).or_insert_with(|| (vec![ ], CodeLine::new())).1.blocks.splice(0..0, template.1.blocks);
    }

    for (name, (params, ref mut template)) in &mut templates {
        let mut parts = name.split("__");
        let mut head_block = Codeblock::function(name, params.clone(), true);
        match (parts.next()) {

            Some("DF_EVENT") => {
                // TODO: LS-CANCEL
                if let (Some(trigger), None) = (parts.next(), parts.next()) {
                    let mut trigger_parts = trigger.split("_");
                    if let (Some(codeblock), Some(action), None) = (trigger_parts.next(), trigger_parts.next(), trigger_parts.next()) {
                        let codeblock = parse::linked_name_to_codeblock (codeblock );
                        let action    = parse::linked_name_to_action    (action    );
                        head_block = Codeblock::event(codeblock, action);
                    }
                }
            },

            _ => { }
        };
        template.blocks.insert(0, head_block);
        codegen::opt::optimise(template);
    }

    Ok(templates.into_values().map(|(_, template)| template).collect())
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
        let Ok(_) = sock.send(ws::Message::Text(format!("place {}", template.to_b64()))) else { return Err("Failed to queue template".into()) };
    }
    eprintln!("Done");
    eprint!("Placing templates... ");
    let Ok(_) = sock.send(ws::Message::Text("place go".to_string())) else { return Err("Failed to place templates".into()) };
    let Ok(ws::Message::Text(resp)) = sock.read() else { return Err("Failed to place templates".into()) };
    if (resp != "place done") { return Err("Failed to place templates".into()) }
    eprintln!("Done");

    Ok(())
}
