#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Let,
    Identifier(String),

    Integer(i64),
    Float(f64),
    String(String),

    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    LeftParen,
    RightParen,

    Semicolon,

    Eof,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_integer_token() {
        let token = TokenKind::Integer(42);
        assert_eq!(token, TokenKind::Integer(42));
    }
}
