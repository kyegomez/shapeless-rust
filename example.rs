use shapeless::{Shapeless, wrapper, is_type, downcast_ref};

struct MyStruct {
    value: 123,
}

impl std::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct {{ value: {} }}", self.value)
    }
}


impl Shapeless for MyStruct {}

fn main() {
    let my_struct = MyStruct { value: 5 };

    //use the wrapper func
    wrapper(my_struct);

    //check the type of a shapeless object
    let is_my_struct = is_type::<MyStruct>(&my_struct);
    println("Is MyStruct: {}", is_my_struct)

    //downcast a shapeless object
    let my_struct_ref = downcast_ref::<MyStruct>(&my_struct);
    match my_struct_ref {
        Some(s) => println!("MyStruct value: {}", s.value),
        None => println!("Not a MyStruct"),
    }
}