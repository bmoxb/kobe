#[derive(Debug)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub line_number: usize,
    pub char_number: usize,
}

#[derive(Debug)]
pub enum TokenType {
    KeywordFn,
}
