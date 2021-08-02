pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    list.iter()
        .zip(list[1..].iter())
        .fold("".to_string(), |acc, (&actual_word, &next_word)| {
            acc + &format!(
                "For want of a {} the {} was lost.\n",
                actual_word, next_word
            )
        })
        + &format!("And all for the want of a {}.", list[0])
}
