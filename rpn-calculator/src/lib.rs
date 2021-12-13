#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::*;

macro_rules! do_operation {
    ($operator:tt, $stack:ident) => {
        match ($stack.pop(),$stack.pop()) {
            (Some(second), Some(first)) => {
                $stack.push(first $operator second);
            },
            _ => { return None; }
        }
    };
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        match input {
            Value(val) => stack.push(*val),
            Add => {
                do_operation!(+,stack);
            }
            Subtract => {
                do_operation!(-,stack);
            }
            Multiply => {
                do_operation!(*,stack);
            }
            Divide => {
                do_operation!(/,stack);
            }
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
