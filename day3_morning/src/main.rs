// helper functions

fn render_revision_on_memory(){

    println!("The Comprehensive Rust book does this better than I ever could so I'm better off just getting it straight from the source: https://google.github.io/comprehensive-rust/memory-management/review.html");
    println!("While programming languages have traditionally chosen one of two camps, either taking the path of FULL control of memory with C, C++ and Pascal, or FULL memory safety by automating it away with runtime garbage collection with Java, Python, Go and Haskell, Rust chooses a HYBRID approach that prioritises full control and safety via compile time enforcement of correct memory management. This is achieved through an explicit ownership concept.");

    println!("\n~To break down the concepts further~\n\n1.Every rust value has only ONE owner at any time, determined by its scope (deliniated by the number of curly braces around a given variable)\n");

    struct Point(i32, i32);
    {
        let p:Point  = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1); 
    println!("The above line would emit an error if run!");

    println!("\n2. Assignemnt transfers value ownership between variables\n");

    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");

    println!("\n3. To merely COPY and obtain the value of a variable (but not the exact pointer), we can just use .clone() also known as the CLONE TRAIT\n");

    fn say_hello(name: String) {
        println!("Hello {name}")
    }

    let name:String = String::from("Alice");
    say_hello(name.clone());
    say_hello(name);

    println!("\n4. By default, Rust implements the MOVE semantic so that variables are reassigned. However, certain types are the exception to this rule and are instead COPIED by default. Further, it is also possible to OPT in to use the copy trait for any type simply be specifying it with '#[derive(Copy, Clone, Debug)]'\n");

    let x = 42;
    let y = x;
    println!("x: {x}"); 
    println!("y: {y}");

    #[derive(Copy, Clone, Debug)]
    struct Point2(i32, i32);
    let p1 = Point2(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    println!("\n5. Rust also has the unqiue trait DROP, whcih allows values that implement to specify special code that runs when those DROPPED values go out of scope (and are literally dropped)\n");

    struct Droppable {
        name: &'static str,
    }
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Dropping {}", self.name);
        }
    }
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");

}

#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

struct PackageBuilder(Package);

impl PackageBuilder {

    fn new(name: impl Into<String>) -> Self {
        PackageBuilder(Package {
            name: name.into(),
            version: String::new(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            language: None,
        })
    }

    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    fn build(self) -> Package {
        self.0
    }

}

fn render_smart_pointers(){
    println!("In line with its emphasis on memory safety, Rust has a really clever way of managing pointers with SMART POINTERS");
    println!("For more details on this subtopic, see here: https://google.github.io/comprehensive-rust/smart-pointers.html");
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

impl<T: Ord> Subtree<T> {

    fn new() -> Self {
        Subtree(None)
    }

    fn insert(&mut self, value: T) {
        match &mut self.0 {
            Some(node) => {
                if value < node.value {
                    node.left.insert(value);
                } else if value > node.value {
                    node.right.insert(value);
                }
            }
            None => {
                self.0 = Some(Box::new(Node {
                    value,
                    left: Subtree::new(),
                    right: Subtree::new(),
                }));
            }
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            Some(node) => {
                if &node.value == value {
                    true
                } else if value < &node.value {
                    node.left.has(value)
                } else {
                    node.right.has(value)
                }
            }
            None => false,
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            Some(node) => 1 + node.left.len() + node.right.len(),
            None => 0,
        }
    }

}

// main function

fn main() {

    println!("~~~ Comprehensive Rust: Day 3 Morning ~~~");

    // memory
    
    render_revision_on_memory();

    // builder type exercise

    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log =
        PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");

    // smart pointers
    
    render_smart_pointers();

    // binary tree exercise
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn len() {
            let mut tree = BinaryTree::new();
            assert_eq!(tree.len(), 0);
            tree.insert(2);
            assert_eq!(tree.len(), 1);
            tree.insert(1);
            assert_eq!(tree.len(), 2);
            tree.insert(2); // not a unique item
            assert_eq!(tree.len(), 2);
        }

        #[test]
        fn has() {
            let mut tree = BinaryTree::new();
            fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
                let got: Vec<bool> =
                    (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
                assert_eq!(&got, exp);
            }

            check_has(&tree, &[false, false, false, false, false]);
            tree.insert(0);
            check_has(&tree, &[true, false, false, false, false]);
            tree.insert(4);
            check_has(&tree, &[true, false, false, false, true]);
            tree.insert(4);
            check_has(&tree, &[true, false, false, false, true]);
            tree.insert(3);
            check_has(&tree, &[true, false, false, true, true]);
        }

        #[test]
        fn unbalanced() {
            let mut tree = BinaryTree::new();
            for i in 0..100 {
                tree.insert(i);
            }
            assert_eq!(tree.len(), 100);
            assert!(tree.has(&50));
        }
    }

}
