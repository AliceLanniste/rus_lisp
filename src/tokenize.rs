use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;



type SCallbe = fn(&[SExp]) -> Result<SExp, SErr>;

#[derive(Clone)]
pub enum SExp {
    Number(f64),
    Bool(bool),
    Symbol(String),
    List(Vec<SExp>),
    Func(SCallbe),
}


#[derive(Debug)]
enum SErr {
   Msg(String),
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
          SExp::Func(c) => format!("<callable >"),
        
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

// env


#[derive(Clone)]
pub struct GlobalEnv {
   Global :HashMap<&'static str,SExp>,
}

impl GlobalEnv {
    pub  fn env_get(&self, key: &str) -> Option<SExp> {
     match self.Global.get(key) {
        Some(v) => Some(v.clone()),
        None   => None
    }
}
}

    



pub fn Env() -> GlobalEnv{
   let mut env: HashMap<&str, SExp> = HashMap::new();
    env.insert("+",SExp::Func( |args: &[SExp]|{
        let sum = parse_list_of_values(args)?.iter().sum();
        Ok(SExp::Number(sum))
    }));
  
    GlobalEnv {Global:env }
}




fn parse_list_of_values(args:&[SExp]) -> Result<Vec<f64>,SErr> {
   let mut v = Vec::new();
   for i in args {
      let f =parse_single_float(i)?;
       v.push(f);
   }
   
   Ok(v)
}


fn parse_single_float(exp: &SExp) -> Result<f64, SErr> {
  match exp {
    SExp::Number(num) => Ok(*num),
    _ => Err(SErr::Msg("expected a number".to_string())),
  }
}

pub fn eval(exp: &SExp,env:GlobalEnv) -> Result<SExp,SErr>  {

   match exp {
       SExp::Bool(_) => Ok(exp.clone()),
       SExp::Number(_n) =>Ok(exp.clone()),
       SExp::Symbol(s) => env.env_get(s).ok_or(SErr::Msg(format!("unexpected symbol {}", s))),
       SExp::List(list) => unimplemented!(),
       SExp::Func(F) => unimplemented!()
   }
}

