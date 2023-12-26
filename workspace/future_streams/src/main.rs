use futures::{
    select,
    stream::{FusedStream, Stream, StreamExt},
};

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_sum) = item {
            total += next_sum;
        }
    }

    total
}

fn main() {
    println!("Hello, world!");
}
