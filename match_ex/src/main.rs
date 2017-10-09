fn main() {
    example_1();
}


fn example_1(){
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
        Nothing,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny   => 1,
            Coin::Nickel  => 5,
            Coin::Dime    => 10,
            Coin::Quarter => 25,
            _             => 0,
        }
    }

    let c = Coin::Nothing;
    println!("Value of c: {}", value_in_cents(c));
}
