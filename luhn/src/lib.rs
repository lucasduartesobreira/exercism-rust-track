/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();

    if code.len() <= 1 {
        return false;
    }

    if code.chars().any(|c| !c.is_ascii_digit() && c != ' ') {
        return false;
    }

    let mut is_second = false;
    let mut n_sum = 0;

    for c in code.chars().rev().filter(|c| c != &' ') {
        let mut number = c.to_digit(10).unwrap();

        if is_second {
            number *= 2;
        }

        n_sum += number / 10;
        n_sum += number % 10;

        is_second = !is_second;
    }

    n_sum % 10 == 0

    //code.chars()
    //.filter(|c| c != &' ')
    //.map(|c| {
    //if c.is_ascii_digit() {
    //c.to_digit(10)
    //} else {
    //None
    //}
    //})
    //.enumerate()
    //.try_fold(0u32, |acc, (index, option)| match option {
    //Some(number) => {
    //let multiple = if index % 2 == 0 { 2 } else { 1 };
    //let number = number * multiple;

    //let new_acc = acc + number / 10;
    //let new_acc = new_acc + number % 10;

    /*
     *let multiplied = match number * multiple {
     *    x if x > 9 => x - 9,
     *    x => x,
     *};
     */

    //Some(new_acc)
    //}
    //None => None,
    //})
    //.unwrap_or(1)
    //% 10
    //== 0
}
