use serde_json::{json, Map, Value};

use crate::{state::write_to_file, to_do::enums::TaskStatus};

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::DONE.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to done\n\n", title);
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::PENDING.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
}
