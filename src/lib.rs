//! A library for pretty-printing stack traces.
//!
//! Programmers typically find stack traces useful for debugging executables.
//! One way to get a trace for a thread is by panicking with the environment variable
//! `RUST_BACKTRACE=1`.
//! This is unfortunately hard to use.
//!
//! Alex Crichton's [backtrace](https://crates.io/crates/backtrace/) crate
//! implements the work required for actually getting information about the call stack on a variety
//! of platforms.
//! [Stacktrace](https://crates.io/crates/stacktrace/)
//! tries to make that information more ergonomic to acquire and use.
//!
//! # Quick Start
//! * For getting a stack trace with a pretty-printing instance of `std::fmt::Debug`, see
//!   [`StackInfo`](struct.StackInfo.html).
//! * For utilities for working with `std::result::Result`, see
//!   [`Trace`](struct.Trace.html).
//!
//! # Build profiles
//! For release builds, consider enabling debugging symbols if you want to keep useful
//! stack trace information available. To do so, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [profile.release]
//! debug = true
//! ```
//!
//! For more information, see [here](http://doc.crates.io/manifest.html#the-[profile.*]-sections).
//!
//! # Notes
//! * All traces will have various calls into `backtrace` and `stacktrace` as their first few
//!   entries. The particular entries will depend on platform as well.
//! * The structs storing call stack information use heap-allocated `String`s.
//!   This may be relevant in environments where allocation might fail, such
//!   as some low-level projects.

extern crate backtrace;

use std::os::raw::c_void;
use std::fmt;
use std::ops::Deref;

/// An error 'annotated' with a stack trace.
#[derive(Clone, PartialEq, Eq)]
pub struct Trace<E> {
    pub err: E,
    pub stacktrace: StackInfo,
}

impl<E: fmt::Debug> fmt::Debug for Trace<E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "Trace {{\n"));
        try!(write!(fmt, "err: {:?}\n", self.err));
        try!(write!(fmt, "{:?}", self.stacktrace));
        write!(fmt, "}}")
    }
}

/// A stack trace. Contains a vector of function call frames, in call stack order.
///
/// # Example use
/// ```
/// use stacktrace::StackInfo;
///
/// fn layer1() -> StackInfo { StackInfo::new() }
///
/// fn layer2() -> StackInfo { layer1() }
///
/// fn layer3() -> StackInfo { layer2() }
///
/// // If debugging symbols are enabled, we should see something like the following:
/// //
/// // stack backtrace:
/// //    0 - 0xdeadbeef00000000 - backtrace::trace (/home/user/cargo/backtrace/something.rs:42)
/// //    1 - 0xdeadbeef0000abcd - stacktrace::StackInfo::new (/home/user/cargo/stacktrace/somet
/// // hing_else.rs:99)
/// //    2 - 0xdeadbeefcafebead - simple::layer1 (src/your_test_file.rs:30)
/// //    3 - 0xdeadbeefcafef00d - simple::layer2 (src/your_test_file.rs:32)
/// //    4 - 0xdeadbeeff00dcafe - simple::layer3 (src/your_test_file.rs:34)
/// //    5 - 0xdeadbeefbead1234 - simple::main (src/your_test_file.rs:51)
/// println!("{:?}", layer3());
/// ```
///
#[derive(Clone, PartialEq, Eq)]
pub struct StackInfo(pub Vec<FrameInfo>);

impl fmt::Debug for StackInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "stack backtrace:\n"));
        for (index, frame) in self.0.iter().enumerate() {
            try!(write!(fmt, "{:4} - {:?}\n", index, frame));
        }
        Ok(())
    }
}

/// Information on a function call frame, for use in a stack trace.
///
/// The `Debug` implementation for `FrameInfo` prints the contents of the struct as follows:
///
/// `<instruction pointer> - <demangled function name> (<file name>:<line number>)`
///
/// The file name and line are each replaced with `"<unknown>"` if they are not available.
#[derive(Clone, PartialEq, Eq)]
pub struct FrameInfo {
    /// Instruction pointer of the call frame
    pub ip: *mut c_void,
    /// Name of the function, demangled
    pub name: Option<String>,
    pub addr: Option<*mut c_void>,
    /// Name and location of the source file for the function definition
    pub filename: Option<String>,
    /// Line number of the source file at the execution point when the trace was made
    pub lineno: Option<u32>,
}

impl fmt::Debug for FrameInfo {
    #[cfg(target_pointer_width = "32")]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        let filename = self.filename.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        let lineno = self.lineno.map(|d| format!("{}", d));
        let lineno = lineno.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        try!(write!(fmt, "{:010p} - {} ({}:{})", self.ip, name, filename, lineno));
        Ok(())
    }
    #[cfg(target_pointer_width = "64")]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        let filename = self.filename.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        let lineno = self.lineno.map(|d| format!("{}", d));
        let lineno = lineno.as_ref().map(Deref::deref).unwrap_or("<unknown>");
        try!(write!(fmt, "{:018p} - {} ({}:{})", self.ip, name, filename, lineno));
        Ok(())
    }
}

impl<E> Trace<E> {
    /// Construct a new `Trace` struct, pairing the provided value with a new stack trace.
    pub fn new(e: E) -> Self {
        Trace {
            err: e,
            stacktrace: StackInfo::new(),
        }
    }
    /// Convert a `Result<A, E>` to a `Result<A, Trace<E>>` by inserting a new stack trace.
    /// This also applys any relevant instances of
    /// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html).
    ///
    /// Use this if you want to trace the error now, but need to handle the trace and error later.
    ///
    /// # Example
    /// ```
    /// use stacktrace::Trace;
    ///
    /// struct MyError(String);
    ///
    /// fn error() -> Result<usize, MyError> {
    ///   Err(MyError("oh no!".to_owned()))
    /// }
    ///
    /// fn deal_with_it_later() -> Result<(), Trace<MyError>> {
    ///   // the returned stack trace will lead to this line in this function.
    ///   let num = try!(Trace::result(error()));
    ///   println!("Got result {}!", num);
    ///   Ok(())
    /// }
    ///
    /// ```
    ///
    pub fn result<A, F: From<E>>(res: Result<A, E>) -> Result<A, Trace<F>> {
        match res {
            Ok(a) => Ok(a),
            Err(e) => Err(Trace {
                err: From::from(e),
                stacktrace: StackInfo::new(),
            }),
        }
    }
    /// Apply any instances of [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
    /// to the wrapped error, while keeping the previous stack trace.
    ///
    /// Use this in combination with the `try!` macro to convert errors before returning them.
    ///
    /// # Example
    /// ```
    /// use stacktrace::Trace;
    ///
    /// struct ErrorA(String);
    ///
    /// struct ErrorB(ErrorA);
    /// impl From<ErrorA> for ErrorB {
    ///   fn from(err: ErrorA) -> ErrorB { ErrorB(err) }
    /// }
    ///
    /// fn error() -> Result<usize, Trace<ErrorA>> {
    ///   Err(Trace::new(ErrorA("oh no!".to_owned())))
    /// }
    ///
    /// fn deal_with_it_later() -> Result<(), Trace<ErrorB>> {
    ///   let num = try!(Trace::trace(error()));
    ///   println!("Got {}!", num);
    ///   Ok(())
    /// }
    /// ```
    ///
    /// # Note
    /// For the curious reader, the reason we can't derive a generic instance of
    /// `From<Trace<E>>` for `Trace<F>`
    /// is because it would conflict with the generic `impl From<T> for T` in `std`.
    pub fn trace<A, F: From<E>>(res: Result<A, Trace<E>>) -> Result<A, Trace<F>> {
        match res {
            Ok(a) => Ok(a),
            Err(Trace {
                err: e,
                stacktrace: st,
            }) => Err(Trace {
                err: From::from(e),
                stacktrace: st,
            }),
        }
    }
}

impl StackInfo {
    /// Construct a stack trace.
    ///
    /// There is a tradeoff between offering a macro for constructing a new trace, and
    /// providing a function. Using a macro removes a few unnecessary calls from the resulting
    /// stack, but usually results in unusable debugging symbols. We choose to only offer the
    /// function.
    pub fn new() -> Self {
        let mut frames = Vec::new();
        backtrace::trace(&mut |frame: &backtrace::Frame| {
            let ip = frame.ip();
            let mut symbol_handler = |symbol: &backtrace::Symbol| {
                let name = symbol.name()
                                 .and_then(|name_bytes| std::str::from_utf8(name_bytes).ok())
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
                                     .and_then(|name| std::str::from_utf8(name).ok())
                                     .map(|name| name.to_owned());
                let lineno = symbol.lineno();
                frames.push(FrameInfo {
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
        StackInfo(frames)
    }
}
