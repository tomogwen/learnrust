// Strings are a collection of bytes
// Plus methods that provide functionality when those bytes are text
// Strings are UTF-8 encoded, so can include many characters

fn basics() {
    let mut s = String::new();
    let data = "Hello";  // string literal - stored in the binary
    s = data.to_string();  // stores data as String - collection on heap
    println!("{s}");

    // can push_str to append a string slice
    let s2 = " World";
    s.push_str(s2);  // this doesn't take ownership of s2
    println!("Appended: {s2}");  // so we can still use it
    println!("{s}");

    let s3 = '!';
    s.push(s3);  // push adds a char
    println!("{s}");

    let s4 = String::from("Bye ");
    let s5 = String::from("World :(");
    let s6 = s4 + &s5;  // you can concat strings with +, but we lose ownership of s4
    // this happens due to 'ownership coercion of s5' - turning String into an &str, in particular
    // 1. add takes ownership of s4
    // 2. it appends a copy of s5 to s4
    // 3. it returns ownership of s4
    // s4 may need reallocating on the heap (if s4+s5 is too long), hence being unable to use s4 again

    // for longer concats we can use the format macro
    let s7 = String::from("Tic");
    let s8 = String::from("Tac");
    let s9 = String::from("Toe");
    let game = format!("{s7}-{s8}-{s9}");
    println!("{game}");
}

fn indexing() {
    let s1 = String::from("hello");
    // let h = s1[0];  // cannot index strings by ints to retrieve chars in rust
    // this is due to how rust stores strings in memory - String wraps Vec<u8>
    // some UTF-8 characters are longer than u8
    // to prevent returning potentially incorrect u8 chars, rust prevents integer indexing Strings

    // there are three ways to consider Strings - as bytes, scalars, and grapheme clusters (essentially letters)
    // for example, the hindi word “नमस्ते”, written in Devanagari is:
    // Bytes: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // Unicode scalars: ['न', 'म', 'स', '्', 'त', 'े'] - 4th and 6th values _aren't_ letters, they're diacritics
    // Letters: ["न", "म", "स्", "ते"]

    // finally, indexing is expected to take O(1) time - if rust needs to parse the number of bytes in each char then it takes too long!
}

fn slicing_iterating() {
    // a string slice requires a range of ints and returns the corresponding bytes
    // it will panic if you request e.g., half of a multi-byte character
    let s = "नमस्ते";
    let slice = &s[0.. 3];
    // let slice = &s[0..4];  // will not run - it is inside 'म' (bytes 3..6) of `नमस्ते`
    println!("{slice}");

    // the best way to access Strings is to be specific about if you want strings or bytes
    println!("String: {s}");
    print!("Chars: ");
    for c in s.chars() {
        print!{"{c}, "};
    }
    print!("\nBytes: ");
    for b in s.bytes() {
        print!{"{b}, "};
    }
    print!("\n");
}

pub fn examples() {
    basics();
    indexing();
    slicing_iterating();
}