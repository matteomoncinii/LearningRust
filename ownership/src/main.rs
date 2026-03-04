fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s.clone()); // s's value moves into the function...
                                // ... and so is no longer valid here

    println!("Variable s still exists in main: {s}");

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    println!("Cannot print s anymore");

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
                   // x does NOT move into the function,
                   // so it's okay to use x afterward.

    // Moving ownership between main and functions
    println!("Moving ownership between main and functions");

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("s1 still exists in main: {s1}");

    let s2 = String::from("hello"); // s2 comes into scope

    let (s3, s3_len) = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s2 do not exists anymore");
    println!("s3 still exists in main: {s3}");
    println!("s3_len still exists in main: {s3_len}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}"); // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn takes_and_gives_back(a_string: String) -> (String, usize) {
    // a_string comes into scope
    let a_length = a_string.len();
    (a_string, a_length) // a_string is returned and moves out to the calling function
}
