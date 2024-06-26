// we can use nested paths to bring multiple items into scope in one line, e.g.,
use std::{cmp::Ordering, io};

// or the whole module with self
use std::io::{self, Write};

// or even all modules
use std::collections::*;


// there are a couple of idiomatic ways to bring items with the same name into scope
// the first is to keep the parent module

use std::fmt;
use std::io;


fn example1() -> fmt::Result {
    // return fmt::Result
}

fn example2() -> io::Result<()> {
    // return io::Result
}

// an alternative way is with use.. as to alias the types locally
use std::fmt::Result;
use std::io::Result as IoResult;

fn example1() -> Result {
    // return fmt::Result
}

fn example2() -> IoResult<()> {
    // return io::Result
}
