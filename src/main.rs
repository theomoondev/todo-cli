use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Failed to initialize db");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo added"),
            Err(error) => println!("An error occurred: {}", error),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo completed"),
                Err(error) => println!("An error occurred: {}", error),
            }
        }
    }
}

struct Todo {
    // use rust built-in hashmap to store key-val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error > {
        // configure how to open the "db.json" file by defining various OpenOptions
        // (e.g., the create(true) flag will create the file if it's not already present)
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("db.json")?;
        // serialize json as HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(error) if error.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(error) => panic!("An error occurred: {}", error),
        }
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // pass true as value
        self.map.insert(key, true);
    }

    // save takes ownership of self to "enforce" save as the last method to be used. the compiler
    // would stop us if we were to accidentally try to update the map after we called save
    // (as the memory of self would be freed).
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    // complete returns the result of the Match expression which will be either an empty Some() or None
    fn complete(&mut self, key: &String) -> Option<()> {
        // get mutable reference to the value of key, or None if the value is not present in the collection
        match self.map.get_mut(key) {
            // de-reference the value and set it to false
            Some(value) => Some(*value = false),
            None => None,
        }
    }
}
