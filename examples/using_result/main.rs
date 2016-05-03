#[macro_use] extern crate stacktrace;

use std::fmt::Debug;

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

trace! {
    ErrorX => ErrorY,
    ErrorY => ErrorZ,
}

fn test1(n: usize) -> Result<String, ErrorX> {
    if n >= 10 {
        return Err(ErrorX(format!("{} is too big!", n)))
    }
    Ok(format!("{} is small enough.", n))
}

fn test2(n: usize) -> Result<String, Trace<ErrorY>> {
    // uses the generated "From<ErrorX> for Trace<ErrorY>"
    let result = try!(test1(n));
    println!("Test 2 got result {:?}", result);
    Ok(result)
}

fn test3(n: usize) -> Result<String, Trace<ErrorZ>> {
    // uses the generated "From<Trace<ErrorY>> for Trace<ErrorZ>"
    let result = try!(test2(n));
    println!("Test 3 got result {:?}", result);
    Ok(result)
}

fn main() {
    println!("Success:\n{:?}\n", test3(1));
    println!("Failure:\n{:?}\n", test3(10));
}
