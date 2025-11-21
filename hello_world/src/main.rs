#![allow(unused)]

use std::fmt;
fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean");

    //Write formatted text to String
    let some_text = format!("Hello");

    // Print to the standard output io::stdout
    println!("{}", some_text);
    print!("{} \n", some_text);

    //Print to io::stderr
    eprint!("Error \n");
    eprintln!("Beep!");

    // Positional arguments
    println!("Hello {1} meet {0}. {0} meet {1}", "John", "Brian");

    // Named arguments
    println!(
        "My name is {name} and I am {years} old",
        name = "Brian",
        years = 23
    );
    // Formatting integers
    println!("Base 10 {}", 255);
    println!("Binary: {:b}", 255);
    println!("Octal: {:o}", 255);
    println!("Hexadecimal: {:x}", 255);

    // Right justify text
    // Outputs 4 white spaces and then 1
    println!("{number:>5}", number = 1);

    // Padding numbers with extra zeroes

    println!("{number:0<5}", number = 1);
    println!("{number:0>5}", number = 1);

    // Named argument in the format specifier // Same as above

    println!("{number:0>width$}", number = 1, width = 5);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Structure(i32);

    let num = Structure(3);

    println!("{:?}", num);

    let pi = 3.141592;

    println!("{pi:.3}");

    // Debug
    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?}", Deep(Structure(7)));

    println!("{:?}", Structure(3));

    // Printing with {:?}
    println!("Months is a year {:?}", 12);

    // Implementing Display
    struct SomeStructure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}
