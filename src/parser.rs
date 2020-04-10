// use crate::lexer::{Lexer, SExp};
// use std::fmt;

//TODO
//根据lexer建立AST

// #[derive(Clone)]
// pub enum AstNode {
//     Number(i64),
//     Bool(bool),
//     List(Vec<AstNode>),
//     Symbol(String),
//     Func(SCallbe)
// }

// impl fmt::Display for AstNode {

//     fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result {
//         let des = match *self {
//             AstNode::Number(n) => n.to_string(),
//             AstNode::Bool(b) => if b {
//                "#t".to_string()
//                 } else {
//                     "#f".to_string()
//                 },
//             AstNode::Symbol(ref s) =>  format!("{}", s),
//             AstNode::List(ref list) => {
//                 let xs: Vec<String> = list
//                   .iter()
//                   .map(|x| x.to_string())
//                   .collect();
//             format!("({})", xs.join(" . "))

//             },
//             AstNode::Func(ref _function) =>  format!("<callable >"),
//         };

//          write!(f,"{}", des)
//     }
// }

// type SCallbe = fn(&[AstNode]) -> Result<AstNode, AstErr>;

// #[derive(Debug)]
// pub enum AstErr {
//    Msg(String),
// }

// #[derive(Debug)]
// pub struct Parser<'a> {
//     lexer: Lexer<'a>
// }

// impl<'a> Parser<'a> {
//     pub fn new (text: &'a str) -> Self{
//         Parser{
//             lexer: Lexer::new(text)
//         }
//     }

//     pub fn parse(&mut self) -> Option<AstNode> {
//         match self.lexer.next() {
//             Some(expr) => self.get_expr2(Some(expr)),
//             None => None,
//         }
//     }

//     fn get_list(&mut self)  -> Vec<AstNode>{
//         let mut list : Vec<AstNode> = Vec::new();
//         loop {
//             match self.lexer.next() {
//                 Some(SExp::RParen)  =>{ return list;},
//                 token => {
//                     if let Some(exp)  = self.get_expr2(token) {

//                      list.push(exp);
//                     }
//                 },

//             }
//         }

//     }

//     fn get_expr2(&mut self, token:Option<SExp>) ->Option<AstNode> {
//         match token {
//             Some(SExp::Number(i)) => Some(AstNode::Number(i)),
//             Some(SExp::Bool(i)) => if i =="#f" {
//                                         return Some(AstNode::Bool(false));
//                                     } else {
//                                         return Some(AstNode::Bool(true));
//                                     },
//             Some(SExp::Symbol(s)) => Some(AstNode::Symbol(s)),
//             Some(SExp::LParen) => {
//                      let list = self.get_list();
//                      return Some(AstNode::List(list)); },

//             Some(SExp::RParen) => {panic!("unexpected char '(' ");}
//             _ =>None,
//         }

//     }

// }
use self::LispErr::{ErrLispValue, ErrString};
use crate::env::Env;
use std::rc::Rc;
#[derive(Debug)]
pub enum LispValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    // Str(String),
    Sym(String),
    List(Rc<Vec<LispValue>>, Rc<LispValue>),
    Vector(Rc<Vec<LispValue>>, Rc<LispValue>),
    Func(fn(LispArgs) -> LispRet, Rc<LispValue>),
    LispFunc {
        eval: fn(ast: LispValue, env: Env) -> LispRet,
        ast: Rc<LispValue>,
        env: Env,
        params: Rc<LispValue>,
        is_macro: bool,
        meta: Rc<LispValue>,
    },
}

#[derive(Debug)]
pub enum LispErr {
    ErrString(String),
    ErrLispValue(LispValue),
}

pub type LispArgs = Vec<LispValue>;
pub type LispRet = Result<LispValue, LispErr>;

macro_rules! list {
    ($seq:expr) => {{
      List(Rc::new($seq),Rc::new(Nil))
    }};
    [$($args:expr),*] => {{
      let v: Vec<MalVal> = vec![$($args),*];
      List(Rc::new(v),Rc::new(Nil))
    }}
  }
pub fn error(s: &str) -> LispRet {
    Err(ErrString(s.to_string()))
}
