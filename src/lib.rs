pub mod error;
pub mod lexer;
pub mod parser;

pub fn parse(input: String) {
    let lexer = lexer::Lexer::builder().lexer_input(input).build();
    let parser = parser::Parser::builder().build();
    println!("{:#?}", lexer);
    println!("{:#?}", parser);
}
