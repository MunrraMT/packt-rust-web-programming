use std::env;

use process::process_input;
use to_do::{enums::TaskStatus, to_do_factory};

mod process;
mod state;
mod to_do;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let command = &args[1];
    let title = &args[2];
    let state = state::read_file("./state.json");
    let status: String;

    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }

        None => {
            status = "pending".to_owned();
        }
    }

    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));

    process_input(item, command.to_string(), &state);
}
