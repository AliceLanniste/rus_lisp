use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GlobalEnv<'a> {
   Global :HashMap<&'a str,&'a [SExp]>,
}

impl GlobalEnv<'_> {
   pub fn new() -> Self {
        Self{
            Global: HashMap::new(),
        }
    }

    fn insert(&mut self,key:&'static str,value:&'static [SExp]) {
        self.Global.insert(key,value);
    }

//    pub fn get(&self, key:&'a str) -> Result<SExp,&'static str> {
//         self.Global.get(key).ok_or("failed")?;
//     }
}


pub fn Env() -> GlobalEnv<'static>{
    let env = GlobalEnv::new();
    // SExp::Func(|values| Ok(SExp::Number(values.iter().sum())));
    // env.insert("-", value: SExp);
    // env.insert("*", value: SExp)
    env
}


#[derive(Clone)]
pub enum SExp {
    Number(f64),
    Bool(bool),
    Symbol(String),
    List(Vec<SExp>),
    Func(SCallabe),
}

type SCallabe = fn(&[SExp]) -> Result<SExp, &'static str>;



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
         SExp::Func(_) => "Function {}".to_string(),
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
       SExp::Bool(_) => println!("bool is: {}", exp.to_string().clone()),
       SExp::Number(_n) =>println!("number is: {}", exp.to_string().clone()),
       SExp::Symbol(_s) => println!("symbol is: {}", exp.to_string().clone()),
       SExp::List(list) => println!("list is {}", list),
   }
}

