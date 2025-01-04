// helper functions

fn demonstrate_match(){

    println!("Rust has a powerful pattern-matching system that allows for powerful match case operations with the match-arm operator => and the catch-all wildcard operator _");

    let input:char = 'x';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"), // match-all statement
    }

    println!("Pattern-matching can even be applied within existing destructuring operations!");

    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }

    println!("Use cases for this powerful pattern-matching quickly become obvious once we combine them with the enum type.");

    enum Result {
        Ok(i32),
        Err(String),
    }

    println!("This is since Enums themselves can act as wrappers that store values that can then be read via pattern-matching.");

    fn divide_in_two(n: i32) -> Result {
        if n % 2 == 0 {
            Result::Ok(n / 2)
        } else {
            Result::Err(format!("cannot divide {n} into two equal parts"))
        }
    }

    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }

}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expression {
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },
    Value(i64),
}


fn eval(e: Expression) -> i64 {
        match e {
        Expression::Value(v) => v, 
        Expression::Op { op, left, right } => {
            let left_val = eval(*left);   
            let right_val = eval(*right); 
            match op {
                Operation::Add => left_val + right_val,
                Operation::Sub => left_val - right_val,
                Operation::Mul => left_val * right_val,
                Operation::Div => {
                    if right_val == 0 {
                        panic!("Division by zero!"); 
                    }
                    left_val / right_val
                }
            }
        }
    }
}

fn demonstrate_methods(){
    println!("Rust further allows you to associate functions with your user-defined types via the impl block");
    println!("Frankly, this is the closest we will get to classes in Rust, with user-defined types and methods implemented on those types.");

    #[derive(Debug)]
    struct Race {
        name: String,
        laps: Vec<i32>,
    }
    impl Race {
        fn new(name: &str) -> Self {
            Self { name: String::from(name), laps: Vec::new() }
        }

        fn add_lap(&mut self, lap: i32) {
            self.laps.push(lap);
        }

        fn print_laps(&self) {
            println!("Recorded {} laps for {}:", self.laps.len(), self.name);
            for (idx, lap) in self.laps.iter().enumerate() {
                println!("Lap {idx}: {lap} sec");
            }
        }

        fn finish(self) {
            let total: i32 = self.laps.iter().sum();
            println!("Race {} is finished, total lap time: {}", self.name, total);
        }
    }
    let mut race:Race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();

}

fn demonstrate_traits(){
    println!("Recall that similar to traits in other languages, Rust traits allow us to define the outline of a method by specifying its type signature without necessarily specifying that method's default implementation.");

    trait Pet {
        fn talk(&self) -> String;

        fn greet(&self) {
            println!("Oh you're a cutie! What's your name? {}", self.talk());
        }
    }

    struct Dog {
        name: String,
        age: i8,
    }

    impl Pet for Dog {
        fn talk(&self) -> String {
            format!("Woof, my name is {}!", self.name)
        }
    }

    let fido = Dog { name: String::from("Fido"), age: 5 };
    fido.greet();

    println!("To be cute (perhaps), Rust also further introduces the concept of supertraits, which operate very similarly to class inheritance but are not considered equivalent to OOP for reasons rooted entirely in what appears to be principle");

    trait Human {
        fn leg_count(&self) -> u32;
    }

    trait Gooner: Human {
        fn name(&self) -> String;
    }

    struct Fella(String);

    impl Human for Fella {
        fn leg_count(&self) -> u32 {
            4
        }
    }

    impl Gooner for Fella {
        fn name(&self) -> String {
            self.0.clone()
        }
    }

    let chunk = Fella(String::from("Blud"));
    println!("{} has {} legs", chunk.name(), chunk.leg_count());

    println!("Further, we are able to derive certain traits with Macros, which can provide incredibly useful when seeing the substantive default macros library Rust provides.");
    #[derive(Debug, Clone, Default)]
    struct Player {
        name: String,
        strength: u8,
        hit_points: u8,
    }

    let p1:Player = Player::default(); 
    let mut p2:Player = p1.clone(); 
    p2.name = String::from("EldurScrollz");
    println!("{p1:?} vs. {p2:?}");

}

pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, verbosity: u8, message: &str) {
        println!("verbosity={verbosity}: {message}");
    }
}

struct VerbosityFilter<L: Logger> {
    max_verbosity: u8,
    inner: L,
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

// execution code

fn main() {

    println!("~~~ Comprehensive Rust: Day 2 Morning ~~~");

    // pattern-matching 

    demonstrate_match();

    // arithmetic parser exercise
    
    #[test]
    fn test_value() {
        assert_eq!(eval(Expression::Value(19)), 19);
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(20)),
            }),
            30
        );
    }

    #[test]
    fn test_recursion() {
        let term1 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        };
        let term2 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5)),
        };
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            }),
            85
        );
    }

    #[test]
    fn test_zeros() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(0)),
                right: Box::new(Expression::Value(0))
            }),
            0
        );
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Mul,
                left: Box::new(Expression::Value(0)),
                right: Box::new(Expression::Value(0))
            }),
            0
        );
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(0)),
                right: Box::new(Expression::Value(0))
            }),
            0
        );
    }

    // methods and traits

    demonstrate_methods();
    demonstrate_traits();

    // generic logger exercise
    
    let logger = VerbosityFilter { max_verbosity: 3, inner: StdoutLogger };
    logger.log(5, "FYI");
    logger.log(2, "Uhoh"); 

}
