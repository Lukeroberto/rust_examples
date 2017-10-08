fn main() {
    test_1();
    test_2();
}

fn test_1(){

}

fn test_2(){

}


fn first_word(s: &String) -> usize {
    let byte = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len();
}
