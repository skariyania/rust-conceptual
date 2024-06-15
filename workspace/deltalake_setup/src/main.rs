use deltalake::{open_table, DeltaTableError};

#[tokio::main]
async fn main() -> Result<(), DeltaTableError> {
    let table = open_table("./workspace/deltalake_setup/data/delta/").await?;
    let files_count = table.get_files_count();
    println!("files count in delta lake {:?}", files_count);
    Ok(())
}
