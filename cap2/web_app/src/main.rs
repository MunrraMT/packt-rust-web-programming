use std::env;

use serde_json::json;

mod state;
mod to_do;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let status = &args[1];
    let title = &args[2];

    let mut state = state::read_file("./state.json");
    println!("Before operation: {:?}", state);

    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);

    state::write_to_file("./state.json", &mut state);
}
