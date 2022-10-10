
#[derive(Debug)]
pub struct User {
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

    pub fn get_username (&self) -> &str {
        &self.username
    }

   pub  fn get_email (&self) -> &str {
        &self.email
    }

  pub  fn get_sign_in_count (&self) -> u64 {
        self.sign_in_count
    }

   pub  fn get_active (&self) -> bool {
        self.active
    }


    fn sing_in (&mut self) {
        self.sign_in_count += 1;
    }

}

use structs::get_avg::AverageCollection;

fn main () {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let mut user_1 = User::new("Alejandro", "aletoma@gmail.com");
    for _ in 0..10 {
        user_1.sing_in();
    }

    let mut users_array : Vec<User>= vec![];

    for i in 0..=50 {
      let i = &i.to_string();
      let name = "Alejandro ".to_owned() + i;
      let email = "alecito_".to_owned() + i + "@gmail.com";
      let new_user = User::new(&name, &email);
      users_array.push(new_user);
    }

    println!("{:#?}", users_array);


        let number = Some(5);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let six = plus_one(number);
    println!("{:?}", six);

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(u32),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(num) => num
        }
    }

    let coin = Coin::Quarter(1);
    println!("{}", value_in_cents(coin));

    let mut avg = AverageCollection::new();

    avg.add(1);
    avg.add(2);
    avg.add(3);
    avg.add(500);
    avg.add(1000);
    println!("{}", avg.average());
    avg.remove();

}


enum IpAddrKind {
    V4(String),
    V6(String),
}
    
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
