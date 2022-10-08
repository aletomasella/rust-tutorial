

fn main() {
    // println!("Hello, world!");
    

    // let array = [1, 2, 3, 4, 5];

    // for mut el in array {
    //       el += 1;
    //     println!("{}", el);
    // }

    // for number in 1..=10 {
    //     print!("{} ", number);
    // }

    // println!("{:?}", array);

    // Fibonacci recursive
    // fn fibonachi(n: i32) -> i32 {
    //     if n == 0 {
    //         return 0;
    //     } else if n == 1 {
    //         return 1;
    //     } else {
    //         return fibonachi(n - 1) + fibonachi(n - 2);
    //     }
    // }

    // Fibonacci iterative
    fn fibonachi(n: i128) -> i128 {

      let mut cache = vec![0, 1]; 
      for num in 2..=n {
        cache.push(cache[num as usize - 1] + cache[num as usize - 2]);
      }
      return cache[n as usize];
    }

    
    let result = fibonachi(100);
    println!("{}", result);
}
