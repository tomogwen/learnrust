// The HashMap<K, V> collection stores a mapping of keys (type K) to values (type V)
use std::collections::HashMap;

fn adding_reading_ownership() {
    // for types that implement copy (e.g., i32) 
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 2);
    scores.insert(String::from("red"), 1);

    println!("{:?}", scores);

    let team = String::from("blue");
    let get_score = scores.get(&team);  // get returns an Option<&V> if key present, else None
    println!("{:?}", get_score);
    let score = get_score.copied().unwrap_or(0);  // copied: Option<&V> -> Option<i32>, unwrap_or(0) unwraps Option
    println!("{:?}", score);

    // can also iterate over keys+values
    for (key, value) in &scores {
        println!("Key: {key}, value: {value}");
    }

    // hashmaps take ownership of values that don't implement copy, like string
    let team3 = String::from("green");
    let score3 = 12;
    scores.insert(team3, score3);
    // println!("Team: {team3}");  // so this won't run
    println!("Score: {score3}");  // but this will, as i32 will be copied
}

fn updating() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // inserting on an exisiting key overwrites
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // hashmap.entry(value) returns Entry(VacantEntry) or Entry(OccupiedEntry {k: v})
    // can be used to add a value only if it doesn't exist in the hashmap
    println!("{:?}", scores.entry(String::from("Yellow")));
    println!("{:?}", scores.entry(String::from("Blue")));
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // you can dereference an entry to update a value based on the previous value
    // this counts the occurences of a word in a string
    let text = "hello world wonderful world";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", counts);
}

pub fn examples() {
    adding_reading_ownership();
    updating();
}
