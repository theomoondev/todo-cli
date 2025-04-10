use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}

struct Todo {
    // use rust built-in hashmap to store key-val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // pass true as value
        self.map.insert(key, true);
    }

    // save takes ownership of self to "enforce" save as the last method to be used. the compiler
    // would stop us if we were to accidentally try to update the map after we called save
    // (as the memory of self would be freed).
    fn save(self) -> Result<(), std::io::Error> {
        // -> annotates the returned type from the function: a Result
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
}
