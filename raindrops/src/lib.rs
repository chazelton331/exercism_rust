pub fn raindrops(int: i32) -> String {
    let mut string = "".to_string();

    if int % 3 == 0 {
        string.push_str("Pling");
    }

    if int % 5 == 0 {
        string.push_str("Plang");
    }

    if int % 7 == 0 {
        string.push_str("Plong");
    }

    if string == "" {
        string.push_str(&int.to_string());
    }

    string
}
