use deltalake::{open_table, DeltaTableError};

#[tokio::main]
async fn main() -> Result<(), DeltaTableError> {
    let table = open_table("./workspace/deltalake_setup/data/delta/").await?;
    let files = table.get_files();
    println!("{:?}", files);
    Ok(())
}
