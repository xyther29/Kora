use super::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Expression(Expression),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_let_statement() {
        let statement = Statement::Let {
            name: "age".to_string(),
            value: Expression::Integer(22),
        };

        assert_eq!(
            statement,
            Statement::Let {
                name: "age".to_string(),
                value: Expression::Integer(22),
            }
        );
    }
    fn create_expression() {
        let statement = Statement::Expression(Expression::Identifier("age".to_string()));

        assert_eq!(
            statement,
            Statement::Expression(Expression::Identifier("age".to_string()))
        );
    }
}
