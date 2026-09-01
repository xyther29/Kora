use crate::lexer::token::TokenKind;
#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnterminatedString,
}
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
    pub fn next_token(&mut self) -> Result<TokenKind, LexerError> {
        self.skip_whitespace();
        match self.current_char() {
            Some(ch) if ch.is_ascii_digit() => Ok(self.read_number()),
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                Ok(self.read_identifier_or_keyword())
            }
            Some('"') => self.read_string(),
            Some('+') => {
                self.advance();
                Ok(TokenKind::Plus)
            }
            Some('-') => {
                self.advance();
                Ok(TokenKind::Minus)
            }
            Some('*') => {
                self.advance();
                Ok(TokenKind::Star)
            }
            Some('/') => {
                if self.peek() == Some('/') {
                    self.skip_comment();
                    self.next_token()
                } else {
                    self.advance();
                    Ok(TokenKind::Slash)
                }
            }
            Some('=') => {
                self.advance();
                Ok(TokenKind::Equal)
            }
            Some(';') => {
                self.advance();
                Ok(TokenKind::Semicolon)
            }

            Some('(') => {
                self.advance();
                Ok(TokenKind::LeftParen)
            }

            Some(')') => {
                self.advance();
                Ok(TokenKind::RightParen)
            }

            None => Ok(TokenKind::Eof),
            Some(ch) => {
                self.advance();
                Err(LexerError::UnexpectedCharacter(ch))
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
    fn read_string(&mut self) -> Result<TokenKind, LexerError> {
        self.advance();
        let mut value = String::new();
        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                return Ok(TokenKind::String(value));
            }
            value.push(ch);
            self.advance();
        }
        Err(LexerError::UnterminatedString)
    }
    fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
    fn peek(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_integer() {
        let mut lexer = Lexer::new("123");

        let token = lexer.next_token();

        assert_eq!(token, Ok(TokenKind::Integer(123)));
    }
}
#[test]
fn lexes_large_integer() {
    let mut lexer = Lexer::new("123456");

    let token = lexer.next_token();

    assert_eq!(token, Ok(TokenKind::Integer(123456)));
}
#[test]
fn skips_whitespace() {
    let mut lexer = Lexer::new("   42  ");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(42)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_operators() {
    let mut lexer = Lexer::new("+ - * / = ; ( )");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Plus));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Minus));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Star));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Slash));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Equal));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Semicolon));
    assert_eq!(lexer.next_token(), Ok(TokenKind::LeftParen));
    assert_eq!(lexer.next_token(), Ok(TokenKind::RightParen));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_arithmetic_expression() {
    let mut lexer = Lexer::new("10 + 20 * 3;");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(10)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Plus));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(20)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Star));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(3)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Semicolon));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_identifier() {
    let mut lexer = Lexer::new("hello");

    assert_eq!(
        lexer.next_token(),
        Ok(TokenKind::Identifier("hello".to_string()))
    );

    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_identifier_with_number() {
    let mut lexer = Lexer::new("student123");

    assert_eq!(
        lexer.next_token(),
        Ok(TokenKind::Identifier("student123".to_string()))
    );
}
#[test]
fn lexes_identifier_with_underscore() {
    let mut lexer = Lexer::new("student_name");

    assert_eq!(
        lexer.next_token(),
        Ok(TokenKind::Identifier("student_name".to_string()))
    );
}
#[test]
fn lexes_let_keyword() {
    let mut lexer = Lexer::new("let");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Let));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_variable_declaration() {
    let mut lexer = Lexer::new("let age = 22;");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Let));

    assert_eq!(
        lexer.next_token(),
        Ok(TokenKind::Identifier("age".to_string()))
    );

    assert_eq!(lexer.next_token(), Ok(TokenKind::Equal));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(22)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Semicolon));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_float() {
    let mut lexer = Lexer::new("12.112");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Float(12.112)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_integer_and_float() {
    let mut lexer = Lexer::new("10 23.3");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(10)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Float(23.3)));
}
#[test]
fn lexes_string() {
    let mut lexer = Lexer::new("\"Kora\"");

    assert_eq!(
        lexer.next_token(),
        Ok(TokenKind::String("Kora".to_string()))
    );

    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn reports_unexpected_character() {
    let mut lexer = Lexer::new("@");

    assert_eq!(
        lexer.next_token(),
        Err(LexerError::UnexpectedCharacter('@'))
    );
}
#[test]
fn reports_unterminated_string() {
    let mut lexer = Lexer::new("\"Kora");

    assert_eq!(lexer.next_token(), Err(LexerError::UnterminatedString));
}
#[test]
fn skips_comment() {
    let mut lexer = Lexer::new("// hello\n42");

    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(42)));

    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
#[test]
fn lexes_parentheses() {
    let mut lexer = Lexer::new("(22 + 10)");

    assert_eq!(lexer.next_token(), Ok(TokenKind::LeftParen));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(22)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Plus));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Integer(10)));
    assert_eq!(lexer.next_token(), Ok(TokenKind::RightParen));
    assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
}
