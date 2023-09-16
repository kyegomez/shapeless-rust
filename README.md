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
shapeless = "0.1.1"
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

# Documentation

Shapeless Trait
---------------

The `Shapeless` trait is the core of the Shapeless crate. It can be implemented for any type and provides methods for dynamic typing.

```rust
pub trait Shapeless: Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
}
```



The `as_any` method returns a reference to the `Any` trait, which allows for dynamic typing. The `type_name` method returns the name of the type as a string.

Wrapper Function
----------------

The `wrapper` function accepts any type that implements the `Shapeless` trait and prints it using the `Debug` trait.

```rust
pub fn wrapper<T: Shapeless>(arg: T) {
    println!("{:?}", arg);
}
```



Type Checking
-------------

The `is_type` function checks if a `Shapeless` object is of a certain type.

```rust
pub fn is_type<T: Shapeless>(arg: &dyn Any) -> bool {
    arg.as_any().is::<T>()
}
```



Type Casting
------------

The `downcast_ref` function attempts to downcast a `Shapeless` object to a specific type.

```rust
pub fn downcast_ref<T: Shapeless>(arg: &dyn Shapeless) -> Option<&T> {
    arg.as_any().downcast_ref::<T>()
}
```



Type Conversion
---------------

The `convert` function attempts to convert a `Shapeless` object to another type if possible.

```rust
pub fn convert<T: Shapeless, U: Shapeless>(arg: &T) -> Result<&U, &'static str> {
    if let Some(val) = arg.as_any().downcast_ref::<U>() {
        Ok(val)
    } else {
        Err("Conversion failed")
    }
}
```



Tutorial
========

Here's a step-by-step guide on how to use the Shapeless crate.

1.  First, add the Shapeless crate to your `Cargo.toml` file:

```toml
[dependencies]
shapeless = "0.1.0"
```



1.  Next, import the Shapeless crate in your Rust file:

```rust
use shapeless::{Shapeless, wrapper, is_type, downcast_ref};
```



1.  Define a struct and implement the `Debug` trait for it:

```rust
struct MyStruct {
    value: i32,
}

impl std::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct {{ value: {} }}", self.value)
    }
}
```



1.  Implement the `Shapeless` trait for your struct:

```rust
impl Shapeless for MyStruct {}
```



1.  Now you can use the `wrapper`, `is_type`, and `downcast_ref` functions with instances of your struct:

```rust
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



That's it! You've now used the main features of the Shapeless crate.

---

## Contributing

We welcome contributions to Shapeless! Please feel free to open an issue or submit a pull request if you have any issues or improvements.

---

## License

Shapeless is licensed under the MIT license. Please see the `LICENSE` file for more details.