# Shapeless

Shapeless is a Rust crate that provides a way to work with shapeless and polymorphic types. It allows you to abstract over types and treat them in a uniform way, regardless of their actual type.

## Features
--------

-   Shapeless Trait: A trait that can be implemented for any type, providing methods for dynamic typing.
-   Wrapper Function: A function that accepts any type that implements the `Shapeless` trait.
-   Type Checking: A function to check if a `Shapeless` object is of a certain type.
-   Type Casting: A function to downcast a `Shapeless` object to a specific type.
-   Type Conversion: A function to convert a `Shapeless` object to another type if possible.

## Usage
-----

First, add the following to your `Cargo.toml`:

```
[dependencies]
shapeless = "0.1.0"
```

Then, you can use the `shapeless` crate in your code like this:

```rust
use shapeless::{Shapeless, wrapper, is_type, downcast_ref};

struct MyStruct {
    value: i32,
}

impl std::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct {{ value: {} }}", self.value)
    }
}

impl Shapeless for MyStruct {}

fn main() {
    let my_struct = MyStruct { value: 5 };

    // Use the wrapper function
    wrapper(my_struct);

    // Check the type of a Shapeless object
    let is_my_struct = is_type::<MyStruct>(&my_struct);
    println!("Is MyStruct: {}", is_my_struct);

    // Downcast a Shapeless object to a specific type
    let my_struct_ref = downcast_ref::<MyStruct>(&my_struct);
    match my_struct_ref {
        Some(s) => println!("MyStruct value: {}", s.value),
        None => println!("Not a MyStruct"),
    }
}
```

---

## Contributing

We welcome contributions to Shapeless! Please feel free to open an issue or submit a pull request if you have any issues or improvements.

---

## License

Shapeless is licensed under the MIT license. Please see the `LICENSE` file for more details.