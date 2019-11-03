
#[derive(Debug)]
enum Symbol {
   RightParen,
   LeftParen,
   Add,
   Sub 
}


#[derive(Debug)]
enum SExp {
    Int(i32),
}


#[derive(Debug)]
pub struct Token<'a> {
    exp: &'a str,
    position: usize,
}


impl Token<'_> {
  pub fn new() -> Self {
      Self{
          exp:"",
          position: 0
      }
  }


  pub fn parse_int(&mut self,line:&'static str) -> Self {
   //先分析数字 1 2
   let current = self.position;
     let mut int_chars = line.chars();
     for i in &mut int_chars {
         if i.is_digit(10) {
             self.position +=1;
         }
     }
      
      Self{
         exp: int_chars.as_str(),
         position:current
      }
      
  }
}