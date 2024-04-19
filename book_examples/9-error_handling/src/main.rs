// Rust groups errors into recoverable or unrecoverable
// A recoverable error can be handled in program - has type Result<T, E>
// Unrecoverable errors halt the program - has panic! macro

mod panic;
mod recoverable;

fn main() {
    // panic::examples();
    /recoverable::examples();
}
