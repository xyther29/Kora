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
    pub fn next_token(&mut self) -> TokenKind {
        match self.current_char() {
            Some(ch) if ch.is_ascii_digit() => self.read_integer(),
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
