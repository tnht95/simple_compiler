use crate::parser::{Block, Condition, Expression, Operator, Program, Statement};

pub struct Optimizer;

impl Optimizer {
    pub fn optimize_ast(program: Program) -> Program {
        match program {
            Program::Statements(statements) => Program::Statements(
                statements
                    .into_iter()
                    .map(|stmt| Self::optimize_statement(stmt))
                    .collect(),
            ),
        }
    }

    fn optimize_statement(statement: Statement) -> Statement {
        match statement {
            Statement::VariableDeclaration { identifier, value } => {
                Statement::VariableDeclaration {
                    identifier,
                    value: Self::constant_fold(&value),
                }
            }
            Statement::FunctionDeclaration {
                name,
                parameters,
                return_type,
                body,
            } => Statement::FunctionDeclaration {
                name,
                parameters,
                return_type,
                body: Self::optimize_block(body),
            },
            Statement::FunctionCall(Expression) => {
                Statement::FunctionCall(Self::constant_fold(&Expression))
            }
            Statement::Assignment { identifier, value } => Statement::Assignment {
                identifier,
                value: Self::constant_fold(&value),
            },
            Statement::Print(expression) => Statement::Print(Self::constant_fold(&expression)),
            Statement::IfStatement {
                condition,
                then_block,
                else_block,
            } => Statement::IfStatement {
                condition: Self::optimize_condition(condition),
                then_block: Self::optimize_block(then_block),
                else_block: match else_block {
                    None => else_block,
                    Some(block) => Some(Self::optimize_block(block)),
                },
            },
        }
    }

    fn optimize_block(block: Block) -> Block {
        Block {
            statements: block
                .statements
                .into_iter()
                .map(Self::optimize_statement)
                .collect(),
            return_expression: block
                .return_expression
                .map(|expr| Self::constant_fold(&expr)),
        }
    }

    fn optimize_condition(condition: Condition) -> Condition {
        match condition {
            Condition::Comparison {
                left,
                operator,
                right,
            } => Condition::Comparison {
                left: Self::constant_fold(&left),
                operator,
                right: Self::constant_fold(&right),
            },
        }
    }
    pub fn constant_fold(expression: &Expression) -> Expression {
        match expression {
            Expression::ArithmeticExpression {
                left,
                operator,
                right,
            } => {
                let left = Optimizer::constant_fold(left);
                let right = Optimizer::constant_fold(right);

                match (left, operator, right) {
                    (Expression::Integer(l), Operator::Add, Expression::Integer(r)) => {
                        Expression::Integer(l + r)
                    }
                    (Expression::Integer(l), Operator::Subtract, Expression::Integer(r)) => {
                        Expression::Integer(l - r)
                    }
                    (Expression::Integer(l), Operator::Multiply, Expression::Integer(r)) => {
                        Expression::Integer(l * r)
                    }
                    (Expression::Integer(l), Operator::Divide, Expression::Integer(r))
                        if r != 0 =>
                    {
                        Expression::Integer(l / r)
                    }

                    // Multiplication-specific rules
                    (Expression::Integer(1), Operator::Multiply, right) => right, // 1 * x -> x
                    (left, Operator::Multiply, Expression::Integer(1)) => left,   // x * 1 -> x
                    (Expression::Integer(0), Operator::Multiply, _) => Expression::Integer(0), // 0 * x -> 0
                    (_, Operator::Multiply, Expression::Integer(0)) => Expression::Integer(0), // x * 0 -> 0

                    // Addition-specific rules
                    (Expression::Integer(0), Operator::Add, right) => right, // 0 + x -> x
                    (left, Operator::Add, Expression::Integer(0)) => left,   // x + 0 -> x

                    // If no optimizations apply, reconstruct the expression
                    (left, operator, right) => Expression::ArithmeticExpression {
                        left: Box::new(left),
                        operator: operator.clone(),
                        right: Box::new(right),
                    },
                }
            }
            other => other.clone(),
        }
    }
}
