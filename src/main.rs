mod ast;
mod codegen;
mod error;
mod lex;
mod parse;
mod token;

use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use codegen::CodeGenerator;
use error::Result;
use lex::Lexer;
use parse::Parser;

use clap::Parser as ClapParser;

fn main() {
    let args = Args::parse();

    if let Some(wasm) = compile_input(&args.infile) {
        write_output(&args.outfile, wasm);
    }
}

fn compile_input(maybe_path: &Option<PathBuf>) -> Option<Vec<u8>> {
    let result = if let Some(path) = maybe_path {
        match File::open(path) {
            Ok(file) => perform_compilation_steps(file, path.to_string_lossy().into_owned()),
            Err(e) => {
                eprintln!("Could not read input file {}: {}", path.display(), e);
                return None;
            }
        }
    } else {
        perform_compilation_steps(io::stdin(), "stdin".to_string())
    };

    if let Err(e) = &result {
        eprintln!("{e}");
    }

    result.ok()
}

fn write_output(maybe_path: &Option<PathBuf>, wasm: Vec<u8>) {
    if let Some(path) = maybe_path {
        let result = File::create(path).and_then(|mut f| f.write_all(&wasm));

        if let Err(e) = result {
            eprintln!("Could not write output file {}: {}", path.display(), e);
        }
    } else {
        println!("{:?}", wasm);
    }
}

fn perform_compilation_steps(input: impl Read, name: String) -> Result<Vec<u8>> {
    let lexer = Lexer::new(input, name);
    let parser = Parser::new(lexer);
    let generator = CodeGenerator::new(parser);
    generator.generate_wasm()
    // TODO: Optimise with wasm-opt?
}

#[derive(ClapParser)]
struct Args {
    /// Input Kobe source file path
    infile: Option<PathBuf>,
    /// Output WASM file path
    outfile: Option<PathBuf>,
}
