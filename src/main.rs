extern crate rand; //indicate that we're using an external cargo crate: rand

use std::io;
use rand::Rng; //A trait that defines methods that random number generators implement
use std::cmp::Ordering;

fn main() {

    println!("Guess the number");
    println!("Please input your guess: ");

    let mut random = rand::thread_rng();
    let number = random.gen_range(1,100);
    //println!("i32: {:b}, u32: {}", random.gen::<i32>(), random.gen::<u32>());

    loop { //loop is a build in keyword that loops forever
        let mut guess = String::new(); //:: indicates static (associated) function

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        //stdin is an associated function that returns an instanc of std::io::Stdin
        //we call the method read_line using dot-notation
        //& indicates that the argument mut is a reference

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {println!("Not a number! Try again..."); continue;}
        };
        //When we reuse a variable name it is called shadowing!
        //We use trim() to eliminate whitespace but also the \n character that is saved!

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small, guess again: "),
            Ordering::Greater => println!("Too large, guess again: "),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
