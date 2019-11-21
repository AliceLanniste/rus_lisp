
use std::io;


pub mod tokenize;

use crate::tokenize::{eval,make_Env ,SErr};


fn slurp_expr() -> String {
  let mut expr = String::new();
  
  io::stdin().read_line(&mut expr)
    .expect("Failed to read line");
  
  expr
}

fn main() {
  let  env = make_Env() ;
  loop {
    println!("risp >");
    let expr = slurp_expr();
    match eval(expr,&env) {
        Ok(res) => println!("// woo => {}", res),
        Err(e) => match e {
         SErr::Msg(e) => println!("//ouch => {}",e),},
    }
   
  }
}

