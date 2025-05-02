
// Functions in Rust are similar to functions in other programming languages.
// functions are defined by the `fn` keyword, the type of each parameter in the function
// must be specified as demonstrated below.
// Additionally, the return type must be specified within the declaration of the function.

fn my_first_rust_function(x: i32, y: i32) -> i32 {
    // The function returns the sum of x and y
    x + y
    // NOTE: The return type is inferred from the last expression in the function
    // but we can include the return keyword if we would like to make it implicit
}

// Template functions are also possible in Rust, as with other programming languages.
// They allow for the creation of functions which don't utilize an implicit data type.
fn template_function1<T>(x: T,y: f32) -> T
where T: std::ops::Mul<f32, Output = T>, {
    // The following is not a valid line for this function
    // It implies that the function will return a value of type T
    // The constraint dictated by the where clause states that T must implement the Mul trait
    // This means that T must be a type that can be multiplied by a f32
    // y * x

    // The following line is valid and will return the multiple of T
    x * y
}

fn main() {

    // Calling the functions within the main function in main.rs to execute them.
    let sum = my_first_rust_function(5, 9);
    println!("The return value is: {}",sum);

    // Calling the template function
    //The where clause
    let product = template_function1(5.0, 9.0);
    println!("The return value is: {}",product);

}
