use futures::{executor::block_on, try_join, TryFutureExt};

async fn hello_world() {
    println!("hello_world");
}

async fn learn(skill: String) {
    println!("learning {}", skill)
}

async fn sing() {
    learn(String::from("how to sing")).await;
    println!("I can sing")
}

async fn dance() {
    println!("I can dance")
}

async fn async_main() {
    let learn = learn(String::from("programming"));
    let sing = sing();
    let dance = dance();

    // as specified in book, join! macro is like await,
    // but it can wait for multiple futures
    futures::join!(sing, dance, learn);
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}
#[derive(Debug)]
struct Music {
    genre: String,
}

async fn get_book(title: String, author: String) -> Result<Book, String> {
    println!("searching book.. {} {}", title, author);
    Ok(Book { author, title })
}

async fn get_music(genre: String) -> Result<Music, String> {
    println!("searching music.. {}", genre);
    Ok(Music { genre })
}

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book(
        "Sahil Kariyania".to_string(),
        "Rust Conceptual Learnings".to_string(),
    )
    .map_err(|_| "Unable to get book".to_string());
    let music_fut = get_music("Verse".to_string()).map_err(|_| "Unable to get Music".to_string());
    try_join!(book_fut, music_fut)
}
fn main() {
    // remember futures are lazy executor,
    // above line will not execute anything yet
    let hello_world_future = hello_world();

    block_on(async_main());
    block_on(hello_world_future); // run future in current thread

    // this is an example to use try_join
    // let details = get_book_and_music();
    // details
    let (book_details, music_details) = block_on(get_book_and_music()).unwrap();
    println!("Book={:#?} and music={:#?}", book_details, music_details);
}
