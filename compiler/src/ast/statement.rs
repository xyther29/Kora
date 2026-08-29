use super::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
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
}
