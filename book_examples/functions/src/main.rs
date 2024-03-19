// functions rustbook

// statements are instructions that perform an action and do not return a value, e.g., using 'let', 'fn'
// expressions evaluate to a value
// rust is an expression-based language

fn main() {
    // println!("hello world");
    // another_function(5, 'h');
    let mut x = assign_five();
    print_value(x);
    x = plus_one(x);
    print_value(x);
}

// fn another_function(value: i32, unit_label: char) {
//     println!("the value of x is: {value}{unit_label}");
// }

fn print_value(value: i32) { 
    println!("the val is: {value}");
}

fn assign_five() -> i32 {
    5
}

fn plus_one(num: i32) -> i32 {
    num + 1
}
