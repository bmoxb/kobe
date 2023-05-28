mod ast;
mod codegen;
mod error;
mod lex;
mod parse;
mod token;

use std::{
    fs::File,
    io::{self, Cursor, Read, Seek, Write},
    path::PathBuf,
};

use clap::Parser;

fn main() {
    let args = Args::parse();

    let wasm = if let Some(infile) = args.infile {
        let f = File::open(infile).unwrap();
        compile(f)
    } else {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        compile(Cursor::new(line))
    };

    if let Some(outfile) = args.outfile {
        let mut f = File::create(outfile).unwrap();
        f.write_all(&wasm).unwrap();
    } else {
        println!("{:?}", wasm);
    }
}

fn compile(input: impl Read + Seek) -> Vec<u8> {
    let lexer = lex::Lexer::new(input);
    let parser = parse::Parser::new(lexer);
    let generator = codegen::CodeGenerator::new(parser);
    generator.generate_wasm().unwrap()
    // TODO: Optimise with wasm-opt?
}

#[derive(Parser)]
struct Args {
    infile: Option<PathBuf>,
    outfile: Option<PathBuf>,
}
