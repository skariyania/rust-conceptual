use std::future::Future;

fn main() {
    // calling sync function bar
    let _ = bar();
    // above line will not execute bar function yet, as async are lazy loaded
    // we will get to see below line printed only
    println!("end main");
}

async fn foo() -> u8 {
    println!("executing in foo");
    5
}
fn bar() -> impl Future<Output = u8> {
    async {
        let x: u8 = foo().await;
        println!("executing in bar");
        x + 5
    }
}

// This is how we write async function
async fn fun(x: &u8) -> u8 {
    *x
}

// Is equivalent to this function
fn fun_expanded_internal<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}
