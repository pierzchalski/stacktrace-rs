#[macro_use]
extern crate stacktrace;

use std::fmt::Debug;
use stacktrace::Trace;

#[derive(Debug)]
struct ErrorX(String);
#[derive(Debug)]
struct ErrorY(ErrorX);

impl From<ErrorX> for ErrorY {
    fn from(x: ErrorX) -> Self {
        ErrorY(x)
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
    let result = try_trace!(test1(a));
    println!("Got result {:?}", result);
    Ok(result)
}

fn main() {
    println!("Success:\n{:?}\n", test2("yolo"));
    println!("FailureL\n{:?}\n", test2("swagger"));
}
