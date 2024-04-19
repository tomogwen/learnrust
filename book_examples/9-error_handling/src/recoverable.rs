// Most errors can be recovered
// Recall Results are enums:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn matching() {
    // with matching errors we can take different actions depending on the problem
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // if the file doesn't exist, create it
            ErrorKind::NotFound => match File::create("hello.txt") {
                // can match errors again in case we can't create it
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            }
            // any other error, panic
            other_error => panic!("Can't open the file: {:?}", other_error),
        }
    };
}

fn closures() {
    // closures are a cleaner/less primitive way to handle errors
    // come back after chapter 13!
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn unwrap_expect() {
    // unwrap returns T on Ok(T) and panics on Err
    // let greeting_file = File::open("hello.txt").unwrap();

    // expect lets us choose the panic error message
    // expect is typically used in production-quality code
    let greeting_file = File::open("hello.txt").expect("hello.txt expected");
}

fn read_username_from_file_manual() -> Result<String, io::Error> {
    // this function manually propogates errors by returning Result
    let username_result = File::open("hello.txt");
    let mut username_file = match username_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // this is the same as the 'manual' error propogation - much easier!
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn propogating_errors() -> Result<String, io::Error> {
    // you can propogate errors back to calling code to decide how to handle it
    // e.g., read_username_from_file_manual returns an Option, allowing it to pass an Ok or Err back
    // read_username_from_file uses ? to propogate errors

    // let user_result: Result<String, io::Error> = read_username_from_file_manual();
    // let user_result: Result<String, io::Error> = read_username_from_file();
    let user_result = read_username_from_file_shorter();
    // let user_result = fs::read_to_string("hello.txt");  // the std lib provides a function to do this!
    println!("{:?}", user_result);
    user_result
}

fn questions_when(text: &str) -> Option<char> {
    // we can only use ? when the return type is compatible with the value ? is used on
    // because ? performs an early return in the case of an error
    // compiling the below without a return type on questions_when fails
    // let file = File::open("hello.txt")?;

    // we can use ? on Option as well - it returns early on None
    text.lines().next()?.chars().last()  // returns none if no lines or last char otherwise
}

pub fn examples() {
    // matching();
    // closures();
    // unwrap_expect();
    let username = propogating_errors().expect("Requires hello.txt");
    let last_char = questions_when(&username).expect("Requires non-empty hello.txt");
    println!("Last char: {}", last_char);
}
