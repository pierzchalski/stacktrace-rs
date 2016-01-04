#[macro_use]
extern crate stacktrace;

fn main() {
    println!("Hello, world!");
    println!("{}", trace!());
}
