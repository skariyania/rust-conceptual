use rocksdb::{Options, DB};

fn main() {
    println!("Hello, world!");
    let path = "rocksdb-example/data";
    {
        let db = DB::open_default(path).unwrap();
        db.put(b"name", b"sahil").unwrap();
        for i in 0..1000 {
            db.put(String::from(i.to_string()), b"sahil").unwrap();
            if i % 100 == 0 {
                println!("{} records inserted ..", i);
            }
        }
        match db.get(b"name") {
            Ok(Some(value)) => println!("found value: {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"name").unwrap();
    }
    let _ = DB::destroy(&Options::default(), path);
}
