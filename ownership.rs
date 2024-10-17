fn main() {
    let s = String::from("hello");
    println!("{}", s);
    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = String::from("hello");

    let _s3 = takes_and_give_back(s1);

    let s4 = String::from("hello");

    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s5, len);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integers: i32) {
    println!("{}", some_integers);
}

fn _gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}