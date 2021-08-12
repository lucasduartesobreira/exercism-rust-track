pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_yelling = is_yelling(message);
    let is_question = message.ends_with('?');
    match message {
        _ if is_yelling && is_question => "Calm down, I know what I'm doing!",
        _ if is_question => "Sure.",
        _ if is_yelling => "Whoa, chill out!",
        "" => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

fn is_yelling(message: &str) -> bool {
    let yelling = message.chars().try_fold(true, |only_symbols, c| {
        if c.is_ascii_lowercase() {
            return Err("Not yelling");
        }
        if c.is_ascii_uppercase() {
            return Ok(false);
        }
        Ok(only_symbols)
    });

    if let Ok(has_only_symbols) = yelling {
        if has_only_symbols {
            return false;
        }
        return true;
    }
    false
}
