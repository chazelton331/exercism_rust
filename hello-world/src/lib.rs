pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(n) => {
            let mut s = String::new();
            s += "Hello, ";
            s += n;
            s += "!";

            s.to_string()
        },
        None    => { "Hello, World!".to_string() }
    }
}
