//! `μfmt`, a (6-40x) smaller, (2-9x) faster and panic-free alternative to [`core::fmt`].
//!
//! # Design goals
//! From highest priority to lowest priority:
//! - Optimized for binary size and speed (rather than for compilation time).
//! - No dynamic dispatch in generated code.
//! - No panicking branches in generated code, when optimized.
//! - No recursion where possible.
//!
//! # Features
//! - [`Debug`](core::fmt::Debug) and [`Display`](core::fmt::Display)-like traits.
//! - [`core::write!`]-like macro.
//! - A generic [`Formatter<'_, impl UWrite>`] instead of a single [`core::fmt::Formatter`]; the
//! [`UWrite`] trait has an associated error type so each writer can choose its error type. For
//! example, the implementation for Rust's standard heap-allocated `String` uses
//! [`Infallible`](`core::convert::Infallible`) as its error type.
//! - [`Formatter::debug_struct`](core::fmt::Formatter::debug_struct)-like API.
//! - [`UDebug`] derivation macro.
//! - Pretty formatting (e.g. `{:#?}`) for [`UDebug`].
//! 
//! Hexadecimal formatting (e.g. `{:x}`) of integer primitives (e.g. [`i32`]) currently cannot be
//! extended to other types.
//!
//! # Non-features
//! These are out of scope:
//! - Padding, alignment and other formatting options.
//! - Formatting floating point numbers.
//!
//! # Examples
//! - `uwrite!` / `uwriteln!`
//! With the `std` feature enabled:
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use ufmt_b::{derive::UDebug, uwrite};
//!
//! #[derive(UDebug)]
//! struct Pair { x: u32, y: u32 }
//!
//! let mut s = String::new();
//! let pair = Pair { x: 1, y: 2 };
//! uwrite!(s, "{:?}", pair).unwrap();
//! assert_eq!(s, "Pair { x: 1, y: 2 }");
//! ```
//!
//! - Hexadecimal formatting
//! Lowercase (`{:x}`), uppercase (`{:X}`), `0x`-prefix (`{:#x}`) and padding (`{:02x}`) are
//! supported on primitive integer types.
//! With the `std` feature enabled:
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use ufmt_b::uwrite;
//!
//! let mut s = String::new();
//! uwrite!(s, "{:#06x}", 0x42);
//! assert_eq!(s, "0x0042");
//! ```
//!
//! - Implementing [`UWrite`].
//! When implementing [`UWrite`], you should prefer the [`ufmt_write`] crate over the re-export for
//! better forward compatibility.
//! ```
//! use core::convert::Infallible;
//!
//! use ufmt_write::UWrite;
//!
//! struct MyWriter;
//!
//! impl UWrite for MyWriter {
//!     type Error = Infallible;
//!
//!     fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
//!         // ..
//!         Ok(())
//!     }
//! }
//! ```
//!
//! - Writing a `macro_rules!` macro that uses [`uwrite!`] (or [`uwriteln!`]).
//! ```
//! // like `std::format!` it returns a `std::String` but uses `uwrite!` instead of `write!`
//! macro_rules! uformat {
//!     // IMPORTANT use `tt` fragments instead of `expr` fragments (i.e. `$($exprs:expr),*`)
//!     ($($tt:tt)*) => {{
//!         let mut s = String::new();
//!         match ufmt_b::uwrite!(&mut s, $($tt)*) {
//!             Ok(_) => Ok(s),
//!             Err(e) => Err(e),
//!         }
//!     }}
//! }
//! ```
//!
//! # Benchmarks
//! The benchmarks ran on a ARM Cortex-M3 chip (`thumbv7m-none-eabi`).
//!
//! The benchmarks were compiled with `nightly-2019-05-01`, `-C opt-level=3`, `lto = true`,
//! `codegen-units = 1`.
//!
//! In all benchmarks `x = i32::MIN` and `y = i32::MIN` plus some obfuscation was applied to
//! prevent const-propagation of the `*write!` arguments.
//!
//! The unit of time is one core clock cycle: 125 ns (8 MHz)
//!
//! The `.text` and `.rodata` columns indicate the delta (in bytes) when commenting out the
//! `*write!` statement.
//!
//! |Code                                      |Time|%        |`.text`|%        |`.rodata`|%       |
//! |------------------------------------------|----|---------|-------|---------|---------|--------|
//! |`write!("Hello, world!")`                 |154 |~        |1906   |~        |248      |~       |
//! |`uwrite!("Hello, world!")`                |20  |**13.0%**|34     |**1.8%** |16       |**6.5%**|
//! |`write!(w, "{}", 0i32)`                   |256 |~        |1958   |~        |232      |~       |
//! |`uwrite!(w, "{}", 0i32)`                  |37  |**14.5%**|288    |**14.7%**|0        |**0%**  |
//! |`write!(w, "{}", x)`                      |381 |~        |
//! |`uwrite!(w, "{}", x)`                     |295 |77.4%    |
//! |`write!(w, "{:?}", Pair { x: 0, y: 0 })`  |996 |~        |4704   |~        |312      |~       |
//! |`uwrite!(w, "{:?}", Pair { x: 0, y: 0 })` |254 |**25.5%**|752    |**16.0%**|24       |**7.7%**|
//! |`write!(w, "{:?}", Pair { x, y })`        |1264|~        |
//! |`uwrite!(w, "{:?}", Pair { x, y })`       |776 |61.4%    |
//! |`write!(w, "{:#?}", Pair { x: 0, y: 0 })` |2853|~        |4710   |~        |348      |~       |
//! |`uwrite!(w, "{:#?}", Pair { x: 0, y: 0 })`|301 |**10.6%**|754    |**16.0%**|24       |**6.9%**|
//! |`write!(w, "{:#?}", Pair { x, y })`       |3693|~        |
//! |`uwrite!(w, "{:#?}", Pair { x, y })`      |823 |**22.3%**|
//!
//!
//! Benchmark program:
//! ``` ignore
//! static X: AtomicI32 = AtomicI32::new(i32::MIN); // or `0`
//! static Y: AtomicI32 = AtomicI32::new(i32::MIN); // or `0`
//!
//! #[exception]
//! fn PendSV() {
//!    // read DWT.CYCCNT here
//!
//!    let x = X.load(Ordering::Relaxed);
//!    let y = Y.load(Ordering::Relaxed);
//!
//!    let p = Pair { x, y };
//!
//!    uwrite!(&mut W, "{:#?}", p).ok();
//!
//!    // write!(&mut W, "{:#?}", p).ok();
//!
//!    asm::bkpt(); // read DWT.CYCCNT here
//! }
//! ```
//!
//! Writer used in the benchmarks:
//! ```
//! use core::{convert::Infallible, fmt, ptr};
//!
//! use ufmt_b::UWrite;
//!
//! struct W;
//!
//! impl UWrite for W {
//!     type Error = Infallible;
//!
//!     fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
//!         s.as_bytes()
//!             .iter()
//!             .for_each(|b| unsafe { drop(ptr::read_volatile(b)) });
//!
//!         Ok(())
//!     }
//! }
//!
//! impl fmt::Write for W {
//!     fn write_str(&mut self, s: &str) -> fmt::Result {
//!         s.as_bytes()
//!             .iter()
//!             .for_each(|b| unsafe { drop(ptr::read_volatile(b)) });
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! # Minimum Supported Rust Version (MSRV)
//! This crate does *not* have a Minimum Supported Rust Version (MSRV) and may make use of language
//! features and API in the standard library available in the latest stable Rust version.
//!
//! In other words, changes in the Rust version requirement of this crate are not considered semver
//! breaking change and may occur in patch version release.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(warnings)]

// this lets us use `uwrite!` in the test suite
#[allow(unused_extern_crates)]
#[cfg(test)]
extern crate self as ufmt;

use core::str;

pub use ufmt_write::UWrite;

/// Write formatted data into a buffer.
///
/// This macro accepts a format string, a list of arguments, and a 'writer'. Arguments will be
/// formatted according to the specified format string and the result will be passed to the writer.
/// The writer must have type `[&mut] impl UWrite` or `[&mut] ufmt_b::Formatter<'_, impl UWrite>`. The
/// macro returns the associated `Error` type of the `UWrite`-r.
///
/// The syntax is similar to [`core::write!`] but only a handful of argument types are accepted:
///
/// - `{}` - [`UDisplay`]
/// - `{:?}` - [`UDebug`]
/// - `{:#?}` - "pretty" [`UDebug`]
///
/// Named parameters and "specified" positional parameters (`{0}`) are not supported.
///
/// `{{` and `}}` can be used to escape braces.
pub use ufmt_macros::uwrite;

/// Write formatted data into a buffer, with a newline appended.
///
/// See [`uwrite!`] for more details.
pub use ufmt_macros::uwriteln;

pub use crate::helpers::{DebugList, DebugMap, DebugStruct, DebugTuple};

mod helpers;
mod impls;
/// Derive macros.
pub mod derive {
    pub use ufmt_macros::UDebug;
}

/// Trait that aims to behave like [`core::fmt::Debug`].
pub trait UDebug {
    /// Formats the value using the given formatter.
    fn fmt<W>(&self, _: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized;
}

/// Trait that aims to behave like [`core::fmt::Display`].
pub trait UDisplay {
    /// Formats the value using the given formatter.
    fn fmt<W>(&self, _: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized;
}

/// Options for formatting hexadecimal numbers.
/// 
/// This is currently an implementation detail and not subject to semver guarantees.
/// Do not use this outside the `ufmt` crate.
#[doc(hidden)]
pub struct HexOptions {
    /// when we need to use digits a-f, should they be upper case instead?
    pub upper_case: bool,
    /// when we are padding to a target length, what character should we pad using?
    pub pad_char: u8,
    /// when we are padding to a target length, how long should our string be?
    pub pad_length: usize,
    /// should we include a 0x prefix? (also controlled by upper_case)
    pub ox_prefix: bool,
}

impl HexOptions {
    /// Applies the various padding/prefix options while writing the `payload` string.
    pub fn with_stuff<W: UWrite + ?Sized>(
        &self,
        fmt: &mut Formatter<'_, W>,
        payload: &str,
    ) -> Result<(), <W as UWrite>::Error> {
        let pad_before = self.ox_prefix && self.pad_char == b' ';

        let pad = self.pad_length as isize
            - (if self.ox_prefix { 2 } else { 0 } + payload.len()) as isize;

        let do_pad = |fmt: &mut Formatter<'_, W>, pad: isize| -> Result<(), <W as UWrite>::Error> {
            if pad > 0 {
                for _ in 0..pad {
                    // miri considers the `write_char` defined in `ufmt-write` v0.1.0 unsound
                    // to workaround the issue we use `write_str` instead of `write_char`
                    fmt.write_str(unsafe { str::from_utf8_unchecked(&[self.pad_char]) })?;
                }
            }
            Ok(())
        };

        let do_prefix = |fmt: &mut Formatter<'_, W>,
                         go: bool,
                         upper_case: bool|
         -> Result<(), <W as UWrite>::Error> {
            if go {
                fmt.write_str(if upper_case { "0X" } else { "0x" })
            } else {
                Ok(())
            }
        };
        if pad_before {
            do_pad(fmt, pad)?;
            do_prefix(fmt, self.ox_prefix, self.upper_case)?;
        } else {
            do_prefix(fmt, self.ox_prefix, self.upper_case)?;
            do_pad(fmt, pad)?;
        }

        fmt.write_str(payload)
    }
}

/// Trait that aims to behave like [`core::fmt::LowerHex`].
/// 
/// This is currently an implementation detail and not subject to semver guarantees.
/// Do not use this outside the `ufmt` crate.
#[doc(hidden)]
pub trait UDisplayHex {
    /// Formats the value using the given formatter.
    fn fmt_hex<W>(&self, _: &mut Formatter<'_, W>, options: HexOptions) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized;
}

/// Configuration for formatting.
pub struct Formatter<'w, W>
where
    W: UWrite + ?Sized,
{
    indentation: u8,
    pretty: bool,
    writer: &'w mut W,
}

impl<'w, W> Formatter<'w, W>
where
    W: UWrite + ?Sized,
{
    /// Creates a formatter from the given writer.
    pub fn new(writer: &'w mut W) -> Self {
        Self {
            indentation: 0,
            pretty: false,
            writer,
        }
    }

    /// Executes the closure with pretty-printing enabled.
    pub fn pretty(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        let pretty = self.pretty;
        self.pretty = true;
        f(self)?;
        self.pretty = pretty;
        Ok(())
    }

    /// Writes a character to the underlying buffer contained within this formatter.
    pub fn write_char(&mut self, c: char) -> Result<(), W::Error> {
        self.writer.write_char(c)
    }

    /// Writes a string slice to the underlying buffer contained within this formatter.
    pub fn write_str(&mut self, s: &str) -> Result<(), W::Error> {
        self.writer.write_str(s)
    }

    /// Writes whitespace according to the current indentation level.
    fn indent(&mut self) -> Result<(), W::Error> {
        for _ in 0..self.indentation {
            self.write_str("    ")?;
        }

        Ok(())
    }
}

// Implementation detail of the `uwrite*!` macros.
#[doc(hidden)]
pub trait UnstableDoAsFormatter {
    type Writer: UWrite + ?Sized;

    fn do_as_formatter(
        &mut self,
        f: impl FnOnce(&mut Formatter<'_, Self::Writer>) -> Result<(), <Self::Writer as UWrite>::Error>,
    ) -> Result<(), <Self::Writer as UWrite>::Error>;
}

impl<W> UnstableDoAsFormatter for W
where
    W: UWrite + ?Sized,
{
    type Writer = W;

    fn do_as_formatter(
        &mut self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(&mut Formatter::new(self))
    }
}

impl<W> UnstableDoAsFormatter for Formatter<'_, W>
where
    W: UWrite + ?Sized,
{
    type Writer = W;

    fn do_as_formatter(
        &mut self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(self)
    }
}
