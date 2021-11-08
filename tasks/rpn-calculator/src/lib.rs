#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(n) => stack.push(*n),
            _ => {
                let n2 = stack.pop()?;
                let n1 = stack.pop()?;

                let result = match input {
                    CalculatorInput::Add => n1 + n2,
                    CalculatorInput::Subtract => n1 - n2,
                    CalculatorInput::Multiply => n1 * n2,
                    CalculatorInput::Divide => n1 / n2,
                    _ => 0,
                };

                stack.push(result);
            }
        }
    }

    if stack.len() > 1 || inputs.is_empty() {
        None
    } else {
        Some(stack[0])
    }
}
