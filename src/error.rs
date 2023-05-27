use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct Error {
    line_number: usize,
    char_number: usize,
    line: String,
    file_path: PathBuf,
    kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Lexical => writeln!(f, "Lexical error: ...")?,
            ErrorKind::Syntax => writeln!(f, "Syntax error: ...")?,
        }
        writeln!(
            f,
            "> {}:{}:{}",
            self.file_path.display(),
            self.line_number,
            self.char_number
        )?;
        writeln!(f, "| {}", self.line)?;
        writeln!(f, "| {}^", " ".repeat(self.char_number))
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Lexical,
    Syntax,
}

pub type Result<T> = std::result::Result<T, Error>;
