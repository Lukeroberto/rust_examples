
extern crate rand;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Guess the number!");
    //println!("Secret Number is: {}", secret_number);  // Debug statement

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);

        // Lets us contiue even if input is not a number
        let guess: u32 = match guess.trim().parse(){ //.expect("Please type a number!");
            Ok(num) => num,
            Err(_)  => continue,
        }  ;

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less    => println!("To small!"),
            std::cmp::Ordering::Greater => println!("To big!"),
            std::cmp::Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
