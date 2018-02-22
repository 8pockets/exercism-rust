pub fn reply(message: &str) -> &str {
    if message.trim() == "" {
        "Fine. Be that way!"
    } else if !message.contains(char::is_lowercase) && message.to_lowercase() != message
        && message.trim().ends_with("?")
    {
        "Calm down, I know what I'm doing!"
    } else if !message.contains(char::is_lowercase) && message.to_lowercase() != message {
        "Whoa, chill out!"
    } else if message.trim().ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}
