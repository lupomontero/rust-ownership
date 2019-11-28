// fn foo(s: &String) {
//     println!("{:?}", s);
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // let a = "hello";
    // let b = a;

    // let a = String::from("hello");
    // let b = &a;
    // foo(&a);
    // foo(b);
    //
    // println!("{}", a);
    // println!("{}", b);

    let first_from_literal = first_word("oh my god");

    let literal = "oh my god";
    let first_from_literal_slice = first_word(&literal[..]);

    let string = String::from("oh my god");
    let first_from_string_slice = first_word(&string[..]);

    println!("{:?}", first_from_literal);
    println!("{:?}", first_from_literal_slice);
    println!("{:?}", first_from_string_slice);
}
