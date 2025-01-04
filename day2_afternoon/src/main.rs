// imports

use std::io::Read;
use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;

// helper functions

fn demonstrate_generics(){

    println!("Whether we like it or not, we have to talk about Rust generics, so the least I can do is make this as painless as possible.");
    println!("In a nutshell, generics allow you to avoid specifying the exact type signature of a function but allow its execution anyway in a more generic fashion.");

    fn pick<T>(n: i32, even: T, odd: T) -> T {
        if n % 2 == 0 {
            even
        } else {
            odd
        }
    }

    println!("picked a number: {:?}", pick(97, 222, 333));
    println!("picked a string: {:?}", pick(28, "dog", "cat"));

    println!("Bringing this one step further, traits themselves can also be generics!");

    #[derive(Debug)]
    struct Foo(String);

    impl From<u32> for Foo {
        fn from(from: u32) -> Foo {
            Foo(format!("Converted from integer: {from}"))
        }
    }

    impl From<bool> for Foo {
        fn from(from: bool) -> Foo {
            Foo(format!("Converted from bool: {from}"))
        }
    }

    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    println!("{from_int:?}, {from_bool:?}");

    println!("Recalling traits from the morning lecture, you can further require your types to implement some trait to call those trait's methods with the : colon syntax.");

    fn duplicate<T: Clone>(a: T) -> (T, T) {
        (a.clone(), a.clone())
    }

    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    println!("Also note that impl Trait syntax can be used within function arguments and return values to work with types that you cannot explicitly name.");

    fn add_42_millions(x: impl Into<i32>) -> i32 {
        x.into() + 42_000_000
    }

    fn pair_of(x: u32) -> impl std::fmt::Debug {
        (x + 1, x - 1)
    }

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(27);
    println!("debuggable: {debuggable:?}");

    println!("Something else interesting that might be relevant to look into in the future is the implementation of DYNAMIC dispatch of traits via generics.");

    struct Dog {
        name: String,
        age: i8,
    }
    struct Cat {
        lives: i8,
    }

    trait Pet {
        fn talk(&self) -> String;
    }

    impl Pet for Dog {
        fn talk(&self) -> String {
            format!("Woof, my name is {}!", self.name)
        }
    }

    impl Pet for Cat {
        fn talk(&self) -> String {
            String::from("Miau!")
        }
    }

    fn generic(pet: &impl Pet) {
        println!("Hello, who are you? {}", pet.talk());
    }

    fn dynamic(pet: &dyn Pet) {
        println!("Hello, who are you? {}", pet.talk());
    }

    let cat = Cat { lives: 9 };
    let dog = Dog { name: String::from("Fido"), age: 5 };

    generic(&cat);
    generic(&dog);

    dynamic(&cat);
    dynamic(&dog);

}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) | Some(Ordering::Equal) => a,
        _ => b,
    }
}

fn cover_rust_standard_library(){
    println!("For brevity's sake, I won't be going too deep into Rust's standard library (specifically its types or traits) for now. However, they will be important to look into just for breadth of knwowledge, so the relevant links have been added below.");
    println!("Rust's standard library TYPES: https://google.github.io/comprehensive-rust/std-types.html");
    println!("Rust's standard library TRAITS: https://google.github.io/comprehensive-rust/std-traits.html");
}

struct Counter<T: Eq + Hash> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {

    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    fn count(&mut self, value: T) {
        *self.values.entry(value).or_insert(0) += 1;
    }

    fn times_seen(&self, value: &T) -> u64 {
        self.values.get(value).copied().unwrap_or_default()
    }

}

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut bytes_read = self.input.read(buf)?;

        for i in 0..bytes_read {
            buf[i] = self.rot13(buf[i]);
        }

        Ok(bytes_read)
    }
}

impl RotDecoder<()> {
    fn rot13(byte: u8) -> u8 {
        match byte {
            b'a'..=b'z' => (byte - b'a' + 13) % 26 + b'a',
            b'A'..=b'Z' => (byte - b'A' + 13) % 26 + b'A',
            _ => byte,
        }
    }
}

// execution code

fn main() {

    println!("~~~ Comprehensive Rust: Day 2 Afternoon ~~~");

    // generics
    
    demonstrate_generics();

    // generic min function exercise

    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");

    // standard library
    
    cover_rust_standard_library();

    // counter exercise

    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));

    // rot13 cipher exercise

    let mut rot = RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn joke() {
            let mut rot = RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
            let mut result = String::new();
            rot.read_to_string(&mut result).unwrap();
            assert_eq!(&result, "To get to the other side!");
        }

        #[test]
        fn binary() {
            let input: Vec<u8> = (0..=255u8).collect();
            let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
            let mut buf = [0u8; 256];
            assert_eq!(rot.read(&mut buf).unwrap(), 256);
            for i in 0..=255 {
                if input[i] != buf[i] {
                    assert!(input[i].is_ascii_alphabetic());
                    assert!(buf[i].is_ascii_alphabetic());
                }
            }
        }
    }
