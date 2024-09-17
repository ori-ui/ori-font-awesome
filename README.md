# Ori Font Awesome
[![Crates.io](https://img.shields.io/crates/v/ori-font-awesome)](https://crates.io/crates/ori-font-awesome)
[![Documentation](https://img.shields.io/docsrs/ori-font-awesome)](https://docs.rs/ori-font-awesome/latest)
[![license](https://img.shields.io/crates/l/ori-font-awesome)](https://github.com/ori-ui/ori-font-awesome/tree/main)

Font Awesome integration with [`Ori`](github.com/ori-ui/ori).

# Example
```rust
use ori::prelude::*;
use ori_font_awesome as fa;

fn ui() -> impl View {
    center(fa::icon("font-awesome").size(64.0))
}

fn main() {
    let window = Window::new().title("Ori Font Awesome example");

    ori::run_simple(window, ui).unwrap()
}
```
