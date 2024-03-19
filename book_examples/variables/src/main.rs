// variables from rustbook

fn mute() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}

fn scopes() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let a: char = '\n';
    println!("{a}");

}