fn main() {
    println!("\n--- Playing with ownership");
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
    println!("\n--- Moving ownership between main and functions");

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("s1 still exists in main - s1: {s1}");

    let s2 = String::from("hello"); // s2 comes into scope

    let (s3, s3_len) = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s2 do not exists anymore");
    println!("s3 still exists in main - s3: {s3}");
    println!("Length s3: {s3_len}");

    println!("\n--- Print length with reference");
    let len_s1 = takes_reference(&s1);
    println!("s1: {s1} - Length s1: {len_s1}");

    println!("\n--- Change String with mutable reference");
    let mut s4 = String::from("hello");
    println!("s4 before: {s4}");
    takes_ref_and_change(&mut s4);
    println!("s4 after: {s4}");

    println!("\n--- Ref and Mut Ref combination");
    let mut s5 = String::from("hello");
    println!("s5: {s5}");
    // Cannot borrow a reference and a mut reference
    // let r0 = & s4;
    // let r1 = &mut s4;
    // Cannot borrow a more than one mut reference
    // let r1 = &mut s4;
    // let r2 = &mut s4;
    let ref1 = &s5; // no problem
    let ref2 = &s5; // no problem
    println!("Reference 1: {ref1}; Reference 2: {ref2}");
    // Variables r1 and r2 will not be used after this point.
    let ref3 = &mut s5; // no problem
    println!("Reference 3: {ref3}");

    // THE SLICE TYPE
    println!("\n--- The Slice Type");
    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..5]);
    println!("Input as slice of the string: {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);
    println!("Input as a reference to the string: {word}");
    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    println!("Input as a slice of the string literal: {word}");
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("Input as a string literal (which is a slice itself){word}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("printing string from 'takes_ownership' function: {some_string}"); // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
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

fn takes_reference(ref_string: &String) -> usize {
    let a_length = ref_string.len();
    a_length
}

fn takes_ref_and_change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
