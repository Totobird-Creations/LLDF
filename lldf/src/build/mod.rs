pub mod parse;
use codegen::CodeblockBlock;
use llvm_ir::Constant;
use llvm_ir::Type;
use parse::{ handle_instr, handle_constant };
pub mod codegen;
use codegen::{ CodeLine, Codeblock, Value };


use std::fs;
use std::mem;
use std::path::Path;
use std::process::{ self, Command };
use std::collections::HashMap;
use std::error::Error;

use inflector::Inflector;
use llvm_ir::{ Module, Name };
use llvm_ir::module::Linkage;

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
        Err(err) => { println!("error: {}", err); process::exit(1) },
        Ok(module) => module
    };
    //let endianness = module.data_layout.endianness;

    if let Err(e) = build_modules(&vec![module]) {
        println!("error: {}", e);
        process::exit(1);
    }

//    for decl in &module.func_declarations {
//        println!("{}", decl.name);
//    }
//
//    for function in &module.functions {
//        // Get a function.
//        let cfg = decomp::cfg::ControlFlowGraph::new(&function);
//        println!("FUNCTION {}", function.name);
//        let Some(cfa) = decomp::cfa::CFAPrim::find_all(cfg) else { println!("error: Failed to analyse control flow of function {:?}", function.name); process::exit(1) };
//        let Some(cfr) = decomp::cfr::CFRGroups::new(&cfa)   else { println!("error: Failed to recover control flow of function {:?}", function.name); process::exit(1) };
//
//        //for group in &cfr.groups { match (group) {
//        //    decomp::cfr::CFRGroup::Block(block) => {
//        //        let block = function.basic_blocks.iter().find(|bb| bb.name == *block).unwrap();
//        //        for instr in &block.instrs {
//        //            println!("\n{:?}", instr);
//        //        }
//        //    },
//        //    _ => { todo!() }
//        //} }
//    }
}



pub struct BuildContext {
    declarations : HashMap<Name, BCDeclaration>
}
impl BuildContext {
    pub fn function(&self) -> FunctionBuildContext { FunctionBuildContext {
        ctx            : self,
        locals         : HashMap::new(),
        lines          : HashMap::new(),
        current_line   : CodeLine::new(),
        next_temporary : 0
    } }
}

#[derive(Debug)]
pub enum BCDeclaration {
    Action {
        codeblock : String,
        kind      : String,
        tags      : Vec<String>
    },
    Gamevalue {
        kind   : String,
        target : String
    },
    Function,
    Constant {
        value : Value
    },
    Static,
    NoOp
}

pub struct FunctionBuildContext<'l> {
    ctx            : &'l BuildContext,
    locals         : HashMap<String, Value>,
    lines          : HashMap<Name, CodeLine>,
    current_line   : CodeLine,
    next_temporary : usize
}
impl FunctionBuildContext<'_> {
    pub fn create_temp_var(&mut self) -> Value {
        let temp_var = Value::line_variable(format!("local.temp.{}", self.next_temporary));
        self.next_temporary += 1;
        temp_var
    }
}


pub fn build_modules(modules : &Vec<Module>) -> Result<(), Box<dyn Error>> {
    for module in modules {
        // Sanity checks
        if let Some(target_triple) = &module.target_triple {
            if (target_triple != "wasm32-unknown-unknown") {
                println!("warning: Target triple of module {:?} is not {:?}. Got {:?}", module.name, "wasm32-unknown-unknown", target_triple);
            }
        } else {
            println!("warning: Target triple of module {:?} is not known.", module.name);
        }

        let mut ctx = BuildContext {
            declarations : HashMap::new()
        };

        // Collect all globals.
        for global in &module.global_vars {
            let mut initialiser = ctx.function();
            let init = match (&global.initializer) {
                None        => None,
                Some (init) => Some(handle_constant(&mut initialiser, init)?)
            };
            if (global.is_constant) {
                let Some(init) = init else {
                    return Err(format!("Global constant {:?} has no initialiser", global.name).into());
                };
                if (initialiser.current_line.blocks.len() > 0) {
                    // Check for string.
                    if let Codeblock::Block(CodeblockBlock { block, action, params, .. }) = &initialiser.current_line.blocks[0] {
                        if (block == "set_var" && action.clone().is_some_and(|a| a == "CreateList") && params.iter().skip(1).all(|p| matches!(p, Value::Int(_)))) {
                            let bytes = params.iter().skip(1).map(|p| { let Value::Int(p) = p else { unreachable!() }; *p as u8 }).collect::<Vec<_>>();
                            if let Ok(string) = String::from_utf8(bytes) {
                                ctx.declarations.insert(global.name.clone(), BCDeclaration::Constant { value : Value::String(string) });
                            }
                        } else {
                            todo!();
                        }
                    } else {
                        todo!();
                    }
                } else {
                    ctx.declarations.insert(global.name.clone(), BCDeclaration::Constant { value : init });
                }
            } else {
                todo!();
            }
        }

        // Collect all externally linked functions.
        for func in &module.func_declarations { if (func.linkage == Linkage::External) {
            if (func.name == "llvm.lifetime.start.p0" || func.name == "llvm.lifetime.end.p0") {
                ctx.declarations.insert(Name::Name(Box::new(func.name.clone())), BCDeclaration::NoOp);
                continue;
            }

            if (func.name.starts_with("DF_ACTION_")) {
                let mut parts = func.name.split("_").skip(2);
                let (Some(codeblock), Some(kind)) = (parts.next(), parts.next()) else {
                    return Err(format!("Externally linked function {:?} is missing required details", func.name).into());
                };
                let mut codeblock = codeblock.to_snake_case();
                codeblock = match (codeblock.as_str()) {
                    "select_object" => String::from("select_obj"),
                    "set_variable"  => String::from("set_var"),
                    _ => { codeblock }
                };
                ctx.declarations.insert(Name::Name(Box::new(func.name.clone())), BCDeclaration::Action {
                    codeblock,
                    kind      : kind.to_string(),
                    tags      : parts.map(|tag| fix_title_case(tag.to_title_case())).collect::<Vec<_>>()
                });
                continue;
            }

            if (func.name.starts_with("DF_GAMEVALUE_")) {
                let mut parts = func.name.split("_").skip(2);
                let (Some(kind), Some(target)) = (parts.next(), parts.next()) else {
                    return Err(format!("Externally linked function {:?} is missing required details", func.name).into());
                };
                ctx.declarations.insert(Name::Name(Box::new(func.name.clone())), BCDeclaration::Gamevalue {
                    kind   : fix_title_case(kind.to_title_case()),
                    target : fix_title_case(target.to_title_case())
                });
                continue;
            }

            return Err(format!("Unknown externally linked function {:?}", func.name).into());
        } }

        // Collect all user defined functions.
        for func in &module.functions {
            ctx.declarations.insert(Name::Name(Box::new(func.name.clone())), BCDeclaration::Function);
        }

        // Parse the function.
        for function in &module.functions {
            let mut fctx = ctx.function();
            
            for block in &function.basic_blocks {
                for instr in &block.instrs {
                    handle_instr(&mut fctx, instr)?;
                }

                { // TODO: Remove, this is a test.
                    fctx.current_line.blocks.insert(0, Codeblock::function(format!("{}:{}", function.name, block.name), vec![], false));
                    let data = "minecraft:ender_chest{PublicBukkitValues:{\"hypercube:codetemplatedata\":'{\"author\":\"TotobirdCreation\",\"name\":\"&b&lFunction &3» &bUnnamed\",\"version\":1,\"code\":\"[INSERT]\"}'},display:{Name:'{\"text\":\"\",\"extra\":[{\"text\":\"Function \",\"obfuscated\":false,\"italic\":false,\"underlined\":false,\"strikethrough\":false,\"color\":\"aqua\",\"bold\":true},{\"text\":\"» \",\"italic\":false,\"color\":\"dark_aqua\",\"bold\":false},{\"text\":\"Unnamed\",\"italic\":false,\"color\":\"aqua\"}]}'}}";
                    println!("\n{}", data.replace("[INSERT]", &fctx.current_line.to_b64()));
                }

                fctx.lines.insert(block.name.clone(), mem::replace(&mut fctx.current_line, CodeLine::new()));
            }
        }

    }
    Ok(())
}


fn fix_title_case<S : AsRef<str>>(string : S) -> String {
    let string = string.as_ref();
    let mut out = string.split(" ").intersperse(" ")
        .map_windows::<_, _, 3>(|[a, b, c]| if (*b == " " && a.chars().last().unwrap().is_uppercase() && c.chars().next().unwrap().is_uppercase()) { None } else { Some(*b) })
        .filter_map(|b| b)
        .collect::<String>();
    let mut split = string.split(" ");
    if let Some(part) = split.next() {
        out.insert_str(0, part);
    }
    if let Some(part) = split.last() {
        out.push_str(part);
    }
    out
}
