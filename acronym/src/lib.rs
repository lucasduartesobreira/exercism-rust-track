pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", " ")
        .replace("_", " ")
        .split(' ')
        .map(|s| match s {
            "" => "".to_string(),
            _ if s.to_uppercase() == s => return s.chars().next().unwrap().to_string(),
            _ => s
                .chars()
                .enumerate()
                .filter_map(|(index, c)| {
                    if index == 0 {
                        Some(c.to_ascii_uppercase())
                    } else if c.is_ascii_uppercase() {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect(),
        })
        .collect()
}
