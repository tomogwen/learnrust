// rustbook

fn main() {
    println!(
      "&String={} &str={}",
      std::mem::size_of::<&String>(),
      std::mem::size_of::<&str>(),
    );
  }

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    // *n = v[i - 1];
}

fn otherstuff() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}

fn stuff() {
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;

    let s: Vec<String> = vec![String::from("hello world")];
    let s_ref: &String = &s[0];
    println!("{}", s_ref);
}

fn refs2() {
    // let mut v: Vec<i32> = vec![1,2,3];
    // let el2: &i32 = &v[2];
    // println!("{:p}", el2);

    // v.push(30);
    // println!("element is {}", *el2);

    let mut v: Vec<i32> = vec![1,2,3];
    let num: &mut i32 = &mut v[2];

    //println!("{:p} : {:?}", &v, v);
    *num += 1;
    //println!("{:p} : {:?}", &v, v);

    let num2: &i32 = &*num;

    println!("{} {}", *num, *num2);

}


// fn references() {
//     // let x = String::from("hello");
//     // println!("{:p}", &x);
//     // let ref_ = &x;
//     // println!("{}", *ref_)

//     let mut x: Box<i32> = Box::new(1); // x points to data on the heap
//      // let a: i32 = *x; // a is the actual value of x (ie derefed value), this is a new variable on the stack
//     println!("{x}");
//     *x += 1; // this modifies the heap value, ie derefs - not dereferencing actually errors
//     println!("{x}");

//     println!("{:?}", x)

//     let r1: &Box<i32> = &x; // r1 points to x on the stack
//     let b: i32 == **r1; // dereference r1 to get x on the stack (which points to the heap val of r1), deref again to get the heap val

//     let r2: &i32 = &*x; // r2 is a ref - *x is the heap value, &*x points the heap value directly
//     let c2: i32 = *r2; // can deref once here as it points to the stack
// }