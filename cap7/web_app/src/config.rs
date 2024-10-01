pub struct Config {
    pub map: std::collections::HashMap<String, serde_yaml::Value>,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let file_path = &args[args.len() - 1];
        let file = std::fs::File::open(file_path).unwrap();
        let map = serde_yaml::from_reader(file).unwrap();

        return Self { map };
    }
}