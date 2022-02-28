use serde_json;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }

    fn new_from_json() -> Result<Todo, std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }

        std::fs::write("db.txt", content)
    }

    fn save_from_json(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(file, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = true),
            None => None,
        }
    }

    fn get_all(&self) -> Result<Todo, Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        let map = serde_json::from_reader(file)?;
        
        Ok(Todo { map })
    }
}

fn main() -> Result<(), String> {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2);

    let task = match item {
        Some(item) => item,
        None => "".to_string(),
    };

    if task.to_string() == "".to_string() && action != "all".to_string() {
        return Err("Please specify action".to_string());
    }

    let mut todo = Todo::new_from_json().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(task);

        match todo.save_from_json() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error ocurred {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&task) {
            None => println!("'{}' is not present in the list", task),
            Some(_) => match todo.save_from_json() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "all" {
        match todo.get_all() {
            Ok(map) => println!("{:?}", map.map),
            Err(why) => println!("An error occurred: {}", why),
        }
    }

    Ok(())
}
