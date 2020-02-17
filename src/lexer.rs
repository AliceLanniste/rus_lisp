use std::fmt;


// TODO
// 返回值应改为Result类型，提供更好的错误提示
//增加symbol，注释，转义字符、float、whitespace支持
//lex_number需要改进
//增加repl



#[derive(Debug,PartialEq)]
pub enum SExp {
    Bool(String),
    Number(i64),
    WhiteSpace,
    LParen,
    RParen,
    Symbol(String),
    Comment,
    EOF,
}


impl fmt::Display for SExp {
   
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            SExp::Bool(b)    => b.to_string(),
            SExp::Number(n)  => n.to_string(),
            SExp::WhiteSpace  =>format!("WhiteSpace"),
            SExp::Comment   => format!("Comment"),
            SExp::LParen  => "(".to_string(),
            SExp::RParen  => ")".to_string(),
            SExp::Symbol(s)  => s.to_string(),
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
    col: usize,
    pos: usize,
}


impl<'a> Lexer<'a> {
    pub fn new(text:&'a str) -> Self {

        Lexer{
            input:text,
            linenumber: 1,
            col: 0,
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
                }
                else if self.is_valid_for_identifier(c){
                    return self.lex_symbol();   
                 } 
                else {
                     match c {
                         ' ' => {self.pos +=1; 
                                self.col += 1;
                                continue
                               },

                         '\n' |'\r'=> {self.linenumber +=1;
                                 self.pos += 1;
                                 self.col =0;
                                 continue },
                          '(' => {self.pos +=1;
                                 self.col +=1;
                                 return SExp::LParen},
                          ')' => {self.pos +=1;
                                 self.col +=1;
                                 return SExp::RParen},
                          ';' => {
                              self.pos +=1;
                              self.col =0;
                              self.linenumber +=1;
                              return SExp::Comment
                          },      
                         _  =>panic!("line {}:{} unexpected char: {}", self.linenumber, self.col, c),
                     }   

                }
           }
               
            return SExp::EOF;
    }

    //可以改成类似"12324"然后在转换类型
    fn lex_number(&mut self) -> SExp {
        let start = self.pos;
        loop {
            match self.peek(0) {
                Some(c)  if c.is_numeric() || c =='-' => {  
                    self.pos += 1;  
                    self.col += 1;
                },
                    
                _ => break,
            }
        }
        SExp::Number(self.input[start..self.col].parse().unwrap())
        
    }

    fn lex_bool(&mut self) -> SExp {
        let start = self.pos;
        self.pos += 1;
        self.col += 1;
        let value = &self.input[start..=self.col]; 
       
        if value =="#t" || value == "#f" {
            return SExp::Bool(String::from(value));
        } else {
            panic!("line {}:{} unexpected char: {}", self.linenumber, self.col, value);
        } 
        
    }

     fn lex_symbol(&mut self) -> SExp {
        let start = self.pos;
        loop {
            match self.peek(0) {
                Some(c)  if self.is_valid_for_identifier(c) =>{  
                    self.pos += 1;  
                    self.col += 1;
                },
                    
                _ => break,
            }
        }
        SExp::Symbol(self.input[start..self.col].to_string())
        
    }

    fn is_valid_for_identifier(&self, c:char) -> bool {
        match c {
            '!' |'$'|'%' | 'a'..='z'|'A'..='Z'|'0'..='9' => true,
            _ => false,
        }
        
    }

}


impl<'a> Iterator for Lexer<'a> {
    type Item = SExp;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read() {
            SExp::EOF  => None,
            t       => Some(t),   
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
