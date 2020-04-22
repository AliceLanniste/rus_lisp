use crate::parser::LispValue::{Bool, Float, Int, List, Nil, Sym, Vector};
use crate::parser::{error, LispRet, LispValue};

use std::str::Chars;

//#[derive(Debug)]
//pub struct Span {
//    beginLineNumber: usize,
//    endLineNumber: usize,
//    beginIndex: usize,
//    endIndex: usize,
//}

fn tokenize(s: &str) -> LispRet {
    let mut lexer = Lexer::new(s);
    lexer.read()
}

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
    input: &'a str,
    prev: char,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Lexer {
            chars: text.chars(),
            input: text,
            prev: '\0',
            line: 1,
            col: 1,
        }
    }

    pub fn current_char(&self) -> char {
        self.nth(0)
    }

    fn peek_char(&self) -> char {
        self.nth(1)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
        Some(c)
    }

    // pub fn is_eof(&self) -> bool {
    //     self.chars.as_str().is_empty()
    // }

    fn nth(&self, offset: usize) -> char {
        self.chars().nth(offset).unwrap()
    }

    fn prev(&self) -> char {
        self.prev
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    pub fn read(&mut self) -> LispRet {
        let first_char = self.next_char();
        match first_char {
            Some('n') if self.current_char() == 'i' && self.peek_char() == 'l' => {
                self.col +=2;    self.chars = self.input[self.col..].chars();
                return Ok(Nil);
            }
            Some('#') => self.lex_bool(),
            Some(c) if is_digit(c) || c == '-' && is_digit(self.current_char()) => {
                self.lex_number()
            }
            Some(c) if is_first_for_identifier(c) => self.lex_symbol(),
            Some(';') => self.skip_comment(),
            Some(c) if is_whitespace(c) => self.skip_whitespace(),
            Some('(') => self.lex_seq(')'),
            Some('[') => self.lex_seq(']'),
            // //报错
            Some(')') => error("unexpected )"),
            Some(']') => error("unexpected ]"),
            _ => {println!("{}",self.prev()); Ok(Nil)},
        }
    }

    fn lex_number(&mut self) -> LispRet {
        debug_assert!(self.prev() >= '0' && self.prev() <= '9' || self.prev() == '-');
        let _start = self.col;
        let mut value = self.prev().to_string();
        let mut seed = false;
        loop {
            match self.current_char() {
                c if is_digit(c) => {
                    value.push(c);
                    self.next_char();
                    self.col += 1;
                }
                c if c == '.' && is_digit(self.peek_char()) => {
                    value.push(c);
                    self.next_char();
                    self.col += 1;
                    seed = true;
                }
                _ => break,
            }

        }

        return if seed {
            Ok(Float(value.parse::<f64>().unwrap()))
        } else {
            Ok(Int(value.parse::<i64>().unwrap()))
        };
    }

    fn lex_symbol(&mut self) -> LispRet {
        let _start = self.col;
        let _line = self.line;

        let mut value: String = self.prev().to_string();
        loop {
            match self.next_char() {
                Some(c) if is_valid_for_identifier(c) => {
                    value.push(c);
                    self.col += 1;
                }
                _ => break,
            }
        }

        Ok(Sym(value))
    }

    fn lex_bool(&mut self) -> LispRet {
        debug_assert!((self.current_char() == 't' || self.current_char() == 'f'));

        match self.next_char() {
            Some('t') => Ok(Bool(true)),
            Some('f') => Ok(Bool(false)),

            _ => unreachable!(),
        }
        
    }

    fn skip_comment(&mut self) -> LispRet {
        debug_assert!(self.prev() == ';' && self.current_char() == ';');
        while self.current_char() != '\n' {
            self.next_char();
        }
        self.read()
    }

    fn skip_whitespace(&mut self) -> LispRet {
        debug_assert!(is_whitespace(self.prev()));
        if self.prev() == '\n' {
            self.line += 1;
            self.col += 1;
        } else {
            self.col += 1;
        }
        self.read()
    }

    //    "aab"以及注意转义字符

    fn lex_string(&mut self) -> LispRet {
        unimplemented!();
    }

    fn lex_seq(&mut self, end: char) -> LispRet {
        let mut seq: Vec<LispValue> = vec![];
        loop {

            match self.current_char() {
               c if c == end  => break,
                _t=> {
                    let token =self.read().unwrap();
                    seq.push(token);
                    }
                
            }
        }
        match end {
            ')' =>Ok(List(std::rc::Rc::new(seq), std::rc::Rc::new(Nil))),
            ']' =>Ok(Vector(std::rc::Rc::new(seq), std::rc::Rc::new(Nil))),
            _ => error("read_seq unknown end value"),
        }

    }
}

fn is_whitespace(c: char) -> bool {
    match c {
        '\u{000C}' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

fn is_first_for_identifier(c: char) -> bool {
    match c {
        '_' | '!' | '$' | '%' | 'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

fn is_valid_for_identifier(c: char) -> bool {
    match c {
        '0'..='9' | '_' | '!' | '$' | '%' | 'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(9)
}

#[cfg(test)]
pub mod tests {
    use super::*;


    #[test]
    fn test_list() {
        let output =  Lexer::new("(1)").read().unwrap();
        assert_eq!("(1)",format!("{}",output));
    }
}
