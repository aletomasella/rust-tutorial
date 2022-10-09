

fn takes_ownership (s: String) {
    println!("{}", s);
}

fn makes_copy (i: i32) -> i32 {
    println!("{}", i);
    i
}

fn make_string (str : &str) -> String {
    let s = String::from(str);
    s
}

fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}



fn main() {

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        print!("{}", s);

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    let s = String::from("hello");  // s is valid from this point forward

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x is valid from this point forward

    let x  = makes_copy(x);
    
    print!("{x}");// x would move into the function,
                                    // but i32 is Copy, so it's okay to still

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);


    print!("{}", s1);

    let mut s = make_string("hello");
    s.push_str(", world!");
    println!("{}", s);

    let mut random_words = String::from("Hola como estas todo bien re loco lo vago que es el mundo");

    let  array = random_words.split(" ").collect::<Vec<&str>>();

    let first = first_word(&random_words[..]);

    println!("The first word is: {}", first);

    println!("{:?}", array);

    random_words.clear();

    println!("{}", random_words);

    // let mut str = "Hola como estas todo bien re loco lo vago que es el mundo";
    // str = "Hola";
    // println!("{}", str);
               // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
