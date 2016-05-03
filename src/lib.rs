//! A library for using stack traces with [`Result`](https://doc.rust-lang.org/std/result/index.html).
//!
//! Alex Crichton's excellent [backtrace](https://crates.io/crates/backtrace/) crate
//! does the work required for actually getting information about the call stack on a variety
//! of platforms.
//! [Stacktrace](https://crates.io/crates/stacktrace/)
//! tries to make that information more ergonomic to use.
//!
//! # Quick Start
//! In your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! stacktrace = "0.2"
//! ```
//!
//! In your project:
//!
//! ```
//! #[macro_use] extern crate stacktrace;
//!
//! trace!{}
//! # fn main() {}
//! ```
//!
//! # Example use
//! ```
//! #[macro_use] extern crate stacktrace;
//!
//! pub struct Error1(usize);
//! pub struct Error2(String);
//!
//! impl From<Error1> for Error2 {
//!     fn from(err: Error1) -> Error2 {
//!         Error2(format!("{}", err.0))
//!     }
//! }
//!
//! trace!{Error1 => Error2}
//!
//! fn makes_a_traced_error() -> Result<(), Trace<Error1>> {
//!     try!(Err(Error1(1337))); // Uses generic instance of "From<Err>" for "Trace<Err>"
//!     Ok(())
//! }
//!
//! fn propagates_a_traced_error() -> Result<(), Trace<Error2>> {
//!     try!(makes_a_traced_error()); // Uses the macro-generated instance of "From<Trace<Error1>>" for "Trace<Error2>"
//!     Ok(())
//! }
//! # fn main() {}
//! # mod macro_test {
//! #   trace!{struct NoSemicolon}
//! #   trace!{struct Semicolon;}
//! # }
//! ```
//!
//! See the section [Usage information](#usage_information) for more.
//!
//! <a name=usage_information></a>
//! # Usage information
//! This crate is intended for use with binary crates.
//! It provides a macro to define and use a `Trace` struct, which wraps errors with an associated
//! stacktrace. The macro also defines instances of [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
//! for use with the standard [`try!`](https://doc.rust-lang.org/std/macro.try!.html) macro.
//!
//! These trait implementations are the reason the `Trace` struct needs to be defined with a macro in the user's
//! crate, since two things prevent them being defined externally:
//!
//! * Because of the generic `impl<A> From<A> for A` in the standard library, we can't implement a
//!   generic `impl<A, B: From<A>> From<Trace<A>> for Trace<B>`, since `rustc` first sees this
//!   as `impl From<Trace<_>> for Trace<_>`.
//! * If `Trace` were defined in this crate, then users wouldn't be able to implement `From<A> for Trace<B>`
//!   because of the trait coherence rules.
//!
//! The call `trace!{StructName; A => B, C => D, ...}` will produce a struct `StructName<E>` with
//! the following implementations:
//!
//! * [`Deref<E>`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
//! * [`DerefMut<E>`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)
//! * `From<A> for StructName<B>`, which requires `From<A> for B`
//! * `From<C> for StructName<D>`, etc.
//! * `From<StructName<A>> for StructName<B>`, which also requires `From<A> for B`
//! * `From<StructName<C>> for StructName<D>`, etc.
//! * `Debug`, which will print the inner error then the stack trace in the same format as the one
//!   defined for [`Backtrace`](http://alexcrichton.com/backtrace-rs/backtrace/struct.Backtrace.html).
//!
//! If unspecified, `StructName` defaults to `Trace`.
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

pub extern crate backtrace;

/// Helper macro for defining a `Trace` struct, and instances of `From<Trace<B>>` for `Trace<A>`.
///
/// # Example use
/// ```
/// #[macro_use] extern crate stacktrace;
///
/// pub struct Error1(usize);
/// pub struct Error2(String);
///
/// impl From<Error1> for Error2 {
///     fn from(err: Error1) -> Error2 {
///         Error2(format!("{}", err.0))
///     }
/// }
///
/// trace!{Error1 => Error2}
///
/// fn makes_a_traced_error() -> Result<(), Trace<Error1>> {
///     try!(Err(Error1(1337))); // Uses generic instance of "From<Err>" for "Trace<Err>"
///     Ok(())
/// }
///
/// fn propagates_a_traced_error() -> Result<(), Trace<Error2>> {
///     try!(makes_a_traced_error()); // Uses the macro-generated instance of "From<Trace<Error1>>" for "Trace<Error2>"
///     Ok(())
/// }
/// # fn main() {}
/// ```
///
/// # Advanced use
/// The `trace` macro takes an optional initial 'name' parameter: `trace!{struct MyTrace}` will define
/// a struct named `Example` that behaves exactly like the default `Trace` struct.
///
/// ```
/// #[macro_use] extern crate stacktrace;
///
/// pub struct Error1(usize);
/// pub struct Error2(String);
///
/// impl From<Error1> for Error2 {
///     fn from(err: Error1) -> Error2 {
///         Error2(format!("{}", err.0))
///     }
/// }
///
/// trace!{struct MyTrace; Error1 => Error2}
///
/// fn makes_a_traced_error() -> Result<(), MyTrace<Error1>> {
///     try!(Err(Error1(1337))); // Uses generic instance of "From<Err>" for "MyTrace<Err>"
///     Ok(())
/// }
///
/// fn propagates_a_traced_error() -> Result<(), MyTrace<Error2>> {
///     try!(makes_a_traced_error()); // Uses the macro-generated instance of "From<MyTrace<Error1>>" for "MyTrace<Error2>"
///     Ok(())
/// }
/// # fn main() {}
/// # mod macro_test {
/// #   trace!{struct NoSemicolon}
/// #   trace!{struct Semicolon;}
/// #   struct Magic;
/// #   fn default_impl_works() -> Result<(), Semicolon<Magic>> {
/// #       try!(Err(Magic)); Ok(())
/// #   }
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    (struct $trace:ident; $($a:ty => $b:ty,)*) => {
        /// An error 'annotated' with a stack trace.
        pub struct $trace<E> {
            /// The current associated error. This may not be the initial error (for instance,
            /// if someone calls `map_error` on a result, or calls `From::from` to convert
            /// the error type).
            pub err: E,
            trace: $crate::backtrace::Backtrace,
        }

        impl<E> $trace<E> {
            /// Produces a new trace wrapping the provided error, including a stack trace.
            pub fn new(err: E) -> Self {
                $trace {
                    err: err,
                    trace: $crate::backtrace::Backtrace::new(),
                }
            }

            /// Provides a reference to the associated stack trace.
            pub fn trace(&self) -> &$crate::backtrace::Backtrace {
                &self.trace
            }
        }

        impl<E: ::std::fmt::Debug> ::std::fmt::Debug for $trace<E> {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use ::std::fmt::Write;
                try!(write!(fmt, "Trace {{\n"));
                try!(write!(fmt, "err: {:?}\n", self.err));
                try!(write!(fmt, "trace:\n{:?}", self.trace));
                try!(write!(fmt, "}}"));
                Ok(())
            }
        }

        impl<E> ::std::convert::From<E> for $trace<E> {
            fn from(err: E) -> Self {
                $trace {
                    err: err,
                    trace: $crate::backtrace::Backtrace::new(),
                }
            }
        }

        /// Allows a borrowed trace to be used as a borrowed error.
        impl<E> ::std::ops::Deref for $trace<E> {
            type Target = E;
            fn deref(&self) -> &E {
                &self.err
            }
        }

        /// Allows a mutably borrowed trace to be used as a mutably borrowed error.
        impl<E> ::std::ops::DerefMut for $trace<E> {
            fn deref_mut(&mut self) -> &mut E {
                &mut self.err
            }
        }

        $(impl ::std::convert::From<$a> for $trace<$b>
            where $b: ::std::convert::From<$a>
        {
            fn from(err: $a) -> $trace<$b> {
                $trace {
                    err: ::std::convert::From::from(err),
                    trace: $crate::backtrace::Backtrace::new(),
                }
            }
        })*

        $(impl ::std::convert::From<$trace<$a>> for $trace<$b>
            where $b: ::std::convert::From<$a>
        {
            fn from(trace: $trace<$a>) -> $trace<$b> {
                $trace {
                    err: ::std::convert::From::from(trace.err),
                    trace: trace.trace,
                }
            }
        })*
    };

    (struct $name:ident; $($a:ty => $b:ty),*) => {
        trace!{struct $name; $($a => $b,)*}
    };

    (struct $name:ident) => {
        trace!{struct $name;}
    };

    ($($a:ty => $b:ty,)*) => {
        trace!{struct Trace; $($a => $b,)*}
    };

    ($($a:ty => $b:ty),*) => {
        trace!{struct Trace; $($a => $b,)*}
    };
}
