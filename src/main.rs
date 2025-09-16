
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    ADD,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    
}

pub struct Token {
    pub token_type : TokenType,
    pub literal : String
}




fn main() {
    println!("Hello, world!");


}