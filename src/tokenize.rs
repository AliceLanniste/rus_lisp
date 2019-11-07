use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GlobalEnv {
   Global :HashMap<String,SExp>,
}

impl GlobalEnv {
   pub fn new() -> Self {
        Self{
            Global: HashMap::new(),
        }
    }

    fn insert(&mut self,key:String,value:SExp) {
        self.Global.insert(key,value);
    }

   pub fn get(&self, key:String) -> Result<SExp,&'static str> {
        unimplemented!();
    }
}



#[derive(Debug)]
pub enum SExp {
    Number(f64),
    Bool(bool),
    Symbol(String),
    List(Vec<SExp>),
}




impl fmt::Display for SExp {
    
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result {
       let des = match self {
            SExp::Bool(a) => a.to_string(),
            SExp::Number(f) => f.to_string(),
            SExp::Symbol(s) => s.clone(),
            SExp::List(list) => {
           let xs: Vec<String> = list
                  .iter()
                  .map(|x| x.to_string())
                  .collect();
            format!("({})", xs.join(","))
          },
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


pub fn parse(tokens: &[String])->Result<SExp,&'static str>  {
   let (first, elements) = tokens.split_first().expect("error");
   match first.as_str() {
      "(" => parse_seq(elements),
       ")" => Err("invalid"),
       _ => Ok(parse_atom(first)),
   }
}

fn parse_seq(tokens:&[String]) -> Result<SExp,&'static str>{
    let mut list = Vec::new();
    let mut t = tokens;
    loop {
        let (first,rest) = t.split_first().ok_or("bad")?;

        if first == ")" {
            return Ok(SExp::List(list))
        }

       let exp =parse(t)?;
       list.push(exp);
       t = rest; 
    }
   
    
}

fn parse_number(exp: &str) -> SExp {
        match exp.parse::<f64>() {
            Ok(v) => SExp::Number(v),
            Err(_) => SExp::Symbol(exp.to_string().clone()),
        }
   
}


fn parse_atom(expr:&str) -> SExp {
    
    match expr {
        "#f" =>SExp::Bool(false),
        "#t" => SExp::Bool(true),
        _  => parse_number(expr),
    }
}





pub fn eval(exp: &SExp)  {

   match exp {
       SExp::Bool(b) => println!("bool is: {}", b),
       SExp::Number(n) =>println!("number is: {}", n),
       SExp::Symbol(_s) => unimplemented!(),
       SExp::List(list) => println!("list is {:?}", list),
   }
}