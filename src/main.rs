
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

pub struct Lexer {
    pub input: String,
    pub position: i32,
    pub read_position: i32,
    pub ch: char

}


fn main() {
    println!("Hello, world!");


}