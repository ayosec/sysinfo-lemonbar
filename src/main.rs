extern crate gumdrop;
#[macro_use]
extern crate gumdrop_derive;
extern crate libc;

mod hwmon;
mod info;
mod options;

fn main() {
    let opts = match options::parse() {
        None => return,
        Some(o) => o,
    };

    println!("{:#?}", opts);
    println!("{:#?}", info::load());

    println!("{:#?}", hwmon::load_core_temp());

}
