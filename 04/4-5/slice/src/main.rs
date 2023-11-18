fn main() {
    let s1: String = String::from("Hello, world!");

    println!("{}", first_word(&s1));

    let s2: &str = "Hello, World";

    println!("{}", first_word(s2));
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[ .. i];
        }
    }

    return &s[ .. ];
}