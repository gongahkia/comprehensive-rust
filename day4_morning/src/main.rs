// helper functions

fn demonstrate_iterators(){

    println!("Rust's equivalent of loops (loop, while, while let, for, labelled block expressions: https://doc.rust-lang.org/reference/expressions/loop-expr.html#labelled-block-expressions) are all powered by pure iterators under the hood.");
    println!("Going any deeper would risk missing the forest for the trees so I'll stop here but suffice to say there's a lot more to be said about Rust's iterator trait as seen from here: https://google.github.io/comprehensive-rust/iterators/iterator.html");
    
    struct SliceIter<'s> {
        slice: &'s [i32],
        i: usize,
    }
    impl<'s> Iterator for SliceIter<'s> {
        type Item = &'s i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.i == self.slice.len() {
                None
            } else {
                let next = &self.slice[self.i];
                self.i += 1;
                Some(next)
            }
        }
    }
    let slice = [2, 4, 6, 8].as_slice();
    let iter = SliceIter { slice, i: 0 };
    for elem in iter {
        println!("elem: {elem}");
    }

    println!("There are also many helper methods for iterators as seen from here: https://google.github.io/comprehensive-rust/iterators/helpers.html");

    println!("The collect method conveniently allows you to generate a collection from an Iterator.");
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");

    println!("While Iterator tells you how to iterate once you have created an iterator, the trait IntoIterator defines how to create an iterator for a type.");

    struct Grid {
        x_coords: Vec<u32>,
        y_coords: Vec<u32>,
    }

    impl IntoIterator for Grid {
        type Item = (u32, u32);
        type IntoIter = GridIter;
        fn into_iter(self) -> GridIter {
            GridIter { grid: self, i: 0, j: 0 }
        }
    }

    struct GridIter {
        grid: Grid,
        i: usize,
        j: usize,
    }

    impl Iterator for GridIter {
        type Item = (u32, u32);

        fn next(&mut self) -> Option<(u32, u32)> {
            if self.i >= self.grid.x_coords.len() {
                self.i = 0;
                self.j += 1;
                if self.j >= self.grid.y_coords.len() {
                    return None;
                }
            }
            let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
            self.i += 1;
            res
        }
    }

    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }

}

fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    let len = values.len();
    if len == 0 {
        return vec![]; 
    }
    let mut result = Vec::with_capacity(len);
    for n in 0..len {
        let wrapped_index = (n + offset) % len;
        result.push(values[wrapped_index] - values[n]);
    }
    result
}

fn demonstrate_modules(){

    println!("The keyword mod lets us namespace types and functions.");

    mod foo {
        pub fn do_something() {
            println!("In the foo module");
        }
    }
    mod bar {
        pub fn do_something() {
            println!("In the bar module");
        }
    }
    foo::do_something();
    bar::do_something();

    println!("Meanwhile, omitting the module's content while specify the keyword mod will tell Rust to look for that unspecified module in another file.");

    println!("Note that modules also provide a privacy boundary, where module items are private by default with their implementation details hidden. However, do note that parent and sibling items are always visible.\n\nEffectively, this means that if an item is visible in module foo, it's visible in all the descendants of foo.");

    mod outer {
        fn private() {
            println!("outer::private");
        }

        pub fn public() {
            println!("outer::public");
        }

        mod inner {
            fn private() {
                println!("outer::inner::private");
            }

            pub fn public() {
                println!("outer::inner::public");
                super::private();
            }
        }
    }
    outer::public();

    println!("In the same way, note that struct fields are also private by default. Private fields are likewise visible within the rest of the module (including child modules). This allows us to encapsulate implementation details of struct, controlling what data and functionality is visible externally.");

    use front::Foo;
    mod front {
        pub struct Foo {
            pub val: i32,
            is_big: bool,
        }

        impl Foo {
            pub fn new(val: i32) -> Self {
                Self { val, is_big: val > 100 }
            }
        }

        pub mod behind {
            use super::Foo;

            pub fn print_foo(foo: &Foo) {
                println!("Is {} big? {}", foo.val, foo.is_big);
            }
        }
    }
    let foo = Foo::new(42);
    println!("foo.val = {}", foo.val);
    front::behind::print_foo(&foo);

    println!("Symbols from another module can be brought into scope with the keyword use. ");

}

fn demonstrate_tests(){
    
    println!("Rust's unit test framework is relatively easy to understand.\n\nTests are marked with #[test]. Unit tests are often put in a nested tests module, using #[cfg(test)] to conditionally compile them only when building tests.");

    fn first_word(text: &str) -> &str {
        match text.find(' ') {
            Some(idx) => &text[..idx],
            None => &text,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_empty() {
            assert_eq!(first_word(""), "");
        }

        #[test]
        fn test_single_word() {
            assert_eq!(first_word("Hello"), "Hello");
        }

        #[test]
        fn test_multiple_words() {
            assert_eq!(first_word("Hello World"), "Hello");
        }
    }

    println!("Rust also provisions for other kinds of tests, such as integration tests and documentation tests as seen from here: https://google.github.io/comprehensive-rust/testing/other.html");

    println!("Where tests fail, Rust has got your back with Compiler Linting and the fantastic error messages provided by Clippy: https://google.github.io/comprehensive-rust/testing/lints.html");

}

pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    for c in cc_number.chars().filter(|&c| c != ' ').rev() {
        if let Some(digit) = c.to_digit(10) {
            let mut value = digit;
            if double {
                value *= 2;
                if value > 9 {
                    value -= 9; 
                }
            }
            sum += value;
            double = !double; 
        }
    }
    sum % 10 == 0
}

// main execution code

fn main() {

    println!("~~~ Comprehensive Rust: Day 4 Morning ~~~");

    // iterators
    
    demonstrate_iterators();
    
    // iterator method chaining exercise
    
    #[test]
    fn test_offset_one() {
        assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
        assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
        assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
    }

    #[test]
    fn test_larger_offsets() {
        assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
        assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
        assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
        assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    }

    #[test]
    fn test_degenerate_cases() {
        assert_eq!(offset_differences(1, vec![0]), vec![0]);
        assert_eq!(offset_differences(1, vec![1]), vec![0]);
        let empty: Vec<i32> = vec![];
        assert_eq!(offset_differences(1, empty), vec![]);
    }

    // modules
    
    demonstrate_modules();

    // tests
    
    demonstrate_tests();

    // luhn algorithm exercise
    
    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_valid_cc_number() {
            assert!(luhn("4263 9826 4026 9299"));
            assert!(luhn("4539 3195 0343 6467"));
            assert!(luhn("7992 7398 713"));
        }

        #[test]
        fn test_invalid_cc_number() {
            assert!(!luhn("4223 9826 4026 9299"));
            assert!(!luhn("4539 3195 0343 6476"));
            assert!(!luhn("8273 1232 7352 0569"));
        }
    }

}
