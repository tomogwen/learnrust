// separating the examples into different files

fn creating_pushing_reading() {
    let v:Vec<i32> = Vec::new();
    let mut w = vec![1, 2, 3];  // this macro creates a Vec<i32> (as default ints are i32)
    w.push(4);  // we can add values to a vector with push
    println!("Vector: {:?}", w);

    // we can read elements of a vector with indexing or .get()
    let third: &i32 = &w[2];  // indexing
    println!("Third element is {third}");

    let third: Option<&i32> = v.get(2);  // getting
    match third {
        Some(third) => println!("Third element is {third}"),
        None => println!("There is no third element"),
    }

}

fn borrowing() {
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // mutable borrow occurs here!
    let first = v[0];  // compiles if not mutable, i.e., not borrowed/referenced
    v.push(6);  // mutable borrow also occurs here
    println!("{first}");
}

pub fn examples() {
    creating_pushing_reading();
    borrowing();
}

