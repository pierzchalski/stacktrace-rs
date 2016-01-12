#[macro_use]
extern crate stacktrace;

use std::fmt::Debug;
use stacktrace::Trace;

#[derive(Debug)]
struct ErrorX(String);
#[derive(Debug)]
struct ErrorY(ErrorX);
#[derive(Debug)]
struct ErrorZ(ErrorY);

impl From<ErrorX> for ErrorY {
    fn from(x: ErrorX) -> Self {
        ErrorY(x)
    }
}

impl From<ErrorY> for ErrorZ {
    fn from(y: ErrorY) -> Self {
        ErrorZ(y)
    }
}

fn test1<A: Debug>(a: A) -> Result<A, ErrorX> {
    let message = format!("An error: {:?}", a);
    if message.len() > "An error: ______".len() {
        return Err(ErrorX(message));
    }
    Ok(a)
}

fn test2<A: Debug>(a: A) -> Result<A, Trace<ErrorY>> {
    let result = try!(Trace::result(test1(a)));
    println!("Got result {:?}", result);
    Ok(result)
}

fn test3<A: Debug>(a: A) -> Result<A, Trace<ErrorZ>> {
    let result = try!(Trace::map(test2(a)));
    println!("Got result {:?}", result);
    Ok(result)
}

fn main() {
    println!("Success:\n{:?}\n", test3("yolo"));
    println!("Failure:\n{:?}\n", test3("swagger"));
}
