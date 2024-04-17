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
    // even though the elements are at different ends of the vector, the borrow checker still prevents this
    // you may e.g. have to move the whole vector on the heap when adding new elements to the end
    println!("{first}");
}

fn iterating() {
    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;  // dereference to get value from address of an element in v
        println!("{n_plus_one}");

        // we can also dereference n_ref to change the value
        // as long as its `in &mut v`
        *n_ref += 10;
    }
    println!("v: {:?}", v);
}

pub fn examples() {
    creating_pushing_reading();
    borrowing();
    iterating();
}
