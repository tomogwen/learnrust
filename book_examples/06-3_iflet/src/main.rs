// Rustbook 6.3

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 'if let' lets you handle values that match one pattern whilst ignoring the rest
// this is particularly useful for the Option type
fn main() {
    let config_max = Some(3u8);  // Option::Some<u8> of value 3
    // this if let...
    if let Some(max) = config_max {
        println!("The max value is {}", max);
    }
    // ... is equivalent to this match
    match config_max {
        Some(max) =>  println!("The max value is {}", max),
        _ => (),
    }

    // we can also include an else with if let, e.g.,
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
