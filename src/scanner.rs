use std::str::Lines;

use crate::{
    error,
    token::{self, Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let line_iterator: Lines = self.source.lines();

        line_iterator.enumerate().for_each(|(row, line)| {
            let mut token_length: usize = 1;
            let mut char_iterator = line.chars().enumerate();
            while let Some((col, char)) = char_iterator.next() {
                let token_size = self.scan_token(
                    &char,
                    line,
                    col as u32,
                    row as u32,
                    &mut token_length,
                    &mut tokens,
                );

                let _ = char_iterator.advance_by(token_length - 1);
            }
        });

        // let lines = self.source.lines().count();

        // tokens.push(Token::new(
        //     TokenType::EOF,
        //     "".to_string(),
        //     None,
        //     lines as u32,
        // ));

        tokens
    }

    fn scan_token(
        &'a self,
        char: &char,
        line: &'a str,
        col: u32,
        row: u32,
        token_length: &mut usize,
        tokens: &mut Vec<Token<'a>>,
    ) {
        let mut literal = None;
        let token = match char {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::Semicolon),
            '*' => Ok(TokenType::Star),
            '!' => {
                if self.match_next_char(line, col, '=') {
                    Ok(TokenType::BangEqual)
                } else {
                    Ok(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next_char(line, col, '=') {
                    *token_length = 2;
                    Ok(TokenType::EqualEqual)
                } else {
                    Ok(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next_char(line, col, '=') {
                    *token_length = 2;
                    Ok(TokenType::LessEqual) // x <= y (x = 3, y = 3) => true
                } else {
                    Ok(TokenType::Less) // x < y (x = x, y = 3) => false
                }
            }
            '>' => {
                if self.match_next_char(line, col, '>') {
                    *token_length = 2;
                    Ok(TokenType::GreaterEqual)
                } else {
                    Ok(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_next_char(line, col, '/') {
                    *token_length = line.len();
                    return;
                } else {
                    Ok(TokenType::Slash)
                }
            }
            ' ' | '\r' | 't' => {
                return;
            }
            '"' => match self.scan_string_literal(line, col) {
                Ok(literal_string) => {
                    literal = Some(literal_string);
                    *token_length = literal_string.len();
                    Ok(TokenType::String)
                }
                Err(err) => Err(err),
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => todo!(),
            _ => Err("Unexpected character"),
        };

        match token {
            Ok(token) => Self::add_token(token, literal, line, col, row, token_length, tokens),
            Err(err_message) => error(row, err_message),
        }
    }

    fn scan_number_literal(&self, line: &'a str, col: u32) -> Result<&str, &str> {
        let mut line_iter = line.chars();
        line_iter
            .advance_by(col as usize)
            .expect("Shouldn't happen...");

        todo!()
    }

    fn scan_string_literal(&self, line: &'a str, col: u32) -> Result<&str, &str> {
        let mut line_iter = line.chars();
        line_iter
            .advance_by((col + 1) as usize)
            .expect("Shouldn't happen...");

        for (i, char) in line_iter.enumerate() {
            if char == '"' {
                let literal = line
                    .get((col + 1) as usize..(col + 1) as usize + i)
                    .unwrap();
                return Ok(literal);
            }
        }
        Err("Unterminated string.")
    }

    fn match_next_char(&self, line: &str, col: u32, expected: char) -> bool {
        if let Some(next_char) = line.chars().nth((col + 1) as usize) {
            return next_char == expected;
        }

        false
    }

    fn add_token(
        token: TokenType,
        literal: Option<&'a str>,
        line: &'a str,
        col: u32,
        row: u32,
        token_length: &usize,
        tokens: &mut Vec<Token<'a>>,
    ) {
        let lexeme = if let Some(literal_value) = literal {
            literal_value
        } else {
            line.get((col + 1) as usize - *token_length..(col + 1) as usize)
                .expect("Should have found lexme of token")
        };

        tokens.push(Token::new(token, lexeme, literal, row));
    }
}
