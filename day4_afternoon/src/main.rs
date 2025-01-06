#![allow(unused_imports, unused_variables, dead_code)]

// helper functions

fn demonstrate_error_handling(){

    println!("PANIC: Rust handles fatal errors with a panic. More details can be found here: https://google.github.io/comprehensive-rust/error-handling/panics.html");

    println!("RESULT: The primary mechanism for error handling in Rust. More details can be found here: https://google.github.io/comprehensive-rust/error-handling/result.html");

    use std::fs::File;
    use std::io::Read;

    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Dear diary: {contents} ({bytes} bytes)");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }

    println!("?: Also known as the TRY-OPERATOR, it is used to return errors to the caller. More details can be found here: https://google.github.io/comprehensive-rust/error-handling/try.html\nAlso be aware of try conversions: https://google.github.io/comprehensive-rust/error-handling/try-conversions.html");

    use std::{fs, io};

    fn read_username(path: &str) -> Result<String, io::Error> {
        let username_file_result = fs::File::open(path);
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(err) => Err(err),
        }
    }
    let username = read_username("config.dat");
    println!("username or error: {username:?}");

    println!("STD::ERROR::ERROR: Allows for the creation of dynamic error types where the specific kind of generated error is left unspecified but the code is nonetheless typesafe. More details can be found here: https://google.github.io/comprehensive-rust/error-handling/error.html");

    use std::error::Error;

    fn read_count(path: &str) -> Result<i32, Box<dyn Error>> {
        let mut count_str = String::new();
        fs::File::open(path)?.read_to_string(&mut count_str)?;
        let count: i32 = count_str.parse()?;
        Ok(count)
    }
    fn main() {
        fs::write("count.dat", "1i3").unwrap();
        match read_count("count.dat") {
            Ok(count) => println!("Count: {count}"),
            Err(err) => println!("Error: {err}"),
        }
    }

    println!("THISERROR: A crate that provides macros to help avoid boilerplate when defining error types by providing derived macros that assist in implementing From<T>, Display, and the Error trait. More details can be found here: https://google.github.io/comprehensive-rust/error-handling/thiserror.html");

    use thiserror::Error;

    #[derive(Debug, Error)]
    enum ReadUsernameError {
        #[error("I/O error: {0}")]
        IoError(#[from] io::Error),
        #[error("Found no username in {0}")]
        EmptyUsername(String),
    }
    fn read_the_username_of_pookie(path: &str) -> Result<String, ReadUsernameError> {
        let mut username = String::with_capacity(100);
        fs::File::open(path)?.read_to_string(&mut username)?;
        if username.is_empty() {
            return Err(ReadUsernameError::EmptyUsername(String::from(path)));
        }
        Ok(username)
    }
    match read_the_username_of_pookie("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }

    println!("ANYHOW: A crate that provides a rich error type with support for carrying additional contextual information, which can be used to provide a semantic trace of what the program was doing leading up to the error. More details can be found here: https://google.github.io/comprehensive-rust/error-handling/anyhow.html");

    use anyhow::{bail, Context, Result};
    use thiserror::Error;

    #[derive(Clone, Debug, Eq, Error, PartialEq)]
    #[error("Found no username in {0}")]
    struct EmptyUsernameError(String);
    fn read_my_username(path: &str) -> Result<String> {
        let mut username = String::with_capacity(100);
        fs::File::open(path)
            .with_context(|| format!("Failed to open {path}"))?
            .read_to_string(&mut username)
            .context("Failed to read")?;
        if username.is_empty() {
            bail!(EmptyUsernameError(path.to_string()));
        }
        Ok(username)
    }
    match read_my_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
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

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

fn eval(e: Expression) -> Result<i64, DivideByZeroError> {
    match e {
        Expression::Op { op, left, right } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            match op {
                Operation::Add => Ok(left + right),
                Operation::Sub => Ok(left - right),
                Operation::Mul => Ok(left * right),
                Operation::Div => {
                    if right != 0 {
                        Ok(left / right)
                    } else {
                        Err(DivideByZeroError)
                    }
                },
            }
        }
        Expression::Value(v) => Ok(v),
    }
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(DivideByZeroError)
    );
}

#[test]
fn test_valid_operations() {
    assert_eq!(eval(Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    }).unwrap(), 30);

    assert_eq!(eval(Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    }).unwrap(), 10);

    assert_eq!(eval(Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(5)),
        right: Box::new(Expression::Value(6)),
    }).unwrap(), 30);

    assert_eq!(eval(Expression::Op {
        op: Operation::Div,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(5)),
    }).unwrap(), 4);
}

fn demonstrate_unsafe_rust(){

    println!("The topic of what's considered unsafe rust is broa d and extensive, and best understood through direct application. To keep it brief, these are the most common issues that arise due to unsafe rust:\n\t1. Dereferencing Raw Pointers\n\t2. Mutable Static Variables\n\t3. Unions\n\t4. Unsafe functions\n\t5. Unsafe traits");
    println!("More details can be found here: https://google.github.io/comprehensive-rust/unsafe-rust.html");

}

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    unsafe extern "C" {
        pub unsafe fn opendir(s: *const c_char) -> *mut DIR;
        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;
        pub unsafe fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        let c_path = CString::new(path).map_err(|e| e.to_string())?;
        let dir_ptr = unsafe { ffi::opendir(c_path.as_ptr()) };
        if dir_ptr.is_null() {
            Err(format!("Failed to open directory '{}'", path))
        } else {
            Ok(DirectoryIterator { path: c_path, dir: dir_ptr })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        unsafe {
            let entry = ffi::readdir(self.dir);
            if entry.is_null() {
                return None; 
            }
            let dirent = &*entry;
            let name_slice = CStr::from_ptr(dirent.d_name.as_ptr());
            Some(name_slice.to_os_string())
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        unsafe {
            ffi::closedir(self.dir);
        }
    }
}

// main execution code

fn main() -> Result<(), String> {

    println!("~~~ Comprehensive Rust: Day 4 Afternoon ~~~");

    // error handling
    
    demonstrate_error_handling();
    
    // rewriting with result exercise
    
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    };
    println!("expr: {expr:?}");
    match eval(expr) {
        Ok(result) => println!("result: {:?}", result),
        Err(_) => println!("Error occurred during evaluation."),
    }

    // unsafe rust
    
    demonstrate_unsafe_rust();

    // safe ffi wrapper exercise

    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(());

}
