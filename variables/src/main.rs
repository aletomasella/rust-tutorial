fn get_inputs() -> (i32, i32) {
    
  println!("Please enter the numbers you want to use.");
  println!("Enter the first number:");
  let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
  let input : i32 = input.trim().parse().expect("Please type a number!");

  println!("Enter the second number:");
  let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
  let input2 : i32 = input2.trim().parse().expect("Please type a number!");

    return (input, input2);
}

fn basic_operations() {
     struct Operations {
      sum: fn(i32, i32) -> i32,
      sub: fn(i32, i32) -> i32,
      mul: fn(i32, i32) -> i32,
      div: fn(i32, i32) -> i32,
      reminder: fn(i32, i32) -> i32,
    }

    let operations = Operations {
      sum: |a, b| a + b,
      sub: |x, y| x - y,
      mul: |x, y| x * y,
      div: |x, y| x / y,
      reminder: |x, y| x % y,
    };

    let (input, input2) = get_inputs();

    println!("Your numbers are {} and {}", input, input2);

    let a = input;
    let b = input2;

    println!("{} + {} = {}", a, b, (operations.sum)(a, b));

    println!("{} - {} = {}", a, b, (operations.sub)(a, b));

    println!("{} * {} = {}", a, b, (operations.mul)(a, b));

    println!("{} / {} = {}", a, b, (operations.div)(a, b));

    println!("{} % {} = {}", a, b, (operations.reminder)(a, b));

    println!("{}", '😻');
 }

  fn access_array() {
   let mut tries = 0;
    let array = [1, 2, 3, 4, 5];

    while tries < 5 {
      println!("Please enter the index of the array you want to access.");
      let mut input = String::new();
      std::io::stdin().read_line(&mut input).expect("Failed to read line");
      let input : usize = input.trim().parse().expect("Please type a number!");

      // fixing error of array index out of bounds
      let input = input % array.len();

      println!("The value at index {} is {}", input, array[input]);
      tries += 1;
    }
    
 }





fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // error[E0384]: cannot assign twice to immutable variable `x`

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // No error

    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);
    // No error (shadowing)

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);
    // No error (shadowing and type change)
 
 basic_operations();

  access_array();
}
