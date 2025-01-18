//! # What is this library
//! Tamanegi-Error is a library that helps you create error types that can track the propagation history.
//!
//! See README.md for more details.
//! (Only the Japanese version is available. Please wait for English version or send PR)
//!
//! # How to use
//!
//! Please add `TamanegiError` to the error struct made by Snafu as follows:
//!
//! ```rust
//! use snafu::{GenerateImplicitData, Snafu};
//! use tamanegi_error::{TamanegiError, location::StaticLocationRef};
//!
//! #[derive(Snafu, TamanegiError)]
//! pub struct ErrorSubA {
//!     #[snafu(implicit)]
//!     location: StaticLocationRef,
//! }
//!
//! impl ErrorSubA {
//!     pub fn new() -> Self {
//!         Self {
//!             location: StaticLocationRef::generate(),
//!         }
//!     }
//! }
//!
//! #[derive(Snafu, TamanegiError)]
//! #[snafu(context(false))]
//! struct MyError {
//!     #[snafu(source)]
//!     source: ErrorSubA,
//!     #[snafu(implicit)]
//!     location: StaticLocationRef,
//! }
//!
//! fn err() -> Result<(), MyError> {
//!     let err: Result<(), ErrorSubA> = Err(ErrorSubA::new());
//!     let _ = err?;
//!     Ok(())
//! }
//!
//! fn main() {
//!     if let Err(e) = err() {
//!         println!("{:?}", e);
//!     }
//! }
//! ```
//!
//! Then the following propagation history can be seen:
//!
//! ```text
//! 1: MyError, at examples/basic_struct.rs:29:13
//! 0: ErrorSubA, at examples/basic_struct.rs:13:23
//! ```
//!
//! # Relation to [Stack Error](https://greptime.com/blogs/2024-05-07-error-rust#how-error-looks-like-with-virtual-user-stack)
//!
//! This library is based on [Stack Error](https://greptime.com/blogs/2024-05-07-error-rust#how-error-looks-like-with-virtual-user-stack)'s idea.
//! I initially intended to use StackError as is because it is a great idea, but it was not suitable for no_std.
//! So I write this crate from scratch suitable for no_std, using only the idea as a reference.
#![cfg_attr(not(test), no_std)]

pub mod location;

pub use tamanegi_error_impl::TamanegiError;

/// This is a marker trait that indicates the implementation of Tamanegi-Error.
/// It is implemented by the [tamanegi_error_impl::TamanegiError] derive macro.
pub trait TamanegiTrait: snafu::ErrorCompat {}
