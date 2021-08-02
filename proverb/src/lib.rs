pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    let mut proverb = String::from("");

    let mut actual_word = list[0];
    for &next_word in list[1..].iter() {
        proverb.push_str(
            format!(
                "For want of a {} the {} was lost.\n",
                actual_word, next_word
            )
            .as_str(),
        );
        actual_word = next_word;
    }
    proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());

    proverb
}
