// There are two ways to cause a panic in practice
// (i) an action that causes panic, e.g., trying to access an array with an invalid index
// (ii) by calling the panic! macro
// By default rust walks up the stack and cleans up on panic, which is a lot of work
// You can choose to immediately abort and leave clean up to the OS by adding to cargo.toml:
// [profile.release]
// panic = 'abort'


pub fn examples() {
   // panic!("crash and burn");

   let v = vec![1, 2, 3];
   v[99];  // oob error
}