fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 6;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let guess: u32 = "4294967295".parse().expect("Not a number!");
    println!("The guess is: {guess}");

    // tuple
    let tup = (500, 6.4, 1);
    let (tup_x, tup_y, tup_z) = tup;
    println!("The value of tup_x, tup_y, tup_z are: {tup_x}, {tup_y}, {tup_z}");

    // array
    let x_array: (i32, f64, u8) = (500, 6.4, 1);
    let x_array_first = x_array.0;
    let x_array_second = x_array.1;
    let x_array_third = x_array.2;
    //println!("The x_array is: {x_array}");
    println!(
        "The value of x_array_first, x_array_second, x_array_third are: {x_array_first}, {x_array_second}, {x_array_third}"
    );

    let a_array: [i32; 5] = [1, 2, 3, 4, 5];

    let a_array_first = a_array[0];
    let a_array_second = a_array[1];
    //println!("The a_array is: {a_array}");
    println!("The value of a_array_first, a_array_second are: {a_array_first}, {a_array_second}");

    //let aa_array = [3; 5];
    //println!("The aa_array is: {aa_array}");
}
