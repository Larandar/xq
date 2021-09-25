use crate::{
    ast::UnaryOp,
    vm::{bytecode::NamedFn1, QueryExecutionError},
    Value,
};

pub(crate) fn unary(operator: &UnaryOp) -> NamedFn1 {
    match operator {
        UnaryOp::Plus => NamedFn1 {
            name: "UnaryPlus",
            func: unary_plus,
        },
        UnaryOp::Minus => NamedFn1 {
            name: "UnaryMinus",
            func: unary_minus,
        },
    }
}

fn unary_plus(value: Value) -> Result<Value, QueryExecutionError> {
    match value {
        Value::Number(_) => Ok(value),
        _ => Err(QueryExecutionError::UnaryOnNonNumeric("plus", value)),
    }
}

fn unary_minus(value: Value) -> Result<Value, QueryExecutionError> {
    match value {
        Value::Number(n) => Ok(Value::number(-(*n).clone())),
        _ => Err(QueryExecutionError::UnaryOnNonNumeric("minus", value)),
    }
}
