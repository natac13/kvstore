use std::collections::HashMap;
fn main() {
    // get the arguments from the command line
    let mut arguments = std::env::args().skip(1);
    let action = arguments.next().unwrap();
    let key = arguments.next().unwrap();

    // create a new file
    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", contents).unwrap();

    let mut database = Database::new().expect("Database::new failed");

    if (action == "get") {
        println!(
            "Value: '{}'",
            database.map.get(&key).unwrap_or(&"Not found".to_owned())
        );
    } else if (action == "set") {
        let value = arguments.next().unwrap();
        println!("Key: '{}', Value: '{}'", key, value);
        database.insert(key.to_uppercase(), value.clone());
        database.insert(key, value);
    } else {
        panic!("Invalid action: {}", action);
    }
}

struct Database {
    map: HashMap<String, String>,
    flushed: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };

        // this is the same as the above code
        // where we bind the result to contents and bubble up the error up to the caller
        let mut map = HashMap::new();
        // std::path::PathBuf::from("kv.db");
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");

            map.insert(key.to_owned(), value.to_owned());
        }
        // parse the String
        // populate the map
        Ok(Database {
            map,
            flushed: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // when we take ownership of self, we can't use it anymore
    // this the enforces the flush to be the last thing called
    // therefore we know no more key, value pairs will be added to the db
    fn flush(mut self) -> std::io::Result<()> {
        self.flushed = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = do_flush(self);
        }
        // we cannot call self.flush() here as it will try to take ownership of self
        // and we only have a mutable reference to self
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair);

        // this is more efficient than the above as we don't need to allocate a new string
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    // this says we don't care about the returned result, which is a Result<(), std::io::Error>
    std::fs::write("kv.db", contents)
}
