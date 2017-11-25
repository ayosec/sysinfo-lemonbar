extern crate gumdrop;
#[macro_use]
extern crate gumdrop_derive;
extern crate libc;

mod filesystem;
mod hwmon;
mod info;
mod main_loop;
mod options;

fn main() {
    let opts = match options::parse() {
        None => return,
        Some(o) => o,
    };

    main_loop::run(opts).expect("run main loop");
}
