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


    fn get_list(&mut self) -> Vec<AstNode> {
        let mut list :Vec<AstNode> = Vec::new();
        loop {
              if let Some(exp) = self.get_expr() {
                    if exp ==")" {
                        return list;
                    } else {
                        list.push(exp);
                    }
                }
            }
        }



    fn get_expr(&mut self) -> Option<AstNode> {
        match self.lexer.next() {
            Some(SExp::Number(i)) => Some(AstNode::Number(i)),
            Some(SExp::Bool(i)) => if i =="#f" {
                                    return Some(AstNode::Bool(false));
                                } else {
                                    return Some(AstNode::Bool(true));
                                },
            Some(SExp::LParen) => {unreachable!()},
            Some(SExp::RParen) => {panic!("");}
            _ => None,                   
        }
    }

    // fn into_ast(arg: Type) -> RetType {
    //     unimplemented!();
    // }

}