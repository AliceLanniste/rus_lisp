
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
pub enum SErr {
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
pub fn tokenize(input: String) -> Vec<String> {
    input.replace("(", "( ").replace(")", " )")
    .split_whitespace()
    .map(|x| x.to_string())
    .collect()
}


pub fn parse(tokens: &[String])->Result<SExp,SErr>  {
   let (first, elements) = tokens.split_first().expect("error");
   match first.as_str() {
      "(" => parse_seq(elements),
       ")" => Err(SErr::Msg("invalid".to_string())),
       _ => Ok(parse_atom(first)),
   }
}

fn parse_seq(tokens:&[String]) -> Result<SExp,SErr>{
    let mut list = Vec::new();
    let mut t = tokens;
    loop {
        let (first,rest) = t.split_first().ok_or(SErr::Msg("bad".to_string()))?;

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


pub fn make_Env() -> GlobalEnv{
   let mut env: HashMap<&str, SExp> = HashMap::new();
    env.insert("+",SExp::Func( |args: &[SExp]|{
        let sum = parse_list_of_values(args)?.iter().sum();
        Ok(SExp::Number(sum))
    }));
  
//    env.insert("-")
    env.insert(">", SExp::Func(|args: &[SExp]|{
        let numbers = parse_list_of_values(args)?;
        (compare(|a, b| a > b, &numbers))
    }));

    //cons
    //
    GlobalEnv {Global:env }
}



// + -
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


//bool


// fn compare<F>(closure:F,args:&[f64]) -> bool 
//         where F: Fn(&f64, &f64) -> bool{
//         let first = &args[0];
//         let rest = &args[1..];
//         match rest.first() {
//         Some(second) => closure(first,second) && compare(closure,&rest[1..]),
//         None => true,
//     } 
// }
      
fn compare<F>(closure:F,args:&[f64]) -> Result<SExp,SErr> 
        where F: Fn(&f64, &f64) -> bool {
             let first = args.first().ok_or(SErr::Msg("expected at least one number".to_string()))?;
             let rest =&args[1..];
             Ok(SExp::Bool(helper(closure, first, rest)))
 
}
      
fn helper<F>(closure:F,first:&f64,args:&[f64]) -> bool
        where F: Fn(&f64, &f64) -> bool {
           match args.first() {
               Some(second) => closure(first,second)&&helper(closure,second,&args[1..]),
               None => true,
           }
        }

fn eval_parse(exp: &SExp,env:&GlobalEnv) -> Result<SExp,SErr>  {

   match exp {
       SExp::Bool(_) => Ok(exp.clone()),
       SExp::Number(_n) =>Ok(exp.clone()),
       SExp::Symbol(s) => env.env_get(s).ok_or(SErr::Msg(format!("unexpected symbol {}", s))),
       SExp::List(list) => {
         let fir = list.first().ok_or(SErr::Msg("empty".to_string()))?;
         let args = &list[1..];
         match eval_parse(fir,env)? {
             SExp::Func(f) => f(args),
             _ =>Err(SErr::Msg("error".to_string())),
         }
       
       },
       SExp::Func(_F) => unimplemented!()
   }
}


pub fn eval(text:String, env:&GlobalEnv) ->Result<SExp,SErr> {
   let parsed_exp = parse(&tokenize(text))?;
  let evaled_exp = eval_parse(&parsed_exp, env)?;

  Ok(evaled_exp)
}


pub mod test {
 
   use super::*;

   struct TestEnv {
       env: GlobalEnv,
    
   }

   impl TestEnv {
       fn new() -> Self {
           let env = make_Env();
           Self{
               env,
           }
       }

      
   }

    fn test_plus()   {
        let test_env = TestEnv::new();
        // assert_eq!(3, env.env_get(key: &str))
      
    }

}