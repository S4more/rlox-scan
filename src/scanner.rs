use std::{
    ops::{Add, AddAssign},
    str::Lines,
};

use crate::{
    error,
    token::{Token, TokenType, KEYWORDS},
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
                self.scan_token(
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

        // tokens.push(Token::new(TokenType::EOF, "", lines as u32));

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
        *token_length = 1;
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
                    // +2 because we need to include the two quotes
                    *token_length = literal_string.len() + 2;
                    Ok(TokenType::String(literal_string))
                }
                Err(err) => Err(err),
            },
            '0'..='9' => match self.scan_number_literal(line, col) {
                Ok(literal) => {
                    let number = literal.parse::<f32>().unwrap();
                    *token_length = literal.len();
                    Ok(TokenType::Number(number))
                }
                Err(err) => Err(err),
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                *token_length = self.scan_identifier(line, col, row, tokens);
                return;
            }
            _ => Err("Unexpected character"),
        };

        match token {
            Ok(token) => Self::add_token(token, line, col, row, token_length, tokens),
            Err(err_message) => error(row, err_message),
        }
    }

    fn scan_number_literal(&self, line: &'a str, col: u32) -> Result<&str, &str> {
        let mut line_iter = line.chars().enumerate().peekable();
        line_iter
            .advance_by(col as usize)
            .expect("Shouldn't happen...");

        while let Some((i, ch)) = line_iter.next() {
            if !ch.is_ascii_digit() {
                if ch == '.' {
                    match line_iter.peek() {
                        Some((_, next_char)) => {
                            if !next_char.is_ascii_digit() {
                                return Ok(line.get(col as usize..i).unwrap());
                            }
                        }
                        None => return Ok(line.get(col as usize..i).unwrap()),
                    }
                } else {
                    let number = line.get(col as usize..i).expect("Failed parsing number.");
                    return Ok(number);
                }
            }
        }

        Ok(line.get(col as usize..line.len()).unwrap())
    }

    fn scan_string_literal(&self, line: &'a str, col: u32) -> Result<&str, &str> {
        let mut line_iter = line.chars().enumerate();
        line_iter
            .advance_by((col + 1) as usize)
            .expect("Shouldn't happen...");

        for (i, char) in line_iter {
            if char == '"' {
                let literal = line.get((col + 1) as usize..i).unwrap();
                return Ok(literal);
            }
        }
        Err("Unterminated string.")
    }

    fn scan_identifier(
        &self,
        line: &'a str,
        col: u32,
        row: u32,
        tokens: &mut Vec<Token<'a>>,
    ) -> usize {
        let mut line_iter = line.chars();
        line_iter.advance_by(col as usize).unwrap();

        let mut partial_str = String::from("");
        for ch in line_iter {
            if !ch.is_alphanumeric() {
                break;
            }

            partial_str.push(ch);
        }

        if let Some(keyword) = KEYWORDS.get(&partial_str.as_str()) {
            Self::add_token(*keyword, line, col, row, &partial_str.len(), tokens);
        } else {
            Self::add_token(
                TokenType::Identifier,
                line,
                col,
                row,
                &partial_str.len(),
                tokens,
            )
        }

        partial_str.len()
    }

    fn match_next_char(&self, line: &str, col: u32, expected: char) -> bool {
        if let Some(next_char) = line.chars().nth((col + 1) as usize) {
            return next_char == expected;
        }

        false
    }

    fn add_token(
        token: TokenType<'a>,
        line: &'a str,
        col: u32,
        row: u32,
        token_length: &usize,
        tokens: &mut Vec<Token<'a>>,
    ) {
        let lexeme = line
            .get(col as usize..(col as usize) + token_length)
            .expect("Should have found lexme of token");

        tokens.push(Token::new(token, lexeme, row));
    }
}
