use crate::ast::{BinaryOperator, Expression, Program, Statement};
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
            Some(_) => self.parse_expression_statement(),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression()?;

        self.expect_token(&TokenKind::Semicolon)?;

        Ok(Statement::Expression(expression))
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
        let left = match self.current_token() {
            Some(TokenKind::Integer(value)) => {
                let expression = Expression::Integer(*value);
                self.advance();
                expression
            }
            Some(TokenKind::Float(value)) => {
                let expression = Expression::Float(*value);
                self.advance();
                expression
            }
            Some(TokenKind::String(value)) => {
                let expression = Expression::String(value.clone());
                self.advance();
                expression
            }
            Some(TokenKind::Identifier(name)) => {
                let expression = Expression::Identifier(name.clone());
                self.advance();
                expression
            }
            Some(token) => {
                return Err(ParserError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: token.clone(),
                });
            }
            None => return Err(ParserError::UnexpectedEndOfInput),
        };

        if let Some(token) = self.current_token() {
            let operator = if *token == TokenKind::Plus {
                BinaryOperator::Add
            } else if *token == TokenKind::Minus {
                BinaryOperator::Subtract
            } else {
                return Ok(left);
            };

            self.advance();

            let right = self.parse_expression()?;

            return Ok(Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        Ok(left)
    }
    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements = Vec::new();
        while self.current_token().is_some() {
            if self.current_token() == Some(&TokenKind::Eof) {
                break;
            }
            let statement = self.parse_statement()?;
            statements.push(statement);
        }
        Ok(Program { statements })
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
#[test]
fn parses_multiple_let_statements() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Equal,
        TokenKind::Integer(22),
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Identifier("name".to_string()),
        TokenKind::Equal,
        TokenKind::String("Hritik".to_string()),
        TokenKind::Semicolon,
        TokenKind::Eof,
    ];

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![
                Statement::Let {
                    name: "age".to_string(),
                    value: Expression::Integer(22),
                },
                Statement::Let {
                    name: "name".to_string(),
                    value: Expression::String("Hritik".to_string()),
                },
            ],
        }
    );
}
#[test]
fn parses_empty_program() {
    let tokens = vec![TokenKind::Eof];

    let mut parser = Parser::new(tokens);

    assert_eq!(parser.parse_program(), Ok(Program { statements: vec![] }));
}
#[test]
fn rejects_wrong_let_keyword() {
    let tokens = vec![
        TokenKind::Identifier("letx".to_string()),
        TokenKind::Identifier("age".to_string()),
        TokenKind::Equal,
        TokenKind::Integer(22),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert!(parser.parse_statement().is_err());
}

#[test]
fn rejects_missing_identifier() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Equal,
        TokenKind::Integer(22),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert!(parser.parse_statement().is_err());
}

#[test]
fn rejects_invalid_identifier_token() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Integer(123),
        TokenKind::Equal,
        TokenKind::Integer(22),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert!(parser.parse_statement().is_err());
}

#[test]
fn rejects_missing_equal_sign() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Integer(22),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert!(parser.parse_statement().is_err());
}

#[test]
fn rejects_invalid_expression_value() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Equal,
        TokenKind::Plus,
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert!(parser.parse_statement().is_err());
}

#[test]
fn rejects_missing_semicolon() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Equal,
        TokenKind::Integer(22),
    ];

    let mut parser = Parser::new(tokens);

    assert!(parser.parse_statement().is_err());
}
#[test]
fn parses_expression_statement() {
    let tokens = vec![
        TokenKind::Identifier("age".to_string()),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_statement(),
        Ok(Statement::Expression(Expression::Identifier(
            "age".to_string()
        )))
    );
}
#[test]
fn parses_integer_expression_statement() {
    let tokens = vec![TokenKind::Integer(22), TokenKind::Semicolon];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_statement(),
        Ok(Statement::Expression(Expression::Integer(22)))
    );
}

#[test]
fn parses_float_expression_statement() {
    let tokens = vec![TokenKind::Float(12.5), TokenKind::Semicolon];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_statement(),
        Ok(Statement::Expression(Expression::Float(12.5)))
    );
}

#[test]
fn parses_string_expression_statement() {
    let tokens = vec![TokenKind::String("hello".to_string()), TokenKind::Semicolon];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_statement(),
        Ok(Statement::Expression(Expression::String(
            "hello".to_string()
        )))
    );
}

#[test]
fn parses_identifier_expression_statement() {
    let tokens = vec![
        TokenKind::Identifier("age".to_string()),
        TokenKind::Semicolon,
    ];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_statement(),
        Ok(Statement::Expression(Expression::Identifier(
            "age".to_string()
        )))
    );
}
#[test]
fn parses_program_with_multiple_statement_types() {
    let tokens = vec![
        TokenKind::Let,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Equal,
        TokenKind::Integer(22),
        TokenKind::Semicolon,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Semicolon,
        TokenKind::Float(12.5),
        TokenKind::Semicolon,
        TokenKind::String("hello".to_string()),
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Identifier("name".to_string()),
        TokenKind::Equal,
        TokenKind::Integer(12),
        TokenKind::Semicolon,
        TokenKind::Eof,
    ];

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![
                Statement::Let {
                    name: "age".to_string(),
                    value: Expression::Integer(22),
                },
                Statement::Expression(Expression::Identifier("age".to_string())),
                Statement::Expression(Expression::Float(12.5)),
                Statement::Expression(Expression::String("hello".to_string())),
                Statement::Let {
                    name: "name".to_string(),
                    value: Expression::Integer(12),
                },
            ],
        }
    );
}
#[test]
fn parses_addition_expression() {
    let tokens = vec![
        TokenKind::Integer(22),
        TokenKind::Plus,
        TokenKind::Integer(10),
    ];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_expression(),
        Ok(Expression::Binary {
            left: Box::new(Expression::Integer(22)),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Integer(10)),
        })
    );
}
#[test]
fn parses_subtraction_expression() {
    let tokens = vec![
        TokenKind::Integer(22),
        TokenKind::Minus,
        TokenKind::Integer(10),
    ];

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse_expression(),
        Ok(Expression::Binary {
            left: Box::new(Expression::Integer(22)),
            operator: BinaryOperator::Subtract,
            right: Box::new(Expression::Integer(10)),
        })
    );
}
