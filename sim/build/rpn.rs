#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(u64),
    Operator(Operator),
    LParen,
    RParen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    ShiftLeft,
    ShiftRight,
}

fn precedence(op: &Operator) -> i32 {
    match op {
        Operator::Add | Operator::Sub => 2,
        Operator::ShiftLeft | Operator::ShiftRight => 1,
    }
}

// aggressive tokenizer assuming a well-formed input
fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut chars = expr
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .peekable();
    let mut tokens = Vec::new();

    while let Some(ch) = chars.next() {
        match ch {
            '0'..'9' => {
                let mut n = ch.to_digit(10).unwrap() as u64;
                while let Some(pk) = chars.peek().and_then(|ch| ch.to_digit(10)) {
                    n = n * 10 + pk as u64;
                    chars.next();
                }
                tokens.push(Token::Number(n));
            }
            // consume extra token for </>
            '<' => {
                tokens.push(Token::Operator(Operator::ShiftLeft));
                chars.next();
            }
            '>' => {
                tokens.push(Token::Operator(Operator::ShiftRight));
                chars.next();
            }
            '+' => tokens.push(Token::Operator(Operator::Add)),
            '-' => tokens.push(Token::Operator(Operator::Sub)),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            _ => return Err(format!("Unrecognized character: {}", ch)),
        }
    }

    Ok(tokens)
}

fn to_rpn(tokens: &[Token]) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut op_stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token.clone()),
            Token::Operator(op) => {
                while let Some(Token::Operator(top_op)) = op_stack.last() {
                    if precedence(top_op) >= precedence(op) {
                        output.push(op_stack.pop().unwrap());
                    }
                }
                op_stack.push(token.clone());
            }
            Token::LParen => op_stack.push(token.clone()),
            Token::RParen => {
                let mut matched = false;
                while let Some(top) = op_stack.pop() {
                    if top == Token::LParen {
                        matched = true;
                        break;
                    } else {
                        output.push(top);
                    }
                }
                if !matched {
                    return Err("Unmatched right parenthesis".to_string());
                }
            }
        }
    }

    while let Some(top) = op_stack.pop() {
        match top {
            Token::LParen | Token::RParen => return Err("Unmatched parentheses".to_string()),
            _ => output.push(top),
        }
    }

    Ok(output)
}

fn eval_rpn(tokens: &[Token]) -> Result<u64, String> {
    let mut stack: Vec<u64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operator(Operator::Add) => {
                let b = stack.pop().ok_or("Not enough operands for '+'")?;
                let a = stack.pop().ok_or("Not enough operands for '+'")?;
                stack.push(a + b);
            }
            Token::Operator(Operator::Sub) => {
                let b = stack.pop().ok_or("Not enough operands for '-'")?;
                let a = stack.pop().ok_or("Not enough operands for '-'")?;
                stack.push(a - b);
            }
            Token::Operator(Operator::ShiftLeft) => {
                let b = stack.pop().ok_or("Not enough operands for '<<'")?;
                let a = stack.pop().ok_or("Not enough operands for '<<'")?;
                stack.push(a << b);
            }
            Token::Operator(Operator::ShiftRight) => {
                let b = stack.pop().ok_or("Not enough operands for '>>'")?;
                let a = stack.pop().ok_or("Not enough operands for '>>'")?;
                stack.push(a >> b);
            }
            _ => return Err("Unexpected token".to_string()),
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("Invalid expression".to_string())
    }
}

pub fn eval_expr(expr: &str) -> Result<u64, String> {
    let tokens = tokenize(expr)?;
    let rpn = to_rpn(&tokens)?;
    return eval_rpn(&rpn);
}
