use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Program {
    Statements(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        identifier: String,
        value: Expression,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<TypeAnnotation>,
        body: Block,
    },
    FunctionCall(Expression), // Function calls can also be standalone statements
    Assignment {
        identifier: String,
        value: Expression,
    },
    Print(Expression),
    IfStatement {
        condition: Condition,
        then_block: Block,
        else_block: Option<Block>,
    },
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: TypeAnnotation,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_expression: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum Condition {
    Comparison {
        left: Expression,
        operator: ComparativeOperator,
        right: Expression,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Identifier(String),
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    ArithmeticExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug)]
pub enum ComparativeOperator {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Int,
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while self.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(Program::Statements(statements))
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn get_current_and_next(&mut self) -> Option<&Token<'a>> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    fn expect(&mut self, expected: Token<'a>) -> Result<(), String> {
        if let Some(token) = self.peek() {
            if *token == expected {
                self.get_current_and_next();
                Ok(())
            } else {
                Err(format!(
                    "Expected {:?} at position {:?}, found {:?}",
                    expected, self.pos, token
                ))
            }
        } else {
            Err(format!("Expected {:?}, but found EOF", expected))
        }
    }

    fn lookahead(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos + 1)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Some(Token::This) => {
                let declaration = self.parse_variable_declaration()?;
                self.expect(Token::SemiColon)?;
                Ok(declaration)
            }
            Some(Token::Identifier(_)) => {
                if self.lookahead() == Some(&Token::Equal) {
                    let assignment = self.parse_assignment()?;
                    self.expect(Token::SemiColon)?;
                    Ok(assignment)
                } else if self.lookahead() == Some(&Token::LeftParen) {
                    let function_call = self.parse_function_call_expression()?;
                    self.expect(Token::SemiColon)?;
                    Ok(Statement::FunctionCall(function_call))
                } else {
                    Err("Invalid statement".to_string())
                }
            }
            Some(Token::Func) => {
                let func_decl = self.parse_function_declaration()?;
                self.expect(Token::SemiColon)?;
                Ok(func_decl)
            }
            Some(Token::Print) => {
                self.next(); // consume the Print token
                self.expect(Token::LeftParen)?;
                let expression = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                self.expect(Token::SemiColon)?;
                Ok(Statement::Print(expression))
            }
            Some(Token::If) => {
                self.next(); // consume the If token
                let condition = self.parse_condition()?;
                let then_block = self.parse_block()?;
                let mut else_block = None;
                if self.peek() == Some(&Token::Else) {
                    self.next(); // consume Else token
                    else_block = Some(self.parse_block()?);
                }
                self.expect(Token::SemiColon)?;
                Ok(Statement::IfStatement {
                    condition,
                    then_block,
                    else_block,
                })
            }

            _ => Err("Invalid statement".to_string()),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        self.expect(Token::This)?;
        let name = if let Some(Token::Identifier(name)) = self.get_current_and_next() {
            name.to_string()
        } else {
            return Err("Expected an identifier after 'this'".to_string());
        };
        self.expect(Token::Equal)?;
        let value = self.parse_expression()?;
        Ok(Statement::VariableDeclaration {
            identifier: name,
            value,
        })
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, String> {
        self.expect(Token::Func)?;
        let name = self.get_identifier()?;
        self.expect(Token::LeftParen)?;
        let parameters = self.parse_parameter_list()?;
        self.expect(Token::RightParen)?;

        let return_type = if let Some(Token::Arrow) = self.peek() {
            self.next(); // consume the arrow
            self.expect(Token::Identifier("int"))?;
            Some(TypeAnnotation::Int) // Assume type annotation as Int for simplicity
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(Statement::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, String> {
        let mut parameters = Vec::new();
        while let Some(Token::Identifier(name)) = self.peek() {
            let param_name = name.to_string();
            self.next();
            self.expect(Token::Colon)?;

            // Assume Int for simplicity
            self.expect(Token::Identifier("int"))?;
            parameters.push(Parameter {
                name: param_name,
                type_annotation: TypeAnnotation::Int,
            });

            // Continue with the next tokens
            if let Some(Token::Comma) = self.peek() {
                self.next();
            } else {
                break;
            }
        }
        Ok(parameters)
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(Token::LeftBracket)?;
        let mut statements = Vec::new();
        let mut return_expression: Option<Expression> = None;

        while let Some(token) = self.peek() {
            if *token == Token::RightBracket {
                break;
            }
            if *token == Token::Return {
                self.next(); // consume the Return token
                return_expression = Some(self.parse_expression()?);
                self.expect(Token::SemiColon)?;
                break;
            }
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::RightBracket)?;
        Ok(Block {
            statements,
            return_expression,
        })
    }

    fn parse_function_call_expression(&mut self) -> Result<Expression, String> {
        let name = self.get_identifier()?;
        self.expect(Token::LeftParen)?;
        let arguments = self.parse_argument_list()?;
        self.expect(Token::RightParen)?;

        Ok(Expression::FunctionCall { name, arguments })
    }

    fn parse_argument_list(&mut self) -> Result<Vec<Expression>, String> {
        let mut arguments = Vec::new();
        while let Some(token) = self.peek() {
            if *token == Token::RightParen {
                break;
            }
            arguments.push(self.parse_expression()?);
            if let Some(Token::Comma) = self.peek() {
                self.next();
            } else {
                break;
            }
        }
        Ok(arguments)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let left = self.parse_term()?;

        // process the next token
        if let Some(token) = self.peek() {
            if matches!(
                token,
                Token::Divide | Token::Minus | Token::Plus | Token::Multiply
            ) {
                let expression = self.parse_arithmetic_expression(left)?;
                return Ok(expression);
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let token = self.peek().ok_or("Unexpected end of input".to_string())?;
        match token {
            Token::Integer(value) => {
                let int_expression = Expression::Integer(*value);
                self.next();
                Ok(int_expression)
            }
            Token::Identifier(name) => {
                let identifier = name.to_string();
                // Check if this is a function call
                if self.lookahead() == Some(&Token::LeftParen) {
                    let function_call = self.parse_function_call_expression()?;
                    Ok(function_call)
                } else {
                    // It's a standalone identifier
                    self.next();
                    Ok(Expression::Identifier(identifier))
                }
            }
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err("Invalid term".to_string()),
        }
    }

    fn parse_assignment(&mut self) -> Result<Statement, String> {
        // Parse the identifier
        let identifier = match self.peek() {
            Some(Token::Identifier(name)) => name.to_string(),
            _ => return Err("Expected identifier for assignment".to_string()),
        };

        self.next();
        // Expect and consume the '=' token
        self.expect(Token::Equal)?;

        // Parse the expression after '='
        let value = self.parse_expression()?;

        Ok(Statement::Assignment { identifier, value })
    }

    fn parse_arithmetic_expression(&mut self, left: Expression) -> Result<Expression, String> {
        // because we need to consume the identifier first and check on the mathematics operator
        // to know if it's a arithmetic exp
        // so the current token at this step is an operator
        // E.g: 3 + 5 * 4
        //        ^--- we are currently here, and we pass along the number 3 to handle the whole exp
        self.parse_expression_with_precedence(0, left)
    }

    fn parse_expression_with_precedence(
        &mut self,
        min_precedence: u8,
        left: Expression,
    ) -> Result<Expression, String> {
        let mut res = left;
        while let Some(operator) = self.peek_operator() {
            let precedence = self.operator_precedence(&operator);
            if precedence < min_precedence {
                break;
            }

            self.next(); // Consume the operator
                         // parse the current term to pass along
            let current_term = self.parse_term()?;
            let right = self.parse_expression_with_precedence(precedence + 1, current_term)?;
            res = Expression::ArithmeticExpression {
                left: Box::new(res),
                operator,
                right: Box::new(right),
            };
        }

        Ok(res)
    }

    fn peek_operator(&self) -> Option<Operator> {
        match self.peek() {
            Some(Token::Plus) => Some(Operator::Add),
            Some(Token::Minus) => Some(Operator::Subtract),
            Some(Token::Multiply) => Some(Operator::Multiply),
            Some(Token::Divide) => Some(Operator::Divide),
            _ => None,
        }
    }

    fn operator_precedence(&self, operator: &Operator) -> u8 {
        match operator {
            Operator::Multiply | Operator::Divide => 2,
            Operator::Add | Operator::Subtract => 1,
        }
    }

    fn parse_condition(&mut self) -> Result<Condition, String> {
        let left = self.parse_expression()?;
        let operator = match self.get_current_and_next() {
            Some(Token::CompareEqual) => ComparativeOperator::Equal,
            Some(Token::CompareNotEqual) => ComparativeOperator::NotEqual,
            _ => return Err("Unsupported comparative operator".to_string()),
        };
        let right = self.parse_expression()?;

        Ok(Condition::Comparison {
            left,
            operator,
            right,
        })
    }

    fn get_identifier(&mut self) -> Result<String, String> {
        if let Some(Token::Identifier(name)) = self.get_current_and_next() {
            Ok(name.to_string())
        } else {
            Err("Expected function name".to_string())
        }
    }
}
