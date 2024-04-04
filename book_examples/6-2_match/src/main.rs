// rustbook 6.2 

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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        },
    }
}

fn process_coin(coin: Coin) {
    println!("{}", value_in_cents(coin));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    // matching
    let coin = Coin::Penny;
    process_coin(coin);

    let coin = Coin::Quarter(UsState::Alabama);
    process_coin(coin);

    // matching with Option<T>
    let five = Some(5);
    let number_perhaps = None;

    println!("Result: {:?}", plus_one(five));
    println!("Result: {:?}", plus_one(number_perhaps));

    // n.b., matches must be exhaustive, e.g., this won't compile
    let x = Some(3);
    match x {
        Some(i) => Some(i+1),
        // None => None,  // uncomment this line to compile
    };

    // you can catch all other cases using other to pass a value, e.g.
    let dice_roll = 6;
    match dice_roll {
        7 => println!("lucky number seven!"),
        other => println!{"you got a {}", other},
    }
    
    // if you don't need the value you can use _
    match dice_roll {
        7 => println!("lucky number seven!"),
        _ => println!{"no seven, unlucky"},
    }

    // if you don't want to do anything for an arm you use ()
    match dice_roll {
        7 => println!("lucky number seven!"),
        _ => (),
    }
}
