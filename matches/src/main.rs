fn main() {
    let my_penny = Coin::Penny;
    let my_quarter = Coin::Quarter;
    let my_quarter_alaska = Coin2::Quarter(UsState::Alaska);

    value_in_cents(my_penny);
    value_in_cents2(my_quarter_alaska);

    // Testing the Option<T> feature
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Testing the match - other feature
    let dice_roll = 9;
    match dice_roll {
        //3 => add_fancy_hat(),
        //7 => remove_fancy_hat(),
        //other => move_player(other),
        3 => println!("three"),
        7 => println!("seven"),
        other => println!("other number!, {other}"),
        _ => println!("other number!"), // if you don't want to handle the data
        _ => (),                        // if you don't want to do anything with it
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
