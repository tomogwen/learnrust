// Lifetimes are another type of generic - that we've already been using!
// Rather than ensure that a type has the behaviour we want, lifetimes ensure references are valid for as long as required
// Every reference in rust has a lifetime, which is the scope for which the reference is valid
// Most of the time they are implicit and inferred, but we must annotate lifetimes when the lifetimes of references could be related in different ways
// We're required to annotate relationships using generic lifetime parameters to ensure the references are valid at runtime

// -- Preventing dangling references -- 
fn dangling() {
    let r;
    {
        let x = 5;
        r = &x;
    }

    // println!("r: {}", r);
    // with the println this code won't compile as x ends at the ending of the inner scope
    // if r was still a reference to x we'd have a dangling reference 
}

pub fn examples() {
    dangling();
}