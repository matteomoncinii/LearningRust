fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_args(5);
    print_labeled_measurement(5, 'h');

    let z = plus_one(5);
    let zr = plus_one_return(5);
    println!("Output from plus_one function is: {z}");
    println!("Output from plus_one_return function is: {zr}");

    let y = {
        let x = 3; // this is an statement
        x + 1 // expression with return because of missing ";"
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// All functions defined below

fn another_function() {
    println!("Called another function.");
}

fn another_function_with_args(x: i32) {
    println!("Called another function with args: The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5 // if not specified, a function returns the last expression
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_one_return(x: i32) -> i32 {
    let y = x + 1;
    return y;
}
