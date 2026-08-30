use crate::ast::{Expression, Statement};
use crate::lexer::TokenKind;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken { expected: String, found: TokenKind },
    UnexpectedEndOfInput,
}

pub struct Parser {
    tokens: Vec<TokenKind>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect_token(&mut self, expected: &TokenKind) -> Result<(), ParserError> {
        match self.current_token() {
            Some(token) if token == expected => {
                self.advance();
                Ok(())
            }

            Some(token) => Err(ParserError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: token.clone(),
            }),

            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token() {
            Some(TokenKind::Let) => self.parse_let_statement(),
            Some(token) => Err(ParserError::UnexpectedToken {
                expected: "statement".to_string(),
                found: token.clone(),
            }),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(&TokenKind::Let)?;

        let name = match self.current_token() {
            Some(TokenKind::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            Some(token) => {
                return Err(ParserError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: token.clone(),
                });
            }
            None => return Err(ParserError::UnexpectedEndOfInput),
        };

        self.expect_token(&TokenKind::Equal)?;

        let value = self.parse_expression()?;

        self.expect_token(&TokenKind::Semicolon)?;

        Ok(Statement::Let { name, value })
    }
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.current_token() {
            Some(TokenKind::Integer(value)) => {
                let expression = Expression::Integer(*value);
                self.advance();
                Ok(expression)
            }
            Some(TokenKind::Float(value)) => {
                let expression = Expression::Float(*value);
                self.advance();
                Ok(expression)
            }
            Some(TokenKind::String(value)) => {
                let expression = Expression::String(value.clone());
                self.advance();
                Ok(expression)
            }
            Some(TokenKind::Identifier(name)) => {
                let expression = Expression::Identifier(name.clone());
                self.advance();
                Ok(expression)
            }
            Some(token) => Err(ParserError::UnexpectedToken {
                expected: "expression".to_string(),
                found: token.clone(),
            }),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_starts_at_first_token() {
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("age".to_string()),
            TokenKind::Equal,
            TokenKind::Integer(22),
            TokenKind::Semicolon,
        ];

        let parser = Parser::new(tokens);

        assert_eq!(parser.current_token(), Some(&TokenKind::Let));
    }

    #[test]
    fn parser_can_advance() {
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("age".to_string()),
            TokenKind::Equal,
            TokenKind::Integer(22),
            TokenKind::Semicolon,
            TokenKind::Eof,
        ];

        let mut parser = Parser::new(tokens);

        assert_eq!(parser.current_token(), Some(&TokenKind::Let));

        parser.advance();

        assert_eq!(
            parser.current_token(),
            Some(&TokenKind::Identifier("age".to_string()))
        );
    }

    #[test]
    fn creates_unexpected_token_error() {
        let error = ParserError::UnexpectedToken {
            expected: "=".to_string(),
            found: TokenKind::Integer(22),
        };

        assert_eq!(
            error,
            ParserError::UnexpectedToken {
                expected: "=".to_string(),
                found: TokenKind::Integer(22),
            }
        );
    }

    #[test]
    fn creates_unexpected_end_of_input_error() {
        let error = ParserError::UnexpectedEndOfInput;

        assert_eq!(error, ParserError::UnexpectedEndOfInput);
    }

    #[test]
    fn expect_token_succeeds_when_token_matches() {
        let tokens = vec![TokenKind::Let, TokenKind::Identifier("age".to_string())];

        let mut parser = Parser::new(tokens);

        assert_eq!(parser.expect_token(&TokenKind::Let), Ok(()));

        assert_eq!(
            parser.current_token(),
            Some(&TokenKind::Identifier("age".to_string()))
        );
    }

    #[test]
    fn expect_token_reports_unexpected_token() {
        let tokens = vec![TokenKind::Let, TokenKind::Integer(22)];

        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.expect_token(&TokenKind::Identifier("age".to_string())),
            Err(ParserError::UnexpectedToken {
                expected: "Identifier(\"age\")".to_string(),
                found: TokenKind::Let,
            })
        );
    }

    #[test]
    fn expect_token_reports_end_of_input() {
        let tokens = vec![];

        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.expect_token(&TokenKind::Let),
            Err(ParserError::UnexpectedEndOfInput)
        );
    }
}
#[test]
fn parses_let_statement() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Equal,
        TokenKind::Integer(22),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    let statement = parser.parse_statement().unwrap();

    assert_eq!(
        statement,
        Statement::Let {
            name: "age".to_string(),
            value: Expression::Integer(22),
        }
    );
}
#[test]
fn parses_integer_expression() {
    let tokens = vec![TokenKind::Integer(22)];

    let mut parser = Parser::new(tokens);

    assert_eq!(parser.parse_expression(), Ok(Expression::Integer(22)));
}
#[test]
fn parses_float_expression() {
    let tokens = vec![TokenKind::Float(12.13)];
    let mut parser = Parser::new(tokens);
    assert_eq!(parser.parse_expression(), Ok(Expression::Float(12.13)));
}
#[test]
fn parses_string_expression() {
    let tokens = vec![TokenKind::String("hello".to_string())];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_expression(),
        Ok(Expression::String("hello".to_string()))
    );
}
#[test]
fn parses_identifier_expression() {
    let tokens = vec![TokenKind::Identifier("age".to_string())];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_expression(),
        Ok(Expression::Identifier("age".to_string()))
    );
}
#[test]
fn parses_let_with_float() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Identifier("decimal".to_string()),
        TokenKind::Equal,
        TokenKind::Float(12.12),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_let_statement(),
        Ok(Statement::Let {
            name: "decimal".to_string(),
            value: Expression::Float(12.12),
        })
    );
}
