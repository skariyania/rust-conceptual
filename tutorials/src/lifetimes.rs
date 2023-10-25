use std::fmt::Display;

#[derive(Debug)]
pub struct ImportExcerpt<'a> {
    part: &'a str,
}

pub fn run_lifetimes_example() {
    let novel = String::from("Call me Sahil. some years ago");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let i = ImportExcerpt {
        part: first_sentence,
    };
    println!("value of i excerpt, {:?}", i.part);
}
pub fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    println!("value of y {y}");
    x
}

/// .
/// this function is an example of lifetime ellission
/// some patterns does not require lifetime signatures
pub fn find_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// .
/// this function is an example of generics, lifetimes, trait bonds
/// all together combined in one implemementation
pub fn longest_with_announcements<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
