use futures::executor::block_on;

async fn hello_world() {
    println!("hello_world");
}

fn main() {
    let future = hello_world();
    // remember futures are lazy executor,
    // above line will not execute anything yet
    block_on(future); // run future in current thread
}
