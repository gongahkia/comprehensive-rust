// helper functions

fn render_array() {
    let mut a: [i8; 10] = [42; 10]; // generator syntax
    a[5] = 0;
    println!("a: {a:?}");
}

fn render_tuple() {
    let t: (i8, bool) = (7, true); // generator syntax again
    println!("t.0: {}", t.0); // note the dot syntax for accessing tuple elements
    println!("t.1: {}", t.1); // note the dot syntax for accessing tuple elements
}

fn demonstrate_nested_loop(){
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
            assert_eq!(prime % i, 0);
        }
    }
}

fn demonstrate_pattern_matching(tuple:(i32, i32)){
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}

// fn demonstrate_invalid_reference(){
//     let x_ref = {
//         let x = 10;
//         &x
//     };
//     println!("x: {x_ref}");
// }

fn demonstrate_references(){
    println!("\n~ The best damn explanation on references you'll likely ever read. ~\n");
    println!("These below are IMMUTABLE REFERENCES (as with everything else in Rust, this is determined by default)");
    let a:char = 'A';
    let b:char = 'B';
    let mut r: &char = &a; // reference provides a way to access another value without taking ownership of the value, and are also called "borrowing", also note that shared references are read-only, and the referenced data cannot change and finally a shared reference to a type T has type &T
    println!("r: {}", *r);
    r = &b; // a reference value is made with the & operator and the * operator "dereferences" a reference, yielding the read-only value stored within that reference
    println!("r: {}", *r);
    println!("These below are MUTABLE REFERENCES and declared with the mut keyword");
    let mut point:(i32, i32) = (1, 2);
    let x_coord:&mut i32 = &mut point.0; // observe that the type annotation and the mutable
                                         // reference creation keyword are the same
    *x_coord = 20; // dereferencing the writeable mutable reference value
    println!("point: {point:?}");
    println!("These below are SLICES which borrow data from the original sliced type");
    let a_slice: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    let s: &[i32] = &a_slice[2..4]; // note the loop range generator can be used for slice indexes
                                    // as well with an inclusive start and exclusive end
    println!("s: {s:?}");
    println!("That also explains the 2 different string types:\n1. &str is a slice of UTF-8 encoded bytes, similar to &[u8].\n2. String is an owned buffer of UTF-8 encoded bytes, similar to Vec<T>.");
    println!("In other words, &str is borrowed as an immutable reference and is read-only while String is a writeable mutable reference to a string value.");

    let s1: &str = "World";
    println!("&str s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("String s2: {s2}");

    s2.push_str(s1);
    println!("String s2 after changing: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("&str s3: {s3}");
    println!("Parting notes regarding references are that references can NEVER be null and CANNOT outlive the data they point to.");
    println!("Meaning the below lines will be invalid.");
    // demonstrate_invalid_reference();

}

// struct definition

struct Person {
    name: String,
    age: u8,
} // named struct

struct Point(i32, i32); // unnamed struct

struct PoundsOfForce(f64); // newtypes defined as single-field wrappers for tuple structs
struct Newtons(f64); // newtypes defined as single-field wrappers for tuple structs

#[derive(Debug)]
enum Direction {
    Left,
    Right,
} 

#[derive(Debug)]
enum PlayerMove {
    Pass,                        
    Run(Direction),              
    Teleport { x: u32, y: u32 }, 
} // observe how literally anything can be made an enum, and things are called with the double
  // colon :: syntax

enum CarryableConcreteItem {
    Left,
    Right,
} // defined enum

type Item = CarryableConcreteItem; // type alias to make the long enum name shorter and easier to
                                   // type out

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn demonstrate_structs(){
    println!("Rust structs share the same dot syntax when altering struct values");
    println!("These below are NAMED structs where field names themselves are IMPORTANT!");
    let mut peter = Person { name: String::from("Peter"), age: 27 };
    describe(&peter);
    peter.age = 28; // struct value reassignment
    describe(&peter);
    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);
    let jackie = Person { name: String::from("Jackie"), ..avery };
    describe(&jackie);
    println!("These below are UNNAMED TUPLE structs, for when field names are UNIMPORTANT!");
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
    println!("This also conveniently allows for the creation of single-field tuple struct wrappers, known as NEWTYPES.");
    println!("Next is my favourite type, the ENUM");
    let player_move: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {player_move:?}");
    println!("Type aliases creates a name for another type. The two types can then be used interchangeably.");
    use std::cell::RefCell;
    use std::sync::{Arc, RwLock};
    type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;
}

// exercises

// but this probably works for all matrix sizes if not for the type signature
fn naive_transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix:[[i32; 3]; 3]= [[0; 3]; 3];
    let mut count:usize = 0;
    for row in matrix {
        let mut icount:usize = 0;
        for elem in row {
            new_matrix[icount][count] = elem;
            icount = icount + 1;
        }
        count = count + 1;
    }
    return new_matrix;
}

// hardcoded function since we know the function should only operate on 3x3 matrices anyway
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn magnitude(vector_coordinate:&[f64;3]) -> f64 {
    let mut total:f64 = 0.0;
    for i in 0..3 {
        total += vector_coordinate[i];
    }
    return total.sqrt();
}

fn normalize(vector_coordinate:&mut [f64;3]) {
    let magnitude:f64 =  magnitude(vector_coordinate);
    for i in 0..3 {
        let mut vectors:[f64; 3] = *vector_coordinate;
        vectors[i] /= magnitude;
    }
}

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    /// A button was pressed.
    ButtonPressed(Button),

    /// The car has arrived at the given floor.
    CarArrived(Floor),

    /// The car's doors have opened.
    CarDoorOpened,

    /// The car's doors have closed.
    CarDoorClosed,
}

/// A floor is represented as an integer.
type Floor = i32;

/// A direction of travel.
#[derive(Debug)]
enum Dir {
    Up,
    Down,
}

/// A user-accessible button.
#[derive(Debug)]
enum Button {
    /// A button in the elevator lobby on the given floor.
    LobbyCall(Dir, Floor),

    /// A floor button within the car.
    CarFloor(Floor),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Dir) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

// main function 

fn main() {

    println!("~~~ Comprehensive Rust: Day 1 Afternoon ~~~");

    // array and tuples

    render_array();
    render_tuple();
    // demonstrate_nested_loop();
    let input_tuple:(i32, i32) = (100, 1000);
    demonstrate_pattern_matching(input_tuple);

    // transpose exercise

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);

    // references and dereferencing
    
    demonstrate_references();

    // geometry exercise
    
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v:[f64; 3] = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v)); 

    // user-defined structs

    demonstrate_structs();

    // elevator events exercise

    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Dir::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));

}
