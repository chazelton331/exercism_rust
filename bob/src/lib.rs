pub fn reply(inquiry: &str) -> String {
    if inquiry.is_empty() {
        "Fine. Be that way!".to_string()
    } else if inquiry.to_uppercase() == inquiry {
        "Whoa, chill out!".to_string()
    } else if inquiry.chars().last().unwrap() == '?' {
        "Sure.".to_string()
    } else {
        "Whatever.".to_string()
    }
}
