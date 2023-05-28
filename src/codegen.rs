use std::io::{Read, Seek};

use crate::{error::Result, parse::Parser};

pub struct CodeGenerator<R> {
    nodes: Parser<R>,
}

impl<R: Read + Seek> CodeGenerator<R> {
    pub fn new(nodes: Parser<R>) -> Self {
        CodeGenerator { nodes }
    }

    pub fn generate_wasm(self) -> Result<Vec<u8>> {
        for _node in self.nodes {
            // ...
        }
        unimplemented!()
    }
}
