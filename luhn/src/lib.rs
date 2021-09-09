/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();

    if code.len() <= 1 {
        return false;
    }

    if code.chars().any(|c| !c.is_ascii_digit() && c != ' ') {
        return false;
    }

    code.chars()
        .filter(|c| c != &' ')
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(index, number)| {
            let multiplier = if index % 2 == 0 { 1 } else { 2 };

            let number = number * multiplier;
            if number > 9 {
                return number - 9;
            }

            number
        })
        .sum::<u32>()
        % 10
        == 0
}
