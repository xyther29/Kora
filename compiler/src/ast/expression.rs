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
        let expr = Expression::Integer(42);
        assert_eq!(expr, Expression::Integer(42));
    }
}
