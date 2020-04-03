use crate::parser::LispValue::{Bool, Float, Int, List, Nil, Sym};
use crate::parser::{LispErr, LispRet};
use std::fmt;
use std::str::Chars;
// TODO
// 1.解析字符串

pub fn first_token(input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Lexer::new(input).read()
}

//fn tokneize(input:&str) -> impl Iterator<Item = Token>  {
//    unimplemented!()
//    // add code here
//}

#[derive(Debug, PartialEq)]
pub enum SExp {
    Bool(String),
    Number(i64),
    Float(f64),
    WhiteSpace,
    LParen,
    RParen,
    Symbol(String),
    Comment,
    Unknow,
    EOI,
}

#[derive(Debug)]
pub struct Span {
    beginLineNumber: usize,
    endLineNumber: usize,
    beginIndex: usize,
    endIndex: usize,
}

#[derive(Debug)]
pub struct Token {
    tokentype: SExp,
    metaData: Span,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
    prev: char,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Lexer {
            chars: text.chars(),
            prev: '\0',
            line: 1,
            col: 1,
        }
    }

    pub fn current_char(&self) -> char {
        self.nth(0)
    }

    pub fn peek_char(&self) -> char {
        self.nth(1)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
        Some(c)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn nth(&self, offset: usize) -> char {
        self.chars().nth(offset).unwrap()
    }

    fn prev(&self) -> char {
        self.prev
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    pub fn read(&mut self) -> Token {
        let first_char = self.next_char();
        match first_char {
            Some('#') => self.lex_bool(),
            Some(c) if is_digit(c) || c == '-' && is_digit(self.current_char()) => {
                self.lex_number()
            }
            Some(c) if is_valid_for_identifier(c) => self.lex_symbol(),
            Some(';') => self.skip_comment(),
            Some(c) if is_whitespace(c) => self.skip_whitespace(),
            Some('(') => self.lex_Paren(SExp::LParen),
            Some(')') => self.lex_Paren(SExp::RParen),
            Some(_c) => {
                let token = Token {
                    tokentype: SExp::Unknow,
                    metaData: Span {
                        beginLineNumber: self.line,
                        endLineNumber: self.line,
                        beginIndex: self.col,
                        endIndex: self.col + 1,
                    },
                };
                return token;
            }
            None => {
                let token = Token {
                    tokentype: SExp::EOI,
                    metaData: Span {
                        beginLineNumber: self.line,
                        endLineNumber: self.line,
                        beginIndex: self.col,
                        endIndex: self.col + 1,
                    },
                };

                return token;
            }
        }
    }

    fn lex_number(&mut self) -> Token {
        debug_assert!(self.prev() >= '0' && self.prev() <= '9' || self.prev() == '-');
        let start = self.col;
        let mut value = self.prev().to_string();
        let mut seed = false;
        loop {
            match self.next_char() {
                Some(c) if is_digit(c) => {
                    value.push(c);
                    self.col += 1;
                }
                Some(c) if c == '.' && is_digit(self.current_char()) => {
                    value.push(c);
                    self.col += 1;
                    seed = true;
                }
                _ => break,
            }
        }
        let meta_data = Span {
            beginLineNumber: self.line,
            endLineNumber: self.line,
            beginIndex: start,
            endIndex: self.col,
        };

        if seed {
            return Token {
                tokentype: SExp::Float(value.parse::<f64>().unwrap()),
                metaData: meta_data,
            };
        } else {
            return Token {
                tokentype: SExp::Number(value.parse::<i64>().unwrap()),
                metaData: meta_data,
            };
        }
    }

    fn lex_symbol(&mut self) -> Token {
        let start = self.col;
        let line = self.line;

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
        let token = Token {
            tokentype: SExp::Symbol(value),
            metaData: Span {
                beginLineNumber: line,
                endLineNumber: line,
                beginIndex: start,
                endIndex: self.col,
            },
        };

        token
    }

    fn lex_bool(&mut self) -> Token {
        debug_assert!(self.current_char() == 't' || self.current_char() == 'f');

        let mut value = "#".to_string();
        value.push(self.next_char().unwrap());

        Token {
            tokentype: SExp::Bool(value),
            metaData: Span {
                beginLineNumber: self.line,
                endLineNumber: self.line,
                beginIndex: self.col,
                endIndex: self.col + 1,
            },
        }
    }

    fn skip_comment(&mut self) -> Token {
        debug_assert!(self.prev() == ';' && self.current_char() == ';');
        while self.current_char() != '\n' {
            self.next_char();
        }
        self.read()
    }

    fn skip_whitespace(&mut self) -> Token {
        debug_assert!(is_whitespace(self.prev()));
        if self.prev() == '\n' {
            self.line += 1;
            self.col += 1;
        } else {
            self.col += 1;
        }
        self.read()
    }

    fn lex_string(&mut self) -> Token {
        unimplemented!();
    }

    fn lex_Paren(&self, t: SExp) -> Token {
        Token {
            tokentype: t,
            metaData: Span {
                beginLineNumber: self.line,
                endLineNumber: self.line,
                beginIndex: self.col,
                endIndex: self.col,
            },
        }
    }
}

fn is_whitespace(c: char) -> bool {
    match c {
        '\u{000C}' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

fn is_valid_for_identifier(c: char) -> bool {
    match c {
        '!' | '$' | '%' | 'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(9)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    impl fmt::Display for SExp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let description = match self {
                SExp::Bool(b) => format!("bool {}", b),
                SExp::Number(n) => format!("Number {}", n),
                SExp::Float(f) => format!("Float {}", f),
                SExp::WhiteSpace => format!("WhiteSpace"),
                SExp::Comment => format!("Comment"),
                SExp::LParen => format!("LParen c"),
                SExp::RParen => format!("RParen c"),
                SExp::Symbol(s) => format!("Symbol {}", s),
                SExp::Unknow => format!("Unknown"),
                SExp::EOI => format!("END"),
            };

            write!(f, "{}", description)
        }
    }

    fn helper(token: Token) -> String {
        token.tokentype.to_string()
    }

    #[test]
    fn test_number() {
        assert_eq!(format!("Number 0"), helper(Lexer::new("0").read()));
        assert_eq!(format!("Number 12345"), helper(Lexer::new("12345").read()));
        assert_eq!(
            format!("Number -12345"),
            helper(Lexer::new("-12345").read())
        );
        assert_eq!(
            format!("Float -123.45"),
            helper(Lexer::new("-123.45").read())
        );
    }

    #[test]
    fn test_identifier() {
        let value2 = Lexer::new("a").read();
        assert_eq!(format!("Symbol a"), helper(value2));
        let value = Lexer::new("-\n").read();
        assert_eq!(format!("Symbol -"), helper(value));
    }

    #[test]
    fn test_bool() {
        assert_eq!(format!("bool #t"), helper(Lexer::new("#t").read()));
        assert_eq!(format!("bool #f"), helper(Lexer::new("#f").read()));
    }

    #[test]
    fn test_end() {
        assert_eq!(format!("END"), helper(Lexer::new("").read()))
    }
}
