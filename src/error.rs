use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub line_number: usize,
    pub char_number: usize,
    pub line: String,
    pub input_source_name: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.kind)?;
        writeln!(
            f,
            "> {}:{}:{}",
            self.input_source_name, self.line_number, self.char_number
        )?;
        writeln!(f, "| {}", self.line)?;
        writeln!(f, "|{}^", " ".repeat(self.char_number))
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Lexical(LexicalErrorKind),
    Syntax,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Lexical(k) => write!(f, "Lexical error: {k}."),
            ErrorKind::Syntax => write!(f, "Syntax error: ..."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorKind {
    UnexpectedCharacter,
    InvalidFloatLiteral,
    InvalidCharLiteral,
    InvalidStringLiteral,
    InvalidEscapeCode,
}

impl fmt::Display for LexicalErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalErrorKind::UnexpectedCharacter => write!(f, "unexpected character in input"),
            LexicalErrorKind::InvalidFloatLiteral => write!(f, "invalid floating-point literal"),
            LexicalErrorKind::InvalidCharLiteral => write!(f, "invalid character literal"),
            LexicalErrorKind::InvalidStringLiteral => write!(f, "invalid string literal"),
            LexicalErrorKind::InvalidEscapeCode => write!(f, "invalid escape code"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
