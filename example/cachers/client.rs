use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // get key hello from server
    let result = client.get("hello").await?;

    println!("got value from server; result={:?}", result);

    Ok(())
}
