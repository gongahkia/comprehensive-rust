// helper functions

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!("add fizzbuzz")
}

fn execute_while_loop(mut x:i32){
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");
}

fn execute_for_loop(lower:i32, upper:i32){
    for x in lower..upper {
        println!("looping x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("looping elem: {elem}");
    }
}

fn execute_generic_loop(){
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}

fn demonstrate_break_and_continue(){
    let mut i:i32 = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}

fn demonstrate_labelled_break_and_continue(){
  let s: [[i32; 3]; 3] = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'ballsOuterLoop: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'ballsOuterLoop;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}

fn render_macros_explanation() {
    let print_ln_explanation:&str = "println!(format, ..) prints a line to standard output, applying formatting described in std::fmt.";
    let format_explanation:&str = "format!(format, ..) works just like println! but returns the result as a string.";
    let debug_explanation:&str = "dbg!(expression) logs the value of the expression and returns it.";
    let todo_explanation:&str = "todo!() marks a bit of code as not-yet-implemented. If executed, it will panic.";
    let unreachable_explanation:&str = "unreachable!() marks a bit of code as unreachable. If executed, it will panic.";
    println!("\n\n{print_ln_explanation}\n\n{format_explanation}\n\n{debug_explanation}\n\n{todo_explanation}\n\n{unreachable_explanation}\n\n");
}

// exercises

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n-1) + fib(n-2);
    }
}

fn collatz_length(mut n: i32) -> u32 {
    let mut len:u32 = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

// main function 

fn main() {

    println!("~~~ Comprehensive Rust: Day 1 Morning ~~~");

    let mut x: i32 = 10; // value can be changed since declared to be mutable
    println!("x: {x}");

    x = 20; // variable shadowing, note a variable's shadowing is limited by its block-delimtied
            // scope by default
    println!("x: {x}");

    println!("result 1: {}", interproduct(120, 100, 248));
    println!("result 2: {}", interproduct(0, 1, 2));

    // fibboncai exercise
    
    let n:u32 = 20;
    println!("fib({n}) = {}", fib(n));

    // gcd function

    let input1:u32 = 100;
    let input2:u32 = 25;
    println!("gcd: {}", gcd(input1, input2));

    // conventional if expressions are accepted, but so are in-line ones
    
    let mut z:i32 = 10;
    let mut size:&str = if z < 20 { "small" } else { "large" };
    println!("number size then: {}", size);
    z = 2000;
    size = if z < 20 { "small" } else { "large" };
    println!("number size now: {}", size);

    // similarly, match expressions are accepted in their expected format, as well as in-line
    
    let flag:bool = true;
    let mut val:i32 = match flag {
        true => 1,
        false => 0,
    };
    println!("The value of the {flag} is {val}");
    let updated_flag:bool= false;
    val = match updated_flag {
        true => 1,
        false => 0,
    };
    println!("The value of the {updated_flag} is {val}");

    // loops, break and continue demo
    
    execute_while_loop(50);
    execute_for_loop(10, 20);
    execute_generic_loop();
    demonstrate_break_and_continue();
    demonstrate_labelled_break_and_continue();

    // macros
        
    render_macros_explanation();
    let e:i32 = 4;
    let f:u32 = 5;
    // println!("{n}! = {}", factorial(n)); 
    // println!("fizzbuz(f) = {}", fizzbuzz(f))

    // collatz sequence exercise
    
    println!("Length: {}", collatz_length(11)); 

}
