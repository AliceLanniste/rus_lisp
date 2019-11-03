use std::io;


pub mod tokenize;

fn main() {
  
    let mut token = tokenize::Token::new();
    loop {
        println!("lisp> ");
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line) {
            Ok(bytes) => {
                   if bytes > 0 {
                      
                       token.parse_int(&input_line);
                   }else {
                      println!("input invalid!");
                      break;
                   }
            }
            Err(error) => {
                println!("Error occured while reading: {}", error);
                println!("Exiting.");
                break;
            }
        }
    }
}


