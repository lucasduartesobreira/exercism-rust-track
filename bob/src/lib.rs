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
    message == message.to_uppercase() && message.chars().any(|c| c.is_alphabetic())
}
