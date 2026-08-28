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
            Some(ch) if ch.is_ascii_digit() => self.read_integer(),

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

            None => TokenKind::EOF,
            _ => {
                self.advance();
                TokenKind::EOF
            }
        }
    }
    fn read_integer(&mut self) -> TokenKind {
        let mut value = String::new();
        while let Some(ch) = self.current_char() {
            if !ch.is_ascii_digit() {
                break;
            }
            value.push(ch);
            self.advance();
        }

        let number = value.parse::<i64>().unwrap();
        TokenKind::Integer(number)
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
    assert_eq!(lexer.next_token(), TokenKind::EOF);
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
    assert_eq!(lexer.next_token(), TokenKind::EOF);
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
    assert_eq!(lexer.next_token(), TokenKind::EOF);
}
