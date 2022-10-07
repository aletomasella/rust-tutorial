use std::io::stdin;

use rand::Rng;

use std::cmp::Ordering;


fn main() {
    // println!("Guess a number!");
    
    // println!("Please input your guess.");
    
    // let secret_numer = rand::thread_rng().gen_range( 1..101);
    
    // // println!("The secret number is: {}", secret_numer);

    // let mut guess = String::new();

    // stdin().read_line(&mut guess)
    //     .expect("Failed to read line");

    // // Shadowing

    // let guess= guess.trim().parse::<u32>().expect("Please type a number!");

    //       match guess.cmp(&secret_numer) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => println!("You win!"),
    //     }


    // while guess != secret_numer {
    // let mut guess = String::new();

    //   stdin().read_line(&mut guess)
    // .expect("Failed to read line");
    
    // let guess= guess.trim().parse::<u32>().expect("Please type a number!");

    //     match guess.cmp(&secret_numer) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => println!("You win!"),
    //     }
    // }

    // With loop


    pub fn game () {
      println!("THE GUESS NUMBERS GAME");
      let mut max_range = String::new();
      println!("Please enter the maximum range of the game");
      stdin().read_line(&mut max_range).expect("Failed to read line");
      let max_range: u32 = max_range.trim().parse().expect("Please type a number!");
      let secret_numer = rand::thread_rng().gen_range( 1..=max_range);
      println!("Guess a number between 1 and {}", max_range);
        // let secret_numer = rand::thread_rng().gen_range( 1..101);
    loop {
        let mut guess = String::new();

        stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess : u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };
        

        match guess.cmp(&secret_numer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                finish_game();
            }
        }
        
    }
    }


    pub fn finish_game() {
        println!("You win!");
        println!("Do you want to play again? (y/N)");
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("Error reading answer");
        if answer.trim() == "N" {
            println!("Thanks for playing!");
            std::process::exit(0);
        } else {
          game()
        }
    }

    game()

  
  }
        
        
    


// Language: rust
