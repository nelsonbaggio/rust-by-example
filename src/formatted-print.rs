use std::fmt;

fn main() {
    #[allow(dead_code)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({})", self.0)
        }
    }

    
    println!("My name is {0}, {1} {0}", "Bond", "James");
    println!("This struct `{}` won't print...", Structure(3));
    
    let pi = 3.141592;
    println!("Pi is roughly {0:.3}", pi);
}
