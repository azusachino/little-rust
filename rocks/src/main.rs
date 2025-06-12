use rocksdb::DB;

fn main() {
    let path = "rocksdb_example";
    let db = DB::open_default(path).expect("Failed to open database");

    // Insert a key-value pair
    db.put(b"key1", b"value1").expect("Failed to put value");
    
    // Retrieve the value
    match db.get(b"key1") {
        Ok(Some(value)) => println!("Retrieved: {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("Key not found"),
        Err(e) => println!("Error retrieving value: {}", e),
    }

    // Delete the key
    db.delete(b"key1").expect("Failed to delete key");
    
    // Close the database
    drop(db);
}
