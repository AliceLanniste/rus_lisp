
use std::io;


pub mod tokenize;

use crate::tokenize::{eval,GlobalEnv,parse,Tokenize};


fn slurp_expr() -> String {
  let mut expr = String::new();
  
  io::stdin().read_line(&mut expr)
    .expect("Failed to read line");
  
  expr
}

fn main() ->Result<(), &'static str>{
//    let env =  &mut GlobalEnv::new();
  loop {
    println!("risp >");
    let expr = slurp_expr();
    let exp = parse(&Tokenize(expr))?;
    eval(&exp)
  }
}

