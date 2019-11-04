use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
enum Symbol {
   RightParen,
   LeftParen,
   Add,
   Sub 
}

// type Result<T> = std::result::Result<T,SError>;

#[derive(Debug)]
enum SExp {
    Int(f64),
    Bool(bool),
    Symbol(String),
    // List(Vec<SExp>)
}

#[derive(Debug)]
enum SError {
    Error(String),
}


impl fmt::Display for SExp {
    
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result {
       let des = match *self {
            SExp::Bool(a) => a.to_string(),
            SExp::Int(f) => f.to_string(),
            SExp::Symbol(s) => s.clone(),
            // SExp::List(list) => {}
        };

        write!(f,"{}", des)
    }
}

//分成token
pub fn Tokenize(input: String) -> Vec<String> {
    input.replace("(", "( ").replace(")", " )")
    .split_whitespace()
    .map(|x| x.to_string())
    .collect()
}



fn parse_number(exp: String) -> SExp {

    match exp.parse::<f64>() {
        Ok(v) => SExp::Int(v),
        Err(e) => SExp::Symbol(e.to_string()),
    }


    
}



