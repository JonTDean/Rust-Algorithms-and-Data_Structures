use std::io;

fn main() {
    println!("Factorial {:?}", factorial(12));

    // // Stack Overflow Error
    // println!("Factorial {:?}", factorial(13)); 

    // // Combination of Factorial(n) and Stack Overflow
    // for num in 0..=13{
    //     println!("Current Factorial of Number {} is : {}", num, factorial(num));
    // }
}

fn factorial(n: i32) -> i32{
    if n <= 1{
        return 1;
    }

    n * factorial(n - 1)
}
  
// Stack Corrective Factorial
fn stackFactorial(n: i32, r: i32) -> i32{
    if n <= 1 {
        return r;
    }

    stackFactorial(n - 1, n * r)
}