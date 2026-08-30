use crate::lexer::TokenKind;

pub struct Parser{
    tokens: Vec<TokenKind>
    position:usize,
}
impl Parser{
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
}
