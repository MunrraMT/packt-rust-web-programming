use std::{collections::HashMap, env, fs::File};

pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[&args.len() - 1];
        let file = File::open(file_path).unwrap();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();

        return Self { map };
    }
}
