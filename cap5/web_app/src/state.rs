use std::{fs, io::Read};

use serde_json::{json, Map, Value};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = fs::File::open(file_name.to_string()).unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = serde_json::from_str::<Value>(&data).unwrap();
    let state = json.as_object().unwrap().clone();

    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state).to_string();
    fs::write(file_name.to_string(), new_data).expect("Unable to write file");
}
