use std::fmt;

#[derive(Debug)]
pub enum SExp {
    Bool(bool),
    Number(f64),
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
}


#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    linenumber: u32,
    pos: u32,
}




impl<'a> Lexer<'a> {
    pub fn new(text:&'a str) -> Self {

        Lexer{
            input:text,
            linenumber: 1,
            pos:0,
        }

    }

  
    fn peek(&mut self, offset: usize) -> &str {
        let mut iter = self.input[..].windows(offset);
         match iter.next() {
             Some(v) => v,
             None => "\0",
         }
    }

    
    pub fn read(&mut self) -> SExp {
        let mut tokens = self.input.chars();
        let cstart = self.pos; 
        // "123456",确定1为数字
        while let Some(c) = tokens.next() {
            if c.is_numeric() {
                //如何移动position,c必须是会变动的
                let mut n = c;
                // 问题是如何循环测试 
                while n.is_numeric() {
                    self.pos += 1;
                   
                    n = match tokens.next() {
                       Some(v)  => v,
                       None   => break,
                   };

                }
                let value = self.input[(cstart as usize)..(self.pos as usize)].parse().unwrap();
            return SExp::Number(value);
            }
           


            
        }
        
      return SExp::EOF;

    }
}

    //先把input 12 
//     pub fn reader_number(&self) ->Result<f64,std::num::ParseFloatError>  {
//        self.input.parse::<f64>()
        
//     }

//     pub fn read_bool(&self) -> Result<SExp,SExpError>{
//         match &self.input[..] {
//              "#t"  => Ok(SExp::Bool(true)),
//              "#f"  => Ok(SExp::Bool(false)),
//               _    => Err(SExpError::ParseError), 
//         }
//     }
// }