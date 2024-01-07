use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType<'a> {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String(&'a str),
    Number(f32),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType<'static>> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    lexeme: &'a str,
    line: u32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, lexeme: &'a str, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    use super::*;

    #[test]
    fn compound_tokens() {
        let scanner = Scanner::new("!=");
        let tokens = scanner.scan_tokens();
        let token = tokens.first().unwrap();
        assert!(matches!(token.token_type, TokenType::BangEqual));
    }

    #[test]
    fn string() {
        let scanner = Scanner::new(r#"("hey, yall()")"#);
        let tokens = scanner.scan_tokens();
        let token = tokens.get(1).unwrap();
        assert!(matches!(token.token_type, TokenType::String("hey, yall()")));
        assert_eq!(token.lexeme, r#""hey, yall()""#)
    }

    #[test]
    fn comment() {
        let scanner = Scanner::new(r#"// ("hey!")"#);
        let tokens = scanner.scan_tokens();
        assert!(tokens.is_empty())
    }

    #[test]
    fn parse_integer() {
        let scanner = Scanner::new("1234");
        let tokens = scanner.scan_tokens();
        let token = tokens.first().unwrap();
        assert_eq!(token.lexeme, "1234");
        assert_eq!(token.token_type, TokenType::Number(1234.0));
    }

    #[test]
    fn parse_identifier() {
        let scanner = Scanner::new("variable");
        let tokens = scanner.scan_tokens();
        let token = tokens.first().unwrap();
        assert_eq!(token.lexeme, "variable");
        assert_eq!(token.token_type, TokenType::Identifier);
    }

    #[test]
    fn parse_keyword() {
        let scanner = Scanner::new("if");
        let tokens = scanner.scan_tokens();
        let token = tokens.first().unwrap();
        assert_eq!(token.lexeme, "if");
        assert_eq!(token.token_type, TokenType::If);
    }
}
