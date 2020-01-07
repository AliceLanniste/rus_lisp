use std::fmt;

#[derive(Debug,PartialEq)]
pub enum SExp {
    Bool(String),
    Number(i64),
    EOF,
}


impl fmt::Display for SExp {
   
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            SExp::Bool(b)    => b.to_string(),
            SExp::Number(n)  => n.to_string(),
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

  
    fn peek(&self) -> Option<char> {
       
        self.input.chars().next()
         
    }

    fn nth(&self, offset:usize) -> Option<char> {
        self.input.chars().nth(offset)
    }

    
    pub fn read(&mut self) -> SExp {
        
      
        while let Some(c) = self.peek() {
            if c.is_numeric() {    
                let mut n = c;
                let cstart = self.pos; 
                while n.is_numeric() {
                    self.pos += 1;
                   
                    n = match self.nth(self.pos) {
                       Some(v)  => v,
                       None   => break,
                   };
                   
                }
                let value = self.input[cstart..self.pos].parse().unwrap();
               
            return SExp::Number(value);
            }
            else if c =='#' {
              let cstart = self.pos;
              self.pos += 1;
              let value = &self.input[cstart..=self.pos]; 
              if value =="#t" || value == "#f" {
                   return SExp::Bool(String::from(value));
              } else {
                 panic!("line {}:{} unexpected char: {}", self.linenumber, self.pos, value);
              } 

            
            }

            else {

            }

            
        }
        
      return SExp::EOF;

    }  
}



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_number() {

        assert_eq!(SExp::Number(0), Lexer::new("0").read());

        assert_eq!(SExp::Number(12345),Lexer::new("12345").read());
    
    }
    #[test]
    fn test_bool()  {
        assert_eq!(SExp::Bool(String::from("#t")), Lexer::new("#t").read() );
        assert_eq!(SExp::Bool(String::from("#f")), Lexer::new("#f").read() );
    }
}
