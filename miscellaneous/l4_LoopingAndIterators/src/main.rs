fn main() {
    let mut n: u8 = 0;
    let mut stLoop = Stepper{ curr:0, step:1, max:6 };
    let mut stWhile = Stepper{ curr:0, step:5, max:51 };
    let stFor = Stepper{ curr:0, step:10, max:101 };

    // Looping Constructs
    // // Basic Loop
    loop {
        n += 1;
        if n > 10 {
            break;
        }
        
        println!("Basic Loop Construct -");
        println!{"Variable n is currently equal to {}", n};
    }
    n = 0;

    // // Basic Loop with Step Struct
    loop{
        match stLoop.next(){
            Some(num) => {
                println!("Basic Loop with Step Struct Construct -");
                println!("Stepper Loop {}", num);
            },
            None => break,
        }
    }

    // // While Loop
    while n < 11 {
        n += 1;
        println!("While Loop Construct -");
        println!{"Variable n is currently equal to {}", n};
    }

    // // While Let Loop with Step Struct
    while let Some(num) = stWhile.next(){
        println!("While Let Loop with Step Struct Construct -");
        println!{"Some(num) is currently equal to {}", num};
    }

    // // for loop
    for i in  0..=10{
        println!("For Loop Construct -");
        println!{"Variable i is currently equal to {}", i};
    }

    for i in stFor {
        println!("For Loop Construct -");
        println!("For Loop Stepper is currently equal to {}", i);
    }
}

pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32>{
        if self.curr >= self.max{
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}