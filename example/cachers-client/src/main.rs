use bytes::Bytes;

use mini_redis::{client, Result};
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: Bytes },
}
#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:63790").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key } => {
                    println!("received command= Get key = {:?}", key);
                    client.get(&key).await.unwrap();
                }
                Set { key, value } => {
                    println!("received command= Set key = {:?}, val = {:?}", key, value);
                    client.set(&key, value).await.unwrap();
                }
            }
        }
    });

    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "foo".to_string(),
        };
        tx.send(cmd).await.unwrap();
    });
    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

    Ok(())
}
