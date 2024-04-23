// enums rust book

// enums list the kinds that can be contained
// e.g. IpAddress can be V4 _or_ V6.
// Listing the types after the kind gives it a constructor as well
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enums can have a range of different kinds
enum Message {
    Quit,  // you don't even need types associated!
    Move { x: i32, y: i32 },  // you can do struct-like named fields in enums too
    Write(String),  // nb curly brackets when not named fields?
    ChangeColour(i32, i32, i32),
}

// we can also define methods on enums
impl Message {
    fn call(&self) {
        println!("Example method");
    }
}

// the Option enum encodes the scenario in which a value is something or nothing
// many operations return nothing, e.g., requesting the first value in an empty list
// this might be represented by nulls in other languages, but this leads to many problems
// instead Rust has Option<T> - <T> is a generic type parameter - it can be any type
// the standard implementation is simply:
/*
enum Option<T> {
    None,
    Some(T),
}
*/

fn main() {
    let home = IpAddress::V4(127, 0, 0, 1);  // this is how you call the V4 'version' of the IpAddress enum
    let loopback = IpAddress::V6(String::from("::1"));
    
    let message = Message::Write(String::from("hello world"));
    message.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let no_number: Option<i32> = None;

    // Options are better than a null value, because the compiler won't let us use an Option type .. 
    // .. as if it definitely has a value. 
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // this will run not run at compile time!
    let sum = x + y;
}
