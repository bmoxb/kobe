mod ast;
mod codegen;
mod error;
mod lex;
mod parse;
mod token;

use std::{fs::File, io::Write, path::PathBuf};

use clap::Parser;

fn main() {
    let args = Args::parse();

    let infile = Box::new(File::open(args.infile).unwrap());

    let lexer = lex::Lexer::new(infile);
    let parser = parse::Parser::new(lexer);
    let generator = codegen::CodeGenerator::new(parser);
    let wasm = generator.generate_wasm().unwrap();
    // TODO: Optimise with wasm-opt?

    let mut outfile = File::create(args.outfile).unwrap();
    outfile.write_all(&wasm).unwrap();
}

#[derive(Parser)]
struct Args {
    infile: PathBuf,
    outfile: PathBuf,
}
