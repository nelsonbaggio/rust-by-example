use std::fmt;

fn main() {
    
    println!("My name is {0}, {1} {0}", "Bond", "James");
    
    #[derive(Debug)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({})", self.0)
        }
    }
    
    println!("This struct `{:#?}` won't print...", Structure(3));
    println!("This struct `{}` won't print...", Structure(3));
    
    let pi = 3.141592;
    println!("Pi is roughly {0:.3}", pi);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    println!("Display: {}", Complex { real: 3.3, imag: 7.2 });
    println!("Debug: {:?}", Complex { real: 3.3, imag: 7.2 });
}
