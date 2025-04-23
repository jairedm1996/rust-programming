use std::collections::HashMap;

fn main() {

    // The let statement in Rust is used to declare variables
    // let x = 5;
    // println!("The value of x is: {}", x);
    // This will cause an error because `x` is immutable
    // In Rust by default there all variables are immutable unless otherwise stated
    // x = 6;
    // println!("The value of x is: {}", x);

    // When using the mutability changing of a datatype is not allowed. When we define a variable
    // as a String we can't later change it to a value which is not an string such as i32 or f32
    // let mut spaces = "  ";
    // spaces = spaces.len();

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 4;
    println!("The value of y is: {}", y);

    // Why is immutability thr default in Rust?

    //Safety: Immutability prevents accidental modifications to variables, reducing bugs related to unexpected state changes.
    //        When you see a variable, you can be confident its value won't change unless explicitly marked as mut.

    // Concurrency: Immutable data is inherently thread-safe. Multiple threads can safely read immutable data without synchronization mechanisms,
    //              making concurrent programming safer and more efficient.

    // Clear Intent: When you see mut, it's an explicit signal that the variable is designed to change.
    //               This makes code more readable and helps document the intended behavior:
    //               Adding the mut keyword to the let statement when defining the variable y allows for a variable to be mutable

    // Compiler Optimizations: The compiler can make better optimizations when it knows a value won't change, potentially leading to faster code execution.

    // Better Design: Having immutability as the default encourages you to think more carefully about your data model and when mutability is actually needed,
    //                leading to more robust program design.

    // Constants

    // Constants differ from variables in the fact that they are always immutable. Also, constants must have a type annotated
    // the underscore used here is optional but is used to improve readability
    const MAX_POINTS: u32 = 100_000;

    // Shadowing

    // Shadowing allows you to declare a new variable with the same name as a previous variable, effectively "hiding" the previous one.
    // This is similar to simply redefining a variable, but it preserves the immutability of the variable after we redefine it,
    // this would however open up for the potential for exploitation.
    let z = 5;
    let z = z + 1;
    println!("The shadowed value of z is: {}", z);


    // Data Types
    // All Rust variables have data types of which there are two categories:

    // Scalar Types: represent a single value. They include:

    // - Integers: signed and unsigned integers of various sizes (e.g., i32, u32, i64, u64).
    let int1 :i32 = -5;
    let int2 :u32 = 5;
    let int3 :i64 = -7;
    let int4 :u64 = 8;

    // - Floating-point numbers: represent decimal numbers (e.g., f32, f64).
    let float1 :f32 = 3.14;
    let float2 :f64 = 2.718281828459045;

    // - Booleans: represent true or false values (bool).
    let bool1 :bool = true;
    let bool2 :bool = false;

    // - Characters: represent a single Unicode character (char).
    let char1 :char = 'a';
    let char2 :char = 'Z';

    // Compound Types: can group multiple values into one type. They include:

    // - Tuples: fixed-size collections of different types (e.g., (i32, f64, char)).
    let tuple1 :(i32,f64,char) = (5,3.14,'d');

    // - Arrays: fixed-size collections of the same type (e.g., [i32; 5] for an array of five i32 integers).
    // The array is defined as data type i32 and will have a size of 5
    let arr1 = [1,2,3,4,5];

    // - Slices: dynamically sized views into arrays or vectors (e.g., &[i32] for a slice of i32 integers).
    // Slices in rust are a reference to a section of a collection, such as an array or vector.
    let numbers = [1, 2, 3, 4, 5];
    // references a slice from numbers ranging from index 0, inclusive, to index 3, exclusive
    let numbers_slice = &numbers[0..3];

    // - Strings: dynamically sized collections of characters (e.g., String for a growable string).
    let str1 = String::from("This is a rust string");

    // Strings can also be defined as a reference but in this case it will be immutable
    let str2 = "This is a rust string";

    // - Vectors: dynamically sized collections of the same type (e.g., Vec<i32> for a vector of i32 integers).
    let mut vec1 = vec![1, 2, 3, 4, 5];
    // This method appends new values to the vector
    vec1.push(6);
    // methods to remove values from a vector are pop(), remove(), and retain().

    // - HashMaps: collections of key-value pairs (e.g., HashMap<String, i32> for a map from strings to i32 integers).
    // Creating a new empty HashMap which uses Strings as the keys and floats as the values.
    let mut scores: HashMap<String,f64> = HashMap::new();

    // - Structs: custom data types that can group related data together (e.g., struct Point { x: i32, y: i32 }).
    // A struct is a data type which allows for you to group data together //
    struct StructExample {
        struct_string: String,
        struct_int: i32,
    }
    // We can create an object from the struct by doing the following
    let example = StructExample {
        struct_string: String::from("This is a string"),
        struct_int: 5,
    };
    // Dot operations allow for us to access the values within the struct
    println!("Struct string: {}", example.struct_string);
    println!("Struct int: {}", example.struct_int);

    // we can add methods to a struct by using the impl keyword
    impl StructExample {
        fn print_struct(&self) {
            println!("Struct string: {}", self.struct_string);
            println!("Struct int: {}", self.struct_int);
        }
    }
    example.print_struct();

}
