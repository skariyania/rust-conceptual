use futures::executor::block_on;

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

fn main() {
    // remember futures are lazy executor,
    // above line will not execute anything yet
    let hello_world_future = hello_world();

    block_on(async_main());
    block_on(hello_world_future); // run future in current thread
}
