
use std::str::Chars;


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

#[derive(Debug)]
pub struct Span {
    beginLineNumber: usize, 
    endLineNumber: usize, 
    beginIndex: usize, 
    endIndex: usize
}


#[derive(Debug)]
pub  struct Token {
    tokentype: SExp,
    metaData: Span
}





#[derive(Debug)]
pub enum SExpError {
    ParseNumberError,
    ParseError,
    BadSyntax,
}


#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
    prev: char,
    line: usize,
    col: usize,
    charIndex: usize,
    currentTokenIndex: usize,
}

impl <'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Lexer{
            chars:text.chars(),
            prev:'\0',
            line :1,
            col: 1,
            charIndex:0,
            currentTokenIndex: 0
        }
    }

    pub fn current_char(&self) -> Option<char>{
        self.nth(0)
    }

    pub fn peek_char(&self) ->Option<char> {
        self.nth(1)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn nth(&self, offset:usize) -> Option<char> {
        self.chars().nth(offset)
       
    }

    fn chars(&self) ->Chars<'a>{
        self.chars.clone()
    }


    pub fn read(&self)  {
        while let Some(c) = self.current_char() {
             if c == '-' {
                 self.Negativeoridentifer();
             }
        }
      
    }

    fn Negativeoridentifer(&self){
      if self.peek_char().unwrap().is_digit(9){
          self.lex_number()
      } else {
          self.lex_symbol()
      }
        
    }

    fn lex_number(&self)  {
       
       
    }

    fn lex_symbol(&self)  {
         let line = self.line;
        let col = self.col;
        let value = "";
        let token = Token {
            tokentype: SExp::Number(0),
            metaData: Span {
                beginLineNumber:line,
                endLineNumber:line,
                beginIndex:col,
                endIndex:self.col,
            }
        };
        
    }


    fn is_valid_for_identifier(&self, c:char) -> bool {
        match c {
            '!' |'$'|'%' | 'a'..='z'|'A'..='Z'|'0'..='9'|'+'|'-'|'*'|'/' => true,
            _ => false,
        }
        
    }
}

// impl<'a> Lexer<'a> {
//     pub fn new(text:&'a str) -> Self {

//         Lexer{
//             input:text,
//             linenumber: 1,
//             col: 0,
//             pos:0,
//         }

//     }
    
//     pub fn current_char(&self) -> Option<char>{
//         self.nth(0)
//     }

//     pub fn peek(&self) ->Option<char> {
//         self.nth(1)
//     }

//     fn nth(&self, offset:usize) -> Option<char> {
//         self.input.chars().nth(offset)
       
//     }


    
//     pub fn read(&mut self) ->SExp {
        
//            while let Some(c) = self.current_char() {
//                 if  c == '-'  {
                    
//                     return self.Negativeoridentifer();
//                 } 
//                 else if c == '#' {
//                     return self.lex_bool();
//                 }
//                 else if self.is_valid_for_identifier(c){
//                     return self.lex_symbol();   
//                  } 
//                 else {
//                      match c {
//                          ' ' => {self.pos +=1; 
//                                 self.col += 1;
//                                 continue
//                                },

//                          '\n' |'\r'=> {self.linenumber +=1;
//                                  self.pos += 1;
//                                  self.col =0;
//                                  continue },
//                           '(' => {self.pos +=1;
//                                  self.col +=1;
//                                  return SExp::LParen},
//                           ')' => {self.pos +=1;
//                                  self.col +=1;
//                                  return SExp::RParen},
//                           ';' => {
//                               self.pos +=1;
//                               self.col =0;
//                               self.linenumber +=1;
//                               return SExp::Comment
//                           },      
//                          _  =>panic!("line {}:{} unexpected char: {}", self.linenumber, self.col, c),
//                      }   

//                 }
//            }
               
//             return SExp::EOF;
//     }

//     fn Negativeoridentifer(&mut self) -> SExp{
//       if self.peek().unwrap().is_digit(9){
//           self.lex_number()
//       } else {
//           self.lex_symbol()
//       }
        
//     }

//     //可以改成类似"12324"然后在转换类型
//     fn lex_number(&mut self) -> SExp {
//         let start = self.pos;
//         loop {
//             match self.peek() {
//                 Some(c)  if c.is_numeric() || c =='-' => {  
//                     self.pos += 1;  
//                     self.col += 1;
//                 },
                    
//                 _ => break,
//             }
//         }
//         SExp::Number(self.input[start..self.col].parse().unwrap())
        
//     }

//     fn lex_bool(&mut self) -> SExp {
//         let start = self.pos;
//         self.pos += 1;
//         self.col += 1;
//         let value = &self.input[start..=self.col]; 
       
//         if value =="#t" || value == "#f" {
//             return SExp::Bool(String::from(value));
//         } else {
//             panic!("line {}:{} unexpected char: {}", self.linenumber, self.col, value);
//         } 
        
//     }

//      fn lex_symbol(&mut self) -> SExp {
//         let start = self.pos;
//         loop {
//             match self.peek() {
//                 Some(c)  if self.is_valid_for_identifier(c) =>{  
//                     self.pos += 1;  
//                     self.col += 1;
//                 },
                    
//                 _ => break,
//             }
//         }
//         SExp::Symbol(self.input[start..self.col].to_string())
        
//     }

//     fn is_valid_for_identifier(&self, c:char) -> bool {
//         match c {
//             '!' |'$'|'%' | 'a'..='z'|'A'..='Z'|'0'..='9'|'+'|'-'|'*'|'/' => true,
//             _ => false,
//         }
        
//     }

// }


// impl<'a> Iterator for Lexer<'a> {
//     type Item = SExp;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.read() {
//             SExp::EOF  => None,
//             t       => Some(t),   
//         }
        
//     }
// }


// #[cfg(test)]
// pub mod tests {
//     use super::*;

//     #[test]
//     fn test_number() {

//         assert_eq!(SExp::Number(0), Lexer::new("0\n").read());

//         assert_eq!(SExp::Number(12345),Lexer::new("12345\n").read());
    
//     }
//     #[test]
//     fn test_bool()  {
//         assert_eq!(SExp::Bool(String::from("#t")), Lexer::new("#t").read() );
//         assert_eq!(SExp::Bool(String::from("#f")), Lexer::new("#f").read() );
//     }
// }
