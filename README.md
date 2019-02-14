# Output-VT100

This simple crates allows developers to enable ANSI escape characters in Windows' console, be it CMD or PowerShell. Its usage is very simple, as shown below:

```rust
extern crate output_vt100;

fn main() {
    output_vt100::init();
    println!("\x1b[31mThis text is red!\x1b[0m");
}
```

If you wish to ensure the `output_vt100::init()` function is only ran once, you can use the crate [ctor](https://crates.io/crates/ctor). Be aware though it might not be suited for every use case, as explained on the crate’s presentation.

```
extern crate output_vt100;
extern crate ctor;
use ctor::*;

#[ctor]
fn init_term() {
    output_vt100::init();
}

fn main() {
    println!("\x1b[31mThis text is red!\x1b[0m");
}
```

# Acknowledgements

A big thank you to [nbouteme](https://github.com/nbouteme) who helped me a lot during the development of this create.
