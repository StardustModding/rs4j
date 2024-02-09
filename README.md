# rs4j

[![Crates.io Version](https://img.shields.io/crates/v/rs4j?style=for-the-badge)](https://crates.io/crates/rs4j)

A small, automatic, efficient, and easy-to-use Rust to Java bridge.

## Usage

`rs4j` works by using a custom language syntax to translate
into Rust and Java code.

A good way to use this is to make a build script and have it
auto-compile your code.

You can add the package to your build script by running:

```sh
cargo add rs4j --build
```

Here's an example `build.rs`:

```rust
use rs4j::build::BindgenConfig;
use anyhow::Result;

fn main() -> Result<()> {
    // Make a new config
    BindgenConfig::new()

        // Set the package for export
        .package("your.package.here")

        // Where to save the Rust bindings
        .bindings(format!("{}/src/bindings.rs", env!("CARGO_MANIFEST_DIR")))

        // Where the input files are
        .glob(format!("{}/bindings/**/*.rs4j", env!("CARGO_MANIFEST_DIR")))?

        // Where to save java classes (is a directory)
        .output(format!("{}/java", env!("CARGO_MANIFEST_DIR")))

        // Go!
        .generate()?;

    Ok(())
}
```

Then, once that's done, use your `lib.rs` (or other file) to include
the generated bindings:

```rust
// Put any imports that are needed for the bindings here.
use path::to::Dependency;

// You can even define structs in the file, just before the bindings!

// Include the generated code.
include!("bindings.rs");
```

## Syntax

The syntax of the binding language is fairly simple. You start
by defining a class, then filling out its methods and items.

The basic syntax run-down is:

```rs4j
// This class, Thing, takes in one type parameter, `A`.
// You can omit this if it doesn't take any type parameters.
class Thing<A> {
    // This makes it so that Rust knows that the type for `A`
    // will have `Clone + Copy`. This doesn't change anything
    // on the Java side, it's just so that Rust will compile.
    bound A: Clone + Copy;

    // Here, the Rust function's name is `new`, but in Java that's
    // illegal as it is a reserved keyword. To combat this, you can
    // specify a different name for the function in Java and the real
    // one in Rust.
    [new] static fn of(value: A) -> Thing;

    // This gets the value. Since this is in snake_case, rs4j will
    // automatically convert it into camelCase, renaming this to
    // `get_value` on the Java side.
    fn get_value() -> A;

    // This marks this function as mutable, meaning in Rust it will
    // mutate the struct, as if it took a `&mut self` as an argument.
    mut fn set_value(value: A);

    // You can even include trait methods, as long as Rust can find the
    // trait it belongs to!
    fn clone() -> A;
};
```

## Support

The following primitive* types are supported:

| Rust     | Java      |
| -------- | --------- |
| `String` | `String`  |
| `str`    | `String`  |
| `bool`   | `Boolean` |
| `u8`     | `Byte`    |
| `u16`    | `Short`   |
| `u32`    | `Integer` |
| `u64`    | `Long`    |
| `i8`     | `Byte`    |
| `i16`    | `Short`   |
| `i32`    | `Integer` |
| `i64`    | `Long`    |
| `f32`    | `Float`   |
| `f64`    | `Double`  |
| `()`     | `Void`    |

\* Not all are primitive, but I consider them to be.
