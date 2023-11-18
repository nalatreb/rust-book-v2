use core::slice;
use std::io::Bytes;

fn main() {
    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let world = first_word(&my_string[0..6]);
    let world = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word =  first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

// fn main() {
//     let mut s = String::from("hello world");

//     // let word = first_word(&s);
//     // s.clear();

//     let hello: &str = &s[0..5];
//     let world: &str = &s[6..11];

//     let slice = &s[0..2];
//     let slice = &s[..2];

//     let len = s.len();

//     let slice = &s[3..len];
//     let slice = &s[3..];

//     let slice = &s[0..len];
//     let slice = &s[..];

//     println!("{hello}");
//     s.push_str("world");

//     let s2: &String = &s;
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
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

// fn second_word(s: &String) -> (usize, usize) {
//     // 
// }