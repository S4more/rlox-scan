use crate::token::KEYWORDS;
use std::{
    iter::{Enumerate, Peekable},
    str::Lines,
};

use crate::token::{Token, TokenType};

use itertools::Itertools;

trait CharIterator = Iterator<Item = (usize, char, Option<char>)>;

pub struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let line_iterator: Enumerate<Lines> = self.source.lines().enumerate();

        for (row, line) in line_iterator {
            // let mut col_iterator = line.chars().enumerate().peekable();
            // self.scan_token_test(&mut col_iterator)

            let iter = line
                .chars()
                .map(Some)
                .chain([None]) // for n-tuple n-1 `None`
                .tuple_windows()
                .enumerate()
                .filter_map(|(a, (b, c))| Some((a, b?, c)))
                .peekable();

            let mut line_tokens = self.scan_token_test(iter, line, row.try_into().unwrap());
            tokens.append(&mut line_tokens)
        }

        println!("{:?}", tokens);

        // tokens.into_iter().map(|(token_type, size)| {
        //     Token::new(token_type, lexeme, line)
        // });

        // // let lines = self.source.lines().count();

        // // tokens.push(Token::new(TokenType::EOF, "", lines as u32));

        tokens
    }

    fn scan_token_test(
        &'a self,
        mut char_iterator: Peekable<impl CharIterator>,
        line: &'a str,
        row: u32,
    ) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        while let Some((pos, current, next)) = char_iterator.next() {
            let line_length = line.len();
            let k: Result<(TokenType<'_>, usize), ()> = match current {
                ' ' | '\r' | '\t' => continue,
                '(' => Ok((TokenType::LeftParen, 1)),
                ')' => Ok((TokenType::RightParen, 1)),
                '{' => Ok((TokenType::LeftBrace, 1)),
                '}' => Ok((TokenType::RightBrace, 1)),
                ',' => Ok((TokenType::Comma, 1)),
                '.' => Ok((TokenType::Dot, 1)),
                '-' => Ok((TokenType::Minus, 1)),
                '+' => Ok((TokenType::Plus, 1)),
                ';' => Ok((TokenType::Semicolon, 1)),
                '*' => Ok((TokenType::Star, 1)),
                '!' if next.is_some_and(|c| c == '=') => Ok((TokenType::BangEqual, 2)),
                '!' => Ok((TokenType::Bang, 1)),
                '=' if next.is_some_and(|c| c == '=') => Ok((TokenType::EqualEqual, 2)),
                '=' => Ok((TokenType::Equal, 1)),
                '<' if next.is_some_and(|c| c == '=') => Ok((TokenType::LessEqual, 2)),
                '<' => Ok((TokenType::Less, 1)),
                '>' if next.is_some_and(|c| c == '=') => Ok((TokenType::GreaterEqual, 2)),
                '>' => Ok((TokenType::Greater, 1)),
                '/' if next.is_some_and(|c| c == '/') => {
                    Ok((TokenType::Comment, line_length - pos))
                }
                '/' => Ok((TokenType::Slash, 1)),
                '"' => Ok(self.scan_string(pos, line, &mut char_iterator)),
                '0'..='9' => Ok(self.scan_number(line, pos, &mut char_iterator)),
                'a'..='z' | 'A'..='Z' | '_' => {
                    Ok(self.scan_identifier(pos, line, &mut char_iterator))
                }
                _ => todo!(),
            };
            if let Ok((token_type, size)) = k {
                let lexeme = &line[pos..pos + size];
                tokens.push(Token::new(token_type, lexeme, row));
                let _ = char_iterator.advance_by(size - 1);
            };
        }
        tokens
    }

    fn scan_string(
        &self,
        initial_pos: usize,
        line: &'a str,
        char_iterator: &mut impl CharIterator,
    ) -> (TokenType<'a>, usize) {
        let start_pos = initial_pos;
        let mut end_pos = initial_pos;
        for (pos, _, next) in char_iterator.by_ref() {
            if next.is_some_and(|c| c == '"') {
                end_pos = pos + 1;
            };
        }
        // Adding +1 here because we don't want to include the first opening quote in the literal.
        let literal = &line[start_pos + 1..end_pos];
        (TokenType::String(literal), end_pos - start_pos + 1)
    }

    fn scan_number(
        &self,
        line: &'a str,
        pos: usize,
        line_iter: &mut impl CharIterator,
    ) -> (TokenType<'a>, usize) {
        let mut literal: &str = "";
        for (current_pos, current, next) in line_iter.by_ref() {
            if !current.is_ascii_digit() {
                if current == '.' && next.is_some_and(|c| c.is_ascii_digit()) {
                    // do nothing. we want to keep going until
                    // we hit the end.
                    continue;
                }
                literal = &line[pos..current_pos + 1];
            } else if next.is_none() {
                // We know that this will always arrive,
                // but the compiler does not...
                literal = &line[pos..current_pos + 1];
            }
        }

        (TokenType::Number(literal.parse().unwrap()), literal.len())
    }

    fn scan_identifier(
        &self,
        initial_pos: usize,
        line: &'a str,
        line_iter: &mut impl CharIterator,
    ) -> (TokenType<'a>, usize) {
        let mut end_pos = initial_pos;
        for (pos, _, next) in line_iter {
            if !next.is_some_and(|c| c.is_alphanumeric()) {
                end_pos = pos;
                break;
            }
        }

        let literal = &line[initial_pos..=end_pos];

        let token_type = if let Some(keyword) = KEYWORDS.get(literal) {
            *keyword
        } else {
            TokenType::Identifier
        };

        (token_type, literal.len())
    }
}
