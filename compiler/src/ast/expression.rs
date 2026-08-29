#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
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
}
