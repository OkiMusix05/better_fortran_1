/*// Function to determine the precedence of operators
fn precedence(operator: char) -> i32 {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

// Function to convert infix expression to postfix expression
fn infix_to_postfix(expression: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in expression.chars() {
        match token {
            '0'..='9' => output.push(token.to_string()),
            '(' => operator_stack.push(token),
            ')' => {
                while let Some(op) = operator_stack.pop() {
                    if op == '(' {
                        break;
                    }
                    output.push(op.to_string());
                }
            }
            '+' | '-' | '*' | '/' | '^' => {
                while let Some(op) = operator_stack.last() {
                    if *op == '(' || precedence(*op) < precedence(token) {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap().to_string());
                }
                operator_stack.push(token);
            }
            _ => {}
        }
    }

    while let Some(op) = operator_stack.pop() {
        output.push(op.to_string());
    }

    output
}

// Function to evaluate postfix expression
pub enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
}

fn evaluate_postfix(postfix: Vec<String>) -> Expr {
    let mut operand_stack = Vec::new();

    for token in postfix {
        match token.parse::<f64>() {
            Ok(operand) => operand_stack.push(Expr::Number(operand)),
            Err(_) => {
                let operand2 = operand_stack.pop().unwrap();
                let operand1 = operand_stack.pop().unwrap();
                let operator = match token.chars().next().unwrap() {
                    '+' => Expr::Add(Box::new(operand1), Box::new(operand2)),
                    '-' => Expr::Subtract(Box::new(operand1), Box::new(operand2)),
                    '*' => Expr::Multiply(Box::new(operand1), Box::new(operand2)),
                    '/' => Expr::Divide(Box::new(operand1), Box::new(operand2)),
                    '^' => Expr::Power(Box::new(operand1), Box::new(operand2)),
                    _ => Expr::Number(0.0), // Handle other operators if necessary
                };
                operand_stack.push(operator);
            }
        }
    }

    operand_stack.pop().unwrap()
}

// Function to separate and calculate expression
pub(crate) fn separate_and_calculate(expression: &str) -> Expr {
    let postfix = infix_to_postfix(expression);
    let expr = evaluate_postfix(postfix);
    expr
}
pub(crate) fn convert_to_function_calls(expr: Expr) -> String {
    match expr {
        Expr::Number(num) => num.to_string(),
        Expr::Add(left, right) => format!("sum({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Subtract(left, right) => format!("subtract({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Multiply(left, right) => format!("mult({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Divide(left, right) => format!("div({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Power(left, right) => format!("power({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
    }
}

pub(crate) fn func_from_exp(expression: &str) -> String {
    convert_to_function_calls(separate_and_calculate(expression))
}*/
/*use std::fmt;

// Enum to represent expressions
pub enum Expr {
    Number(f64),
    Matrix(String), // Assume a matrix variable is represented by a string
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
    MATAdd(Box<Expr>, Box<Expr>),
    MATSubtract(Box<Expr>, Box<Expr>),
    MATMultiply(Box<Expr>, Box<Expr>),
    MATDivide(Box<Expr>, Box<Expr>),
    MATScalarMultiply(Box<Expr>, Box<Expr>),
    // Add other matrix operations as needed
}
// Implement Display for Expr
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{}", num),
            Expr::Matrix(var) => write!(f, "{}", var),
            Expr::Add(left, right) => write!(f, "sum({}, {})", left, right),
            Expr::Subtract(left, right) => write!(f, "subtract({}, {})", left, right),
            Expr::Multiply(left, right) => write!(f, "mult({}, {})", left, right),
            Expr::Divide(left, right) => write!(f, "div({}, {})", left, right),
            Expr::Power(left, right) => write!(f, "power({}, {})", left, right),
            Expr::MATAdd(left, right) => write!(f, "MATAdd({}, {})", left, right),
            Expr::MATSubtract(left, right) => write!(f, "MATSubtract({}, {})", left, right),
            Expr::MATMultiply(left, right) => write!(f, "MATMultiply({}, {})", left, right),
            Expr::MATDivide(left, right) => write!(f, "MATDivide({}, {})", left, right),
            Expr::MATScalarMultiply(left, right) => write!(f, "MATScalarMultiply({}, {})", left, right),
        }
    }
}
fn precedence(operator: char) -> i32 {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}
// Function to convert infix expression to postfix expression
fn infix_to_postfix(expression: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in expression.chars() {
        match token {
            '0'..='9' => output.push(token.to_string()),
            '(' => operator_stack.push(token),
            ')' => {
                while let Some(op) = operator_stack.pop() {
                    if op == '(' {
                        break;
                    }
                    output.push(op.to_string());
                }
            }
            '+' | '-' | '*' | '/' | '^' => {
                while let Some(op) = operator_stack.last() {
                    if *op == '(' || precedence(*op) < precedence(token) {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap().to_string());
                }
                operator_stack.push(token);
            }
            _ => {}
        }
    }

    while let Some(op) = operator_stack.pop() {
        output.push(op.to_string());
    }

    output
}
// Function to evaluate postfix expression with support for matrix operations
fn evaluate_postfix(postfix: Vec<String>, matrix_vars: Vec<String>) -> Expr {
    let mut operand_stack = Vec::new();

    for token in postfix {
        match token.parse::<f64>() {
            Ok(operand) => operand_stack.push(Expr::Number(operand)),
            Err(_) => {
                if matrix_vars.contains(&token) {
                    let operand2 = operand_stack.pop().unwrap();
                    let operand1 = operand_stack.pop().unwrap();
                    let operator = match token.chars().next().unwrap() {
                        '+' => Expr::MATAdd(Box::new(operand1), Box::new(operand2)),
                        '-' => Expr::MATSubtract(Box::new(operand1), Box::new(operand2)),
                        '*' => {
                            if matrix_vars.contains(&token) && matrix_vars.contains(&operand1.to_string()) {
                                Expr::MATMultiply(Box::new(operand1), Box::new(operand2))
                            } else {
                                Expr::MATScalarMultiply(Box::new(operand1), Box::new(operand2))
                            }
                        }
                        '/' => Expr::MATDivide(Box::new(operand1), Box::new(operand2)),
                        _ => Expr::Matrix(token), // Assume it's a matrix variable
                    };
                    operand_stack.push(operator);
                } else {
                    let operand2 = operand_stack.pop().unwrap();
                    let operand1 = operand_stack.pop().unwrap();
                    let operator = match token.chars().next().unwrap() {
                        '+' => Expr::Add(Box::new(operand1), Box::new(operand2)),
                        '-' => Expr::Subtract(Box::new(operand1), Box::new(operand2)),
                        '*' => Expr::Multiply(Box::new(operand1), Box::new(operand2)),
                        '/' => Expr::Divide(Box::new(operand1), Box::new(operand2)),
                        '^' => Expr::Power(Box::new(operand1), Box::new(operand2)),
                        _ => Expr::Number(0.0), // Handle other operators if necessary
                    };
                    operand_stack.push(operator);
                }
            }
        }
    }

    operand_stack.pop().unwrap()
}

// Function to separate and calculate expression with support for matrix operations
pub(crate) fn separate_and_calculate(expression: &str, matrix_vars: Vec<String>) -> Expr {
    let postfix = infix_to_postfix(expression);
    let expr = evaluate_postfix(postfix, matrix_vars);
    expr
}

// Function to convert expression to function calls
pub(crate) fn convert_to_function_calls(expr: Expr) -> String {
    match expr {
        Expr::Number(num) => num.to_string(),
        Expr::Matrix(var) => var, // Just return the variable name
        Expr::Add(left, right) => format!("sum({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Subtract(left, right) => format!("subtract({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Multiply(left, right) => format!("mult({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Divide(left, right) => format!("div({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::Power(left, right) => format!("power({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::MATAdd(left, right) => format!("MATAdd({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::MATSubtract(left, right) => format!("MATSubtract({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::MATMultiply(left, right) => format!("MATMultiply({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::MATDivide(left, right) => format!("MATDivide({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        Expr::MATScalarMultiply(left, right) => format!("MATScalarMultiply({}, {})", convert_to_function_calls(*left), convert_to_function_calls(*right)),
        // Add other matrix operations as needed
    }
}

// Function to generate function calls from expression with support for matrix operations
pub(crate) fn func_from_exp(expression: &str, matrix_vars: Vec<String>) -> String {
    println!("Matrix: {:?}", matrix_vars);
    convert_to_function_calls(separate_and_calculate(expression, matrix_vars))
}*/
