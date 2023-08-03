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

fn main() -> Result<()>  {
    let args: Vec<String> = env::args().collect();

    let inputs;
    if args.len() > 1 {
        let md = metadata(&args[1])?;
        if md.is_file() {
            inputs = read_from_file(&args[1])?;
        } else {
            assert!(md.is_dir());
            panic!("recursive folder reading not implemented!")
        }
    } else {
        inputs = read_from_stdin()?;
    }



    for input in inputs.iter() {
        //println!("{:?}", &input);

        let lexer = Lexer::new(input);

        // Iterate through all tokens
        for token in lexer.iter(0) {
            match token {
                Ok(tok) => {
                    // Process each token here
                    println!("Token: {:?}", tok);
                }
                Err(err) => {
                    eprintln!("Error while parsing token: {:?}", err);
                }
            }
        }


            match wat::parse_str(input) {
                Ok(_wasm) => {
                    println!("Valid WAT");
                    exit(exitcode::OK);
                }
                Err(error) => {
                    println!("Invalid WAT: {}", &error);
                    exit(exitcode::DATAERR);

                }
            }

    */
        }
    Ok(())
}
