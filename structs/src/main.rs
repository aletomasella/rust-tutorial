
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new (username : &str, email : &str) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count: 1,
            active: true,
        }
    }

    fn sing_in(&mut self) {
        self.sign_in_count += 1;
    }
}



fn main () {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let mut user_1 = User::new("Alejandro", "aletoma@gmail.com");
    for _ in 0..10 {
        user_1.sing_in();
    }

    println!("{:#?}", user_1)

}