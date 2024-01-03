#[derive(Debug)]
pub enum TokenType {
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
    String,
    Number,

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

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: Option<&'a str>,
    line: u32,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: u32,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
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
        assert!(matches!(token.token_type, TokenType::String));
        assert_eq!(token.literal, Some("hey, yall()"));
    }

    #[test]
    fn comment() {
        let scanner = Scanner::new(r#"// ("hey!")"#);

        let tokens = scanner.scan_tokens();
        assert!(tokens.is_empty())
    }
}
