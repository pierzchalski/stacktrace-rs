//! This example has its 'release' build profile adjusted to
//! include debug symbols, so it can show useful debugging
//! information even in production (see 'Cargo.toml', under
//! the configuration of example "ex1").

#[macro_use]
extern crate stacktrace;

use stacktrace::Trace;
use stacktrace::StackInfo;

#[derive(Debug)]
struct MyError(String);

fn test1<A>(thing: A) -> Result<A, Trace<MyError>> {
    let a = try!(test2(thing));
    Ok(a)
}

fn test2<A>(thing: A) -> Result<A, Trace<MyError>> {
    Err(Trace::new(MyError("hello, world!".to_owned())))
}

fn test3(i: i32) -> Result<String, Trace<MyError>> {
    if i == 42 {
        Ok("got thing!".to_owned())
    } else {
        test1("yolo".to_owned())
    }
}

fn main() {
    println!("A basic trace:\n{:?}\n", StackInfo::new());
    println!("Going through 2 calls:\n{:?}\n", test1("yolo").err().unwrap());
    println!("Going through 3 calls:\n{:?}\n", test3(41));
    println!("Panicking:");
    test1(3).unwrap();
}
