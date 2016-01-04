extern crate backtrace;

use std::os::raw::c_void;
use std::fmt;
use std::ops::Deref;

pub use backtrace as macro_backtrace;

#[macro_export]
macro_rules! trace {
    () => {{
    use $crate::macro_backtrace as backtrace;
    let mut frames = Vec::new();
    backtrace::trace(&mut |frame: &backtrace::Frame| {
        let ip = frame.ip();
        let mut symbol_handler = |symbol: &backtrace::Symbol| {
            let name = symbol.name()
                             .and_then(|name_bytes| ::std::str::from_utf8(name_bytes).ok())
                             .and_then(|name_str| {
                                 let mut demangled = String::new();
                                 if let Ok(_) = backtrace::demangle(&mut demangled, name_str) {
                                     Some(demangled)
                                 } else {
                                     None
                                 }
                             });
            let addr = symbol.addr();
            let filename = symbol.filename()
                                 .and_then(|name| ::std::str::from_utf8(name).ok())
                                 .map(|name: &str| name.to_owned());
            let lineno = symbol.lineno();
            frames.push($crate::FrameInfo {
                ip: ip,
                name: name,
                addr: addr,
                filename: filename,
                lineno: lineno,
            });
        };
        backtrace::resolve(ip, &mut symbol_handler);
        true
    });
    $crate::StackInfo(frames)
    }}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace<E> {
    pub err: E,
    pub stacktrace: StackInfo,
}

impl<E: fmt::Display> fmt::Display for Trace<E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}\n{}", self.err, self.stacktrace)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StackInfo(pub Vec<FrameInfo>);

impl fmt::Display for StackInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "stack backtrace:\n"));
        for (index, frame) in self.0.iter().enumerate() {
            try!(write!(fmt, "{:4} - {}\n", index, frame));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameInfo {
    /// Instruction pointer of the call frame
    pub ip: *mut c_void,
    /// Name of the function
    pub name: Option<String>,
    pub addr: Option<*mut c_void>,
    pub filename: Option<String>,
    pub lineno: Option<u32>,
}

impl fmt::Display for FrameInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        let filename = self.filename.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        let lineno = self.lineno.map(|d| format!("{}", d));
        let lineno = lineno.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        try!(write!(fmt, "{:14p} - {} ({}:{})", self.ip, name, filename, lineno));
        Ok(())
    }
}

impl<E> Trace<E> {
    /// Wraps the provided argument in a trace of the stack.
    /// This is probably only useful for returning errors up a long, unhandled
    /// `Result` chain, to give a rough idea for the location of the source of
    /// the error.
    pub fn wrap(e: E) -> Self {
        Trace {
            err: e,
            stacktrace: trace!(),
        }
    }
}

impl StackInfo {
    pub fn new() -> Self {
        trace!()
    }
}
