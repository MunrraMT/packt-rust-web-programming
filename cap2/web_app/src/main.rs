use to_do::{
    enums::TaskStatus,
    to_do_factory,
    traits::{delete::Delete, edit::Edit, get::Get},
};

mod to_do;

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::DONE);

    match to_do_item {
        to_do::ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }

        to_do::ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }
}
