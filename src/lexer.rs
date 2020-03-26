
use std::str::Chars;
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
    Float(f64),
    WhiteSpace,
    LParen,
    RParen,
    Symbol(String),
    Comment,
    EOF,
}
 impl fmt::Display for SExp{
         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            SExp::Bool(b) => format!("bool {}",b),
            SExp::Number(n) =>  format!("Number {}",n),
             SExp::Float(f) =>  format!("Float {}",f),
            SExp::WhiteSpace => format!("WhiteSpace"),
            SExp::Comment => format!("Comment"),
            SExp::LParen =>  format!("LParen c"),
            SExp::RParen =>  format!("RParen c"),
            SExp::Symbol(s) =>  format!("Symbol {}",s),
            SExp::EOF =>  format!("EOF"),
        };

        write!(f, "{}", description)
    }
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
}

impl <'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Lexer{
            chars:text.chars(),
            prev:'\0',
            line :1,
            col: 1,
           
        }
    }

    pub fn current_char(&self) -> char{
        self.nth(0)
    }

    pub fn peek_char(&self) ->char {
        self.nth(1)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
        Some(c)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn nth(&self, offset:usize) -> char {
        self.chars().nth(offset).unwrap()
       
    }

    fn prev(&self) -> char {
        self.prev
    }

    fn chars(&self) ->Chars<'a>{
        self.chars.clone()
    }


    pub fn read(&mut self) ->Token {
        let first_char = self.next_char().unwrap();
        match first_char {
            '-' => self.negativeoridentifer(),
            '#' => self.lex_bool(),
            c if is_valid_for_identifier(c) => self.lex_symbol(),
            c if is_digit(c)=> self.lex_number(),
            _ => panic!(" bad token"),
        }
      
    }

    fn negativeoridentifer(&mut self) ->Token{
      if self.current_char().is_digit(9){
          self.lex_number()
      } else {
          self.lex_symbol()
      }
        
    }

    fn lex_number(&mut self) ->Token {
        debug_assert!(self.prev()>='0' && self.prev()<='9' || self.prev() =='-');
        let start = self.col;
        let value=self.prev().to_string();
        let mut seed = false;
        let meta_data = Span {
                beginLineNumber:self.line,
                endLineNumber:self.line,
                beginIndex:start,
                endIndex:self.col,
            };
       
        if seed {
            
           return Token {tokentype: SExp::Float(value.parse::<f64>().unwrap()),
                  metaData:meta_data}
        }else {
           return Token {tokentype: SExp::Number(value.parse::<i64>().unwrap()),
         metaData:meta_data}
        }
         
        // 无法识别float类型
       
    }

    fn lex_symbol(&mut self) -> Token {
          let start = self.col;
         let line = self.line;
       
        let mut value: String = self.prev().to_string();
         loop {
             match self.next_char() {
                 Some(c) if is_valid_for_identifier(c)=>{value.push(c); self.col +=1;},
                 _ => break,
             }
         }
        let token = Token {
            tokentype: SExp::Symbol(value),
            metaData: Span {
                beginLineNumber:line,
                endLineNumber:line,
                beginIndex:start,
                endIndex:self.col,
            }
        };
        token
    }

    fn lex_bool(&mut self) ->Token {
        debug_assert!(self.current_char()=='t' || self.current_char()=='f');
        let start = self.col;
        self.col +=1;
        let mut value = "#".to_string();
        value.push(self.next_char().unwrap());

        Token{
            tokentype:SExp::Bool(value),
            metaData: Span{
                beginLineNumber:self.line,
                endLineNumber:self.line,
                beginIndex:start,
                endIndex:self.col,
            }
        }
        
    }
    

}


fn is_valid_for_identifier(c:char) -> bool {
        match c {
            '!' |'$'|'%' | 'a'..='z'|'A'..='Z'|'+'|'-'|'*'|'/' => true,
            _ => false,
        }
        
    }

fn is_digit(c:char) -> bool {
    c.is_digit(9)
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

pub fn helper(token: Token) -> SExp {
        token.tokentype
}
#[cfg(test)]
pub mod tests {
    use super::*;
    
   
    // #[test]
    // fn test_number() {

    //     assert_eq!(SExp::Number(0), Lexer::new("0\n").read());

    //     assert_eq!(SExp::Number(12345),Lexer::new("12345\n").read());
    
    // }
    // #[test]
    // fn test_bool()  {
    //     assert_eq!(SExp::Bool(String::from("#t")), Lexer::new("#t").read() );
    //     assert_eq!(SExp::Bool(String::from("#f")), Lexer::new("#f").read() );
    // }

    fn helper2(token: Token) -> String {
        token.tokentype.to_string()
    }

    #[test]
    fn test_identifier()  {
        let value = Lexer::new("-").read();
        // let output = SExp::Symbol(String::from("-"));
        assert_eq!(format!("Symbol -"), helper2(value));
        let value2 = Lexer::new("a").read();
         assert_eq!(format!("Symbol a"), helper2(value2));
    }
    #[test]
     fn test_bool()  {
        assert_eq!(format!("bool #t"), helper2(Lexer::new("#t").read()) );
        assert_eq!(format!("bool #f"), helper2(Lexer::new("#f").read()) );
    }
}
