#[macro_use]
extern crate stacktrace;

use stacktrace::Trace;
use stacktrace::StackInfo;

fn test1<A>(thing: A) -> Result<A, Trace<String>> {
    let a = try!(test2(thing));
    Ok(a)
}

fn test2<A>(thing: A) -> Result<A, Trace<String>> {
    Err(Trace::new("hello, world!".to_owned()))
}

fn main() {
    println!("Hello, world!");
    println!("{}", StackInfo::new());
    println!("{}", test1("yolo").err().unwrap());
    test1("swagger").unwrap();
}
