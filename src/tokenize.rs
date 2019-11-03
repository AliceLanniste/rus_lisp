
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
pub struct Token {
    exp: Vec<char>,
    position: usize,
}


impl Token {
  pub fn new() -> Self {
      Self{
          exp:Vec::new(),
          position: 0
      }
  }


  pub fn parse_int<'a>(&mut self,line:&'a str) -> Self {
   //先分析数字 1 2
   let current = self.position;
   let mut temp_vec = Vec::new();
     let mut int_chars = line.chars();
     for i in &mut int_chars {
         if i.is_digit(10) {
             temp_vec.push(i);
             self.position +=1;
         }
     }
      
      Self{
         exp:temp_vec,
         position:current
      }
      
  }
}