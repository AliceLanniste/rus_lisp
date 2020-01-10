use std::fmt;

#[derive(Debug,PartialEq)]
pub enum SExp {
    Bool(String),
    Number(i64),
    WhiteSpace,
    LEFTPAREN,
    RIGHTPARE,
    // Symbol,
    //Comment
    EOF,
}


impl fmt::Display for SExp {
   
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            SExp::Bool(b)    => b.to_string(),
            SExp::Number(n)  => n.to_string(),
            SExp::WhiteSpace  =>format!("WhiteSpace"),
            SExp::LEFTPAREN  => "(".to_string(),
            SExp::RIGHTPARE  => ")".to_string(),
            SExp::EOF    => "EOF".to_string(),
         };

         write!(f,"{}", description)
    
        }

}


#[derive(Debug)]
pub enum SExpError {
    ParseNumberError,
    ParseError,
    BadSyntax,
}


#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    linenumber: usize,
    pos: usize,
}


impl<'a> Lexer<'a> {
    pub fn new(text:&'a str) -> Self {

        Lexer{
            input:text,
            linenumber: 1,
            pos:0,
        }

    }

  
    
    
    fn peek(&mut self, offset:usize) -> Option<char> {
        let pos  =self.pos + offset;
        self.input.chars().nth(pos)
       
    }

    pub fn read(&mut self) ->SExp {
        
           while let Some(c) = self.peek(0) {
                if c.is_numeric() || c == '-'  {
                    
                    return self.lex_number();
                } 
                else if c == '#' {
                    return self.lex_bool();
                } else {
                     match c {
                         ' ' => {self.pos +=1; 
                         println!("whitespace");
                               continue
                               },

                         '\n' => {unreachable!()}
                         _  => {unreachable!()}      
                     }   

                }
           }
           
            
               
            return SExp::EOF;
    }

    fn lex_number(&mut self) -> SExp {
        let start = self.pos;
        loop {
            match self.peek(0) {
                Some(c)  if c.is_numeric() || c =='-' => {  
                    self.pos +=1;  
                },
                    
                _ => break,
            }
        }
        SExp::Number(self.input[start..self.pos].parse().unwrap())
        
    }

    fn lex_bool(&mut self) -> SExp {
        let start = self.pos;
        let value = &self.input[start..=self.pos+1]; 
        if value =="#t" || value == "#f" {
            return SExp::Bool(String::from(value));
        } else {
            panic!("line {}:{} unexpected char: {}", self.linenumber, self.pos, value);
        } 
        
    }

    // fn lex_whitespace(&mut self) -> SExp {
    //     while let ' ' = self.peek2(0) {
    //         self.pos +=1;
    //     }
        
    //     SExp::WhiteSpace
    // }

    fn is_valid_for_identifier(&self, c:char) -> bool {
        match c {
            '!' |'$'|'%' | 'a'..='z'|'A'..='Z'|'0'..='9' => true,
            _ => false,
        }
        
    }

}



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_number() {

        assert_eq!(SExp::Number(0), Lexer::new("0\n").read());

        assert_eq!(SExp::Number(12345),Lexer::new("12345\n").read());
    
    }
    #[test]
    fn test_bool()  {
        assert_eq!(SExp::Bool(String::from("#t")), Lexer::new("#t").read() );
        assert_eq!(SExp::Bool(String::from("#f")), Lexer::new("#f").read() );
    }
}