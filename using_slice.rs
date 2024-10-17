fn main() {
    let mut s = String::from("Hello, world");

   let word = first_word(&s);

    println!("{}", word);

    s.clear();
    //let s = String::from("hello, world");

    //let hello = &s[0..5];
    //let world = &s[6..11];

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }

        &s[..]
    }
}