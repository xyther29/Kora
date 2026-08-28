use crate::lexer::token::TokenKind;
pub struct Lexer {
    source: Vec<char>,
    position: usize,
}
impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }
    fn current_char(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }
    pub fn next_token(&mut self) -> TokenKind {
        self.skip_whitespace();
        match self.current_char() {
            Some(ch) if ch.is_ascii_digit() => self.read_number(),
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => self.read_identifier_or_keyword(),
            Some('"') => self.read_string(),
            Some('+') => {
                self.advance();
                TokenKind::Plus
            }
            Some('-') => {
                self.advance();
                TokenKind::Minus
            }
            Some('*') => {
                self.advance();
                TokenKind::Star
            }
            Some('/') => {
                self.advance();
                TokenKind::Slash
            }
            Some('=') => {
                self.advance();
                TokenKind::Equal
            }
            Some(';') => {
                self.advance();
                TokenKind::Semicolon
            }

            Some('(') => {
                self.advance();
                TokenKind::LeftParen
            }

            Some(')') => {
                self.advance();
                TokenKind::RightParen
            }

            None => TokenKind::Eof,
            _ => {
                self.advance();
                TokenKind::Eof
            }
        }
    }
    fn read_number(&mut self) -> TokenKind {
        let mut value = String::new();
        let mut has_decimal = false;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else if ch == '.' && !has_decimal {
                has_decimal = true;
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if has_decimal {
            let number = value.parse::<f64>().unwrap();
            TokenKind::Float(number)
        } else {
            let number = value.parse::<i64>().unwrap();
            TokenKind::Integer(number)
        }
    }
    fn read_identifier_or_keyword(&mut self) -> TokenKind {
        let mut value = String::new();

        while let Some(ch) = self.current_char() {
            if !(ch.is_ascii_alphanumeric() || ch == '_') {
                break;
            }

            value.push(ch);
            self.advance();
        }

        match value.as_str() {
            "let" => TokenKind::Let,
            _ => TokenKind::Identifier(value),
        }
    }
    fn read_string(&mut self) -> TokenKind {
        self.advance();
        let mut value = String::new();
        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                break;
            }
            value.push(ch);
            self.advance();
        }
        TokenKind::String(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_integer() {
        let mut lexer = Lexer::new("123");

        let token = lexer.next_token();

        assert_eq!(token, TokenKind::Integer(123));
    }
}
#[test]
fn lexes_large_integer() {
    let mut lexer = Lexer::new("123456");

    let token = lexer.next_token();

    assert_eq!(token, TokenKind::Integer(123456));
}
#[test]
fn skips_whitespace() {
    let mut lexer = Lexer::new("   42  ");

    assert_eq!(lexer.next_token(), TokenKind::Integer(42));
    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
#[test]
fn lexes_operators() {
    let mut lexer = Lexer::new("+ - * / = ; ( )");

    assert_eq!(lexer.next_token(), TokenKind::Plus);
    assert_eq!(lexer.next_token(), TokenKind::Minus);
    assert_eq!(lexer.next_token(), TokenKind::Star);
    assert_eq!(lexer.next_token(), TokenKind::Slash);
    assert_eq!(lexer.next_token(), TokenKind::Equal);
    assert_eq!(lexer.next_token(), TokenKind::Semicolon);
    assert_eq!(lexer.next_token(), TokenKind::LeftParen);
    assert_eq!(lexer.next_token(), TokenKind::RightParen);
    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
#[test]
fn lexes_arithmetic_expression() {
    let mut lexer = Lexer::new("10 + 20 * 3;");

    assert_eq!(lexer.next_token(), TokenKind::Integer(10));
    assert_eq!(lexer.next_token(), TokenKind::Plus);
    assert_eq!(lexer.next_token(), TokenKind::Integer(20));
    assert_eq!(lexer.next_token(), TokenKind::Star);
    assert_eq!(lexer.next_token(), TokenKind::Integer(3));
    assert_eq!(lexer.next_token(), TokenKind::Semicolon);
    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
#[test]
fn lexes_identifier() {
    let mut lexer = Lexer::new("hello");

    assert_eq!(
        lexer.next_token(),
        TokenKind::Identifier("hello".to_string())
    );

    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
#[test]
fn lexes_identifier_with_number() {
    let mut lexer = Lexer::new("student123");

    assert_eq!(
        lexer.next_token(),
        TokenKind::Identifier("student123".to_string())
    );
}
#[test]
fn lexes_identifier_with_underscore() {
    let mut lexer = Lexer::new("student_name");

    assert_eq!(
        lexer.next_token(),
        TokenKind::Identifier("student_name".to_string())
    );
}
#[test]
fn lexes_let_keyword() {
    let mut lexer = Lexer::new("let");

    assert_eq!(lexer.next_token(), TokenKind::Let);
    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
#[test]
fn lexes_variable_declaration() {
    let mut lexer = Lexer::new("let age = 22;");

    assert_eq!(lexer.next_token(), TokenKind::Let);

    assert_eq!(lexer.next_token(), TokenKind::Identifier("age".to_string()));

    assert_eq!(lexer.next_token(), TokenKind::Equal);
    assert_eq!(lexer.next_token(), TokenKind::Integer(22));
    assert_eq!(lexer.next_token(), TokenKind::Semicolon);
    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
#[test]
fn lexes_float() {
    let mut lexer = Lexer::new("12.112");

    assert_eq!(lexer.next_token(), TokenKind::Float(12.112));
    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
#[test]
fn lexes_integer_and_float() {
    let mut lexer = Lexer::new("10 23.3");

    assert_eq!(lexer.next_token(), TokenKind::Integer(10));
    assert_eq!(lexer.next_token(), TokenKind::Float(23.3));
}
#[test]
fn lexes_string() {
    let mut lexer = Lexer::new("\"Kora\"");

    assert_eq!(lexer.next_token(), TokenKind::String("Kora".to_string()));

    assert_eq!(lexer.next_token(), TokenKind::Eof);
}
