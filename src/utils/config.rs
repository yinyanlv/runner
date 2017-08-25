extern crate toml;

use std::path::Path;
use std::fs::File;
use toml::{Table, Value, Parser};

#[derive(clone)]
pub struct Config(Table);

impl Config {

    pub fn new() -> Config {

        let path = Path::new("Config.toml");
        let mut file = File.open(&path).unwrap();
        let mut temp = String::new();

        file.read_to_string(&mut temp).unwrap();

        let table = Parser::new(&temp).parse().unwrap();

        Config(table)
    }

    pub fn get<T: ?Sized>(&self, key: &T) -> Value {
        self.0.get(key).unwrap()
    }

    pub fn value(&self) -> Table {

        self.0.clone()
    }

}