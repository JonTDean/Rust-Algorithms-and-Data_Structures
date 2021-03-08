// #[derive(Debug)]
// pub enum Res<T,E>{
//     Thing(T),
//     Error(E),
// }

fn main() {
    println!("Hello, world!");
    let a = divide(4, 5);
    let b = divide(10, 0);
    let c = divide(10, 5);
 
    println!("a = {:?} b = {:?}", a, b);
    match c {
        Ok(v) => println!("Value = {}", v),
        _ => {}
    }
}

fn divide(a:i32, b:i32) -> Result<i32, String>{
    if b == 0{
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
    
}