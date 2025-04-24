// use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};


struct Database {
    store : BTreeMap<String, String>,
    filename : String,
    indexer : String
}

impl Database {
    fn new(filename : &str , indexer : &str) -> Self {
        let mut store = BTreeMap::new();
        let file = File::open(filename).unwrap_or_else(|_| File::create(filename).unwrap());
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let parts : Vec<&str> = entry.split(',').collect();
                if parts.len() == 2 {
                    store.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }

        Database {
            store,
            filename: filename.to_string(),
            indexer: indexer.to_string()
        }

    }
    fn insert (&mut self, key: String, value: String) {
        self.store.insert(key.clone(), value.clone());
        self.insert_sorted_on_disk(&key, &value);
    }

    fn insert_sorted_on_disk(&self, key: String, value: String) {
        let file = std::fs::read_to_string(&self.filename).unwrap_or_default();
        let mut lines: Vec<_> = file.lines().map(|l| l.to_string()).collect();
    
        // Insert in sorted order
        match lines.binary_search_by(|line| {
            let k = line.split('=').next().unwrap_or("");
            k.cmp(&key)
        }) {
            Ok(pos) => lines[pos] = format!("{}={}", key, value), // Overwrite
            Err(pos) => lines.insert(pos, format!("{}={}", key, value)), // Insert
        }
    
        // Rewrite file
        let mut file = File::create(&self.filename).unwrap();
        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }
    }
    

    fn get (&self,key: String) -> Option<&String> {
        self.store.get(&key)
    }
    fn remove (&mut self, key: String) {
        self.store.remove(&key);
        self.persist_to_file();
    }
    // fn update (&mut self, key: String, value: String) {
    //     if let Some(v) = self.store.get_mut(&key) {
    //         *v = value;
    //     }
    // }
    fn append_to_file (&self, key: &String, value: &String) {
        let mut file = OpenOptions::new().append(true).open(&self.filename).unwrap();
        let mut indexer = OpenOptions::new().append(true).open(&self.indexer).unwrap();
        writeln!(file, "{},{}", key, value).unwrap();
        writeln!(indexer, "{},{}", key, value).unwrap();
    }

    fn persist_to_file(&self) {
        let mut file = File::create(&self.filename).unwrap();
        for (key, value) in &self.store {
            writeln!(file, "{},{}", key, value).unwrap();
        }
    }

    // fn clear (&mut self) {
    //     self.store.clear();

    // }
    // fn contains_key (&self, key: String) -> bool {
    //     self.store.contains_key(&key)
    // }
}

fn main () {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> put <key> <value>", args[0]);
        std::process::exit(1);
    }

    let mut db = Database::new("database.log");

    match args[1].as_str() {
        "put" => {
            if args.len() != 4 {
                eprintln!("Usage: {} put <key> <value>", args[0]);
                std::process::exit(1);
            }
            let key = args[2].clone();
            let value = args[3].clone();
            db.insert(key, value);
        }
        "get" => {
            if args.len() != 3 {
                eprintln!("Usage: {} get <key>", args[0]);
                std::process::exit(1);
            }
            let key = args[2].clone();
            match db.get(key) {
                Some(value) => println!("Found: {}", value),
                None => println!("Not found"),
            }
        }
        "remove" => {
            if args.len() != 3 {
                eprintln!("Usage: {} remove <key>", args[0]);
                std::process::exit(1);
            }
            let key = args[2].clone();
            db.remove(key);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}
