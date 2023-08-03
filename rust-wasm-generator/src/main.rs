use wasm_mutate::WasmMutate;
use anyhow::Result;
use std::env;
use std::fs::metadata;
use std::io::Read;
use std::fs;
use exitcode;
use std::process::exit;
use wast::Wat;
use wast::parser::{self, ParseBuffer};
use wast::lexer::Lexer;
use wast::lexer::Token;
use colored::Colorize;

fn read_from_file(file_path: &str) -> Result<Vec<String>> {
    dbg!("Reading content from file {}", String::from(file_path));
    let file_content = fs::read_to_string(file_path)?;
    Ok(vec![file_content])
}

fn read_from_stdin() -> Result<Vec<String>> {
    dbg!("Reading content from STDIN");

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(vec!(buffer))
}

fn load_wat(args: Vec<String>) -> Result<Vec<String>> {
    if args.len() > 1 {
        let md = metadata(&args[1])?;
        if md.is_file() {
            read_from_file(&args[1])
        } else {
            assert!(md.is_dir());
            panic!("recursive folder reading not implemented!")
        }
    } else {
        read_from_stdin()
    }
}

fn main() -> Result<()>  {

    let inputs = load_wat(env::args().collect())?;

    for input in inputs.iter() {
        println!("{}", "----------".white());
        println!("{}", "Before Transformation".bold());
        println!("{}", "----------".white());
        println!("{}", &input);

        match wat::parse_str(input) {
            Ok(input_wasm) => {
                let mut mutate = WasmMutate::default();
                mutate
                    // Set the RNG seed.
                    .seed(42)
                    // Allow mutations that change the semantics of the Wasm module.
                    .preserve_semantics(false)
                    // Use at most this much "fuel" when trying to mutate the Wasm module before
                    // giving up.
                    .fuel(1_000);

                for mutated_wasm in mutate.run(&input_wasm)? {
                    let wat = wasmprinter::print_bytes(&mutated_wasm?)?;


                    println!("{}", "----------".white());
                    println!("{}", "After Transformation".bold());
                    println!("{}", "----------".white());
                    println!("{}", wat);

                    exit(exitcode::OK);
                }

                println!("waht?!");
                exit(exitcode::SOFTWARE);

                //println!("Valid WAT");
            }
            Err(error) => {
                println!("Invalid WAT: {}", &error);
                exit(exitcode::DATAERR);
            }
        }
    }
    Ok(())
}
