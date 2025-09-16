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

}


