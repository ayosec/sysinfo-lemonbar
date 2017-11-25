extern crate libc;
extern crate gumdrop;
#[macro_use] extern crate gumdrop_derive;

mod options;
mod info;

fn main() {
    let opts = match options::parse() {
        None => return,
        Some(o) => o,
    };

    println!("{:#?}", opts);
    println!("{:#?}", info::load());

}
