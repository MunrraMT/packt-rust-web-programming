use todo_app::{enums::TaskStatus, to_do_factory};

mod todo_app;

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::DONE);

    match to_do_item {
        todo_app::ItemTypes::Done(item) => {
            println!("{}", item.super_struct.title);
            println!("{}", item.super_struct.status.stringify());
        }

        todo_app::ItemTypes::Pending(item) => {
            println!("{}", item.super_struct.title);
            println!("{}", item.super_struct.status);
        }
    }
}
