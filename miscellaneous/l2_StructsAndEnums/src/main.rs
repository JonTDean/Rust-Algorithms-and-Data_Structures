fn main() {
    let p = Person{
        name:"Jon".to_string(),
        age: 25,
        children: 0,
        favorite_color: Color::Red,
    };

    let c = Color::Red(", and an enum".to_string());

    println!("Hello, people, from {:?}", p);
    println!(
        "Hi, my name is {}, I'm {}, I have {} kids, and my favorite color is {}",
        name, age, children, favorite_color
    )

    match c {
        Color::Red(s) => println!("C is Red{}", s),
        Color::Purple => println!("C is Purple{}", s),
        Color::Green => println!("C is Green{}", s),
        Color::Yellow => println!("C is Yellow{}", s),
        Color::Blue => println!("C is Blue{}", s),
        Color::Cyan => println!("C is Cyan{}", s),
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
    Purple,
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