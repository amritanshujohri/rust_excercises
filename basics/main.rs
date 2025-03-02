use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess: u32;
    let mut count = 0;
    loop
    {
        println!("Please input your guess.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        guess = input.trim().parse().expect( &format!("___Please type a number! --> {} ___", input));
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        if count == 6
        {
            println!("You lose! The number was: {}", secret_number);
            break;
        }
        count += 1;
    }
 
}