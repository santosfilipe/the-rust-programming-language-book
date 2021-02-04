fn main() {
    let test_string = String::from("hello world");
    let single_word = first_word(&test_string);

    println!("{}", single_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}