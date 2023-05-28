use crate::{error::Result, parse::Parser};

pub struct CodeGenerator {
    nodes: Parser,
}

impl CodeGenerator {
    pub fn new(nodes: Parser) -> Self {
        CodeGenerator { nodes }
    }

    pub fn generate_wasm(self) -> Result<Vec<u8>> {
        for _node in self.nodes {
            // ...
        }
        unimplemented!()
    }
}
