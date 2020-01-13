use lexer::{Lexer, SExp};
use Ast::AstNode;


#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>
}

impl Parser<'a> {
    pub fn new (text: &'a str) -> Self{
        Parser{
            lexer: Lexer::new(text)
        }
    }


    fn get_expr(&mut self) -> Option<AstNode> {
        match self.lexer.next() {
            Some(SExp::Number(i)) => Some(AstNode::Number(i)),
            Some(SExp::Bool(i)) => if i =="#f" {
                                    Some(AstNode::Bool(#f))
                                } else {
                                    Some(AstNode::Bool(#t))
                                },
            Some(SExp::LParen) => {unreachable!()},
            Some(SExp::RParen) => {panic!("");}
            _ => None,                   
        }
    }
}