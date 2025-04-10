use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

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
        // configure how to open the "db.txt" file by defining various OpenOptions
        // (e.g., the create(true) flag will create the file if it's not already present)
        let mut f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("db.txt")?;
        // read all the bytes and append them into the content String
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        // convert from the String type of the file to a HashMap (i.e., one of the occasions where the
        // compiler has trouble inferring the type for us, so we declare it)
        let mut map = HashMap::new();

        // loop over each lines of the file
        for entries in content.lines() {
            // split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            // insert them into HashMap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        // Return Ok
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // pass true as value
        self.map.insert(key, true);
    }

    // save takes ownership of self to "enforce" save as the last method to be used. the compiler
    // would stop us if we were to accidentally try to update the map after we called save
    // (as the memory of self would be freed).
    fn save(self) -> Result<(), std::io::Error> {
        // iterate over the map, and format each string, separating key and value with a tab
        // character and each line with a new line
        // push the formatted string into a content variable
        // write content inside a file called db.txt
        let mut content = String::new();
        for (k, v) in self.map{
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
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
