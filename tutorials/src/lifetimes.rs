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
    x
}
