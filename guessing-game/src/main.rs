use std::io::stdin;

use rand::Rng;

use std::cmp::Ordering;


fn main() {
    println!("Guess a number!");
    
    println!("Please input your guess.");
    
    let secret_numer = rand::thread_rng().gen_range( 1..101);
    
    // println!("The secret number is: {}", secret_numer);

    let mut guess = String::new();

    stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess= guess.trim().parse::<u32>().expect("Please type a number!");

          match guess.cmp(&secret_numer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }


    while guess != secret_numer {
    let mut guess = String::new();

      stdin().read_line(&mut guess)
    .expect("Failed to read line");
    
    let guess= guess.trim().parse::<u32>().expect("Please type a number!");

        match guess.cmp(&secret_numer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
        
    }
        
        
    


// Language: rust
