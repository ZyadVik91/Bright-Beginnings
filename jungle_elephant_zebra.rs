fn main() {
    // Section 1: Variables and Types
    let mut favorite_number: i32 = 42;  // initialize a signed 32-bit integer.
    let pi: f32 = 3.14;  // initialize a 32-bit float.
    let is_awesome: bool = true;  // initialize a boolean.
    let name: &str = "Bright Beginnings";  // initialize a string reference.
    let color_list: [&str; 4] = ["red", "green", "blue", "yellow"];  // initialize an array of strings.

    // Section 2: Control Flow
    if favorite_number < 40 {  // evaluate an expression; if true, execute code in the block
        println!("The number is less than 40!");  // print a message with the println! macro.
    } else {  // execute code in this block if the expression was false.
        println!("The number is greater than or equal to 40!");
    }

    // Section 3: Loops
    let mut i = 0;  // declare a mutable variable
    while i < 5 {  // evaluate an expression; if true, execute code in the block
        println!("{}", i);  // print the value of i
        i += 1;  // increment i by 1
    }

    // Section 4: Functions
    fn greet(name: &str) {  // declare a function with parameters
        println!("Hello, {}!", name);  // print a message with the println! macro
    }
    greet(name);  // call the function

    // Section 5: Structs
    #[derive(Debug)]  // enable printing of structs with the println! macro
    struct Car {
        make: String,  // declare a String field
        year: i32,  // declare a 32-bit integer field
    }
    let my_car = Car {  // create a Car struct
        make: String::from("Honda"),  // assign values to the fields
        year: 2012,
    };
    println!("My car is a {:?}.", my_car);  // print the struct

    // Section 6: Traits
    trait Fruit {  // declare a trait
        fn get_name(&self) -> String;  // declare a trait method
    }
    struct Apple {  // declare a struct
        color: String,  // declare a String field
    }
    impl Fruit for Apple {  // implement the trait for Apple
        fn get_name(&self) -> String {  // implement the trait method
            String::from("Apple")  // return the name of the fruit
        }
    }
    let mut apple = Apple {  // instantiate an Apple
        color: String::from("red"),
    };
    println!("This fruit is an {}.", apple.get_name());  // print the name of the fruit
}