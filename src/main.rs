use std::io;
use std::io::Write;

pub mod lexer;
use crate::lexer::Lexer;


fn main() {
    let mut stdin = io::stdin();
    loop {
      print!("lisp> ");
      let mut input_line = String::new();
      io::stdout().flush().ok().expect("could not flush stdout");

      match stdin.read_line(&mut input_line){
          Ok(bytes) => {
            if bytes > 0 {
                process_line(input_line);
            } else {
              println!("exited!");
              break;
            }
          }

          Err(e) => {
            println!("Error occured while reading: {}", e);
            println!("Exiting.");
            break;
          }
      }
    }
}


fn process_line(text:String)  {
  let mut lexr = Lexer::new(text.as_str());
  let value = lexr.read();
  println!("well done {}",value);
   
}


