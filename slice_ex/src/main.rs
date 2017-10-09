#![allow(unused_variables)]

fn main() {
    // Trying to slice strings with indicies
    //test_1();

    // Using slices
    // test_2();

    // Test 1 but with slices istead
    test_3();
}

fn test_1(){
    // Define the string s as hello world
    let mut s = String::from("hello world");
    println!("s is currently: {}", s);
    // return the index of first word, which is 5
    let word = first_word(&s);
    println!("First word index: {}", word);

    s.clear();
    println!("s is now: {}", s);
    println!("First world index is: {}", word);
}

fn test_2(){
    let s = String::from("hello world");

    // Only a reference to a slice of the string
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Slices (1,2): ({}, {})", hello, world);
}

fn test_3(){
    // Define the string s as hello world
    let mut s = String::from("hello world");
    println!("s is currently: {}", s);
    // return the index of first word, which is 5
    let word = first_word_v2(&s);
    println!("First word index: {}", word);

    s.clear();
    println!("s is now: {}", s);
    println!("First world index is: {}", word);
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
