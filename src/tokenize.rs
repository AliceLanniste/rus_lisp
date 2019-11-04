use std::fmt;



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



fn parse_number(exp: &str) -> SExp {
        match exp.parse::<f64>() {
            Ok(v) => SExp::Int(v),
            Err(e) => SExp::Symbol(e.to_string()),
        }
   
}


fn parse_bool(expr:&str) -> SExp {
    match expr {
        "false" =>SExp::Bool(false),
        "true" => SExp::Bool(true),
        _  => unimplemented!(),
    }
}