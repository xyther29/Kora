#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_integer_expression() {
        let expression = Expression::Integer(22);

        assert_eq!(expression, Expression::Integer(22));
    }

    #[test]
    fn creates_float_expression() {
        let expression = Expression::Float(12.1415);

        assert_eq!(expression, Expression::Float(12.1415));
    }

    #[test]
    fn creates_string_expression() {
        let expression = Expression::String("Kora".to_string());

        assert_eq!(expression, Expression::String("Kora".to_string()));
    }

    #[test]
    fn creates_identifier_expression() {
        let expression = Expression::Identifier("age".to_string());

        assert_eq!(expression, Expression::Identifier("age".to_string()));
    }
    #[test]
    fn creates_binary_add_expression() {
        let expression = Expression::Binary {
            left: Box::new(Expression::Integer(22)),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Integer(10)),
        };

        assert_eq!(
            expression,
            Expression::Binary {
                left: Box::new(Expression::Integer(22)),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Integer(10)),
            }
        );
    }
}
#[test]
fn creates_binary_subtract_expression() {
    let expression = Expression::Binary {
        left: Box::new(Expression::Integer(22)),
        operator: BinaryOperator::Subtract,
        right: Box::new(Expression::Integer(10)),
    };
    assert_eq!(
        expression,
        Expression::Binary {
            left: Box::new(Expression::Integer(22)),
            operator: BinaryOperator::Subtract,
            right: Box::new(Expression::Integer(10)),
        }
    );
}
