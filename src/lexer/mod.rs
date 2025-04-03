// imports
use bon;

// modules
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    LET,
    IDENTIFIER,
    EQUAL,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    column: usize,
    file_path: String,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    curr_char: char,
}

#[bon::bon]
impl Lexer {
    #[builder]
    pub fn new(lexer_input: String) -> Lexer {
        return Lexer {
            input: lexer_input,
            pos: 0,
            read_pos: 0,
            curr_char: 'a',
        };
    }
}
