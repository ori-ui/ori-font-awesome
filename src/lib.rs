//! This crate provides a simple way to use Font Awesome icons in your ['ori'] applications.
//!
//! # Example
//! ```rust
//! use ori::prelude::*;
//! use ori_font_awesome as fa;
//!
//! fn ui() -> impl View {
//!     center(fa::icon("font-awesome").size(64.0))
//! }
//!
//! fn main() {
//!     let window = Window::new().title("Ori Font Awesome example");
//!
//!     ori::run_simple(window, ui).unwrap()
//! }
//! ````

#![warn(missing_docs)]

mod icon;
mod kind;

pub use icon::*;
pub use kind::*;
