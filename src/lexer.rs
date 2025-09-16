mod Token;
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize, // changed from i32 to usize because that is required for data fields having properties such as length
    pub ch: char

}


impl Lexer {
    fn new(input: String) -> Self {
         let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None
         };

         l.read_char();
         return l;
    }


    fn read_char(&mut self) {
       if self.read_position > self.input.len(){
        self.ch = None;
       } else {
           self.ch = self.input[self.read_position..].chars().next()

       }

       self.position = self.read_position;
       self.read_position = self.read_position + 1;


    }

    fn next_token(&mut self) -> Token{
        let tok: Token::token_type;

        match self.ch {

             Some('=') => tok = new_token(token_type.ASSIGN, self.ch),
             Some('(') => tok = new_token(token_type.LPAREN, self.ch),
             Some(';') => tok = new_token(token_type.SEMICOLON, self.ch),
             Some(')')=> tok = new_token(token_type.RPAREN, self.ch),
             Some(',')=> tok = new_token(token_type.COMMA, self.ch),
             Some('+')=> tok = new_token(token_type.PLUS, self.ch),
             Some('{')=> tok = new_token(token_type.LBRACE, self.ch),
             Some('}') =>  tok = new_token(token_type.RBRACE, self.ch),
             None => Token {
               token_type: token_type.EOF,
               literal: "".to_string(), 
             },
             Some(ch) => Token {
                token_type: token_type.ILLEGAL,
                literal: ch.to_string(),

             }
            
        }

    }


    fn new_token(token_type: Token::token_type, ch: char) -> Token {
        return Token{token_type: token_type, literal: ch.to_string()}
    }

}


