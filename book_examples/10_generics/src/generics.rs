// Generics allow us to remove code duplication if you have a function that is e.g. identical up to input/return types

// -- function generics --
fn find_largest_i32(nums: &[i32]) -> &i32 {
    let mut largest = &nums[0];
    for number in nums {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Define T as the generic type, require it has the PartialOrd trait
fn find_largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn function_generics() {
    let nums = vec![1, 2, 3, 4, 5];
    let largest = find_largest_i32(&nums);
    println!("Largest: {}", largest);

    let chars = vec!['b', 'a', 'c'];
    let largest = find_largest_char(&chars);
    println!("Largest: {}", largest);

    let largest_int = find_largest_generic(&nums);
    let largest_char = find_largest_generic(&chars);
    println!("Largest: {}", largest_int);
    println!("Largest: {}", largest_char);
}

// -- struct generics --
struct Point<T> {
    x: T,
    y: T,
}

struct TwoTypePoint<T, U> {
    x: T,
    y: U,
}

fn struct_generics() {
    let int_point = Point {x: 5, y: 10};
    let float_point = Point {x: 4.3, y: 2.2};

    // our Point Struct won't work if T is implied to be two different types, e.g., with
    // let bad_point = Point {x:4, y: 2.4};
    // but we can define a struct to have two generics, e.g.,
    let new_point = TwoTypePoint {x: 5.5, y: 4};
}

// -- enum generics --
// we've seen examples of these before:
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// -- generics in methods on structs and enums --
struct PointMethods<T> {
    x: T,
    y: T,
}

// this method on Point<T> has generic type
impl<T> PointMethods<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// but we can also make methods only available to specific instantations
impl PointMethods<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// generic types in structs aren't always the same as those in the methods
// you can use different generic type names to indicate this, e.g.:
struct PointMixed<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMixed<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMixed<X2, Y2>) -> PointMixed<X1, Y2> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

fn method_generics() {
    let p1 = PointMixed { x: 5, y: 10.4 };
    let p2 = PointMixed { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


pub fn examples() {
    function_generics();
    struct_generics();
    method_generics();
}