#[derive(Debug)]
struct UserLedger {
    name: String,
}

impl Drop for UserLedger {
    fn drop(&mut self) {
        println!("Dropping UserLedger with name '{:?}'..", self.name);
    }
}
fn main() {
    let global_ledger = UserLedger {
        name: String::from("global"),
    };

    let local_ledger: UserLedger = UserLedger {
        name: String::from("local"),
    };
    println!("Global ledger created {:?}", global_ledger);
    println!("Local ledger created {:?}", local_ledger);
    // now trying to call drop method early
    drop(local_ledger);
    // uncommenting below line will cause compilation error
    // println!("Local ledger after drop called {:?}", local_ledger);
}
