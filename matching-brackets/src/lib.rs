use std::char;

pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .filter(|c| matches!(c, '{' | '[' | '(' | '}' | ']' | ')'))
        .try_fold(vec![], |mut stack, actual_bracket| match actual_bracket {
            '{' | '[' | '(' => {
                stack.push(actual_bracket);
                Some(stack)
            }
            '}' | ']' | ')' => match stack.last() {
                Some(&last_bracket) if get_initial_pair(actual_bracket) == last_bracket => {
                    stack.pop();
                    Some(stack)
                }
                _ => None,
            },
            _ => None,
        })
        .map_or(false, |stack| stack.is_empty())
}

fn get_initial_pair(bracket: char) -> char {
    match bracket {
        '}' => '{',
        ']' => '[',
        _ => '(',
    }
}
