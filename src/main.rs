use std::io;
use std::io::Write;

pub mod env;
pub mod eval;
pub mod lexer;
pub mod parser;

use crate::eval::{eval, make_env};
use crate::lexer::Lexer;
use crate::parser::Parser;

// TODO: 1.Lexer提供更好的错误提示（pos，col）单独提取出来
//2.lexer添加next()方法，
//2.Env.get()构建递归查找
//添加关键字
fn main() {
    let stdin = io::stdin();
    loop {
        print!("lisp> ");
        let mut input_line = String::new();
        io::stdout().flush().ok().expect("could not flush stdout");

        match stdin.read_line(&mut input_line) {
            Ok(bytes) => {
                if bytes > 0 {
                    process_line2(input_line);
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

fn process_line(text: String) {
    let mut lexr = Lexer::new(text.as_str());
    let value = lexr.read();
    println!("well done {}", value);
}

fn process_line2(text: String) {
    let mut parser = Parser::new(text.as_str());
    let enviroment = make_env();
    if let Some(exp) = parser.parse() {
        let value = eval(exp, &enviroment);

        println!("test {}", value.unwrap());
    }
}
