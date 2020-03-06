pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if x.is_empty() => "Fine. Be that way!",
        x if x.ends_with("?") => match x {
            x if is_uppercase(x) => "Calm down, I know what I'm doing!",
            _ => "Sure.",
        },
        x if is_uppercase(x) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_uppercase(string: &str) -> bool {
    (string.clone().to_ascii_uppercase() == string)
        && (string != string.clone().to_ascii_lowercase())
}
