use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub line_number: usize,
    pub char_number: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({:?}) line: {} character: {}",
            self.lexeme, self.tok_type, self.line_number, self.char_number
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Assign,
    Colon,
    Comma,
    OpenBracket,
    CloseBracket,
    OpenSquare,
    CloseSquare,
    Plus,
    Minus,
    Times,
    Divide,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equivalent,
    NotEquivalent,
    Arrow,
    Not,
    IntLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    Identifier,
    DoKeyword,
    EndKeyword,
    ForKeyword,
    WhileKeyword,
    IfKeyword,
    ThenKeyword,
    ElseKeyword,
    FnKeyword,
    ReturnKeyword,
    AndKeyword,
    OrKeyword,
}
