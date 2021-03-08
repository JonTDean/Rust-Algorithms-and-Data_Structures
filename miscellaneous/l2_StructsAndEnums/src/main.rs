fn main() {
    let p = Person{
        name:"Jon".to_string(),
        age: 25,
        children: 0,
        favorite_color: Color::Purple,
    };

    let c = Color::Red(", and an enum".to_string());

    println!("Hello, people, from {:?}", p);

    match c {
        Color::Red(s) => println!("C is Red{}", s),
        Color::Purple => println!("C is Purple"),
        Color::Green => println!("C is Green"),
        Color::Yellow => println!("C is Yellow"),
        Color::Blue => println!("C is Blue"),
        Color::Cyan => println!("C is Cyan"),
    }
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    favorite_color: Color,
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Purple = "Purple".to_string(),
    Green,
    Yellow,
    Blue,
    Cyan,
}

impl Person {
    pub fn print(self) -> String{
        format!(
            "name = {}, age = {}, has {} children",
            self.name, self.age, self.children
        )
    }
}