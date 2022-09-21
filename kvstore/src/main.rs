use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() crashed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file

        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //        return Err(error); 
        //     }
        // };
        let map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No Value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse the string
        // populate our map
        Ok(Database { map: map })
    }
}
