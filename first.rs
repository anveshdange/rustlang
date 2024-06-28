#![allow(unused)]

// importing the modules 
use std::io; 

// main function driver code 
fn main() {
    println!("Hello World");
    let p = format!("My name is {name}.", name="Anvesh Dange");
    println!("My name is {}", p); 

    println!("Base 10: {}", 100); 
    println!("Base 2 (binary): {:b}", 100);
    println!("Base 8 (octal): {:o}", 100);
    println!("Base 16 (hexadecimal): {:x}", 100); 

    println!("{number:>5}", number=1);
    println!("{number:p>5}", number= 1);
    println!("{number:p<5}", number=1);
    println!("{number:0<width$}", number=1, width=10);
    println!("{number:0>width$}", number=1, width=10);
    
    let a : &str = "Anvesh" ;
    let b : &str = "Dange";
    println!("My name is {0} {1} !!", a, b);

    // Alowing dead code
    #[allow(dead_code)] 
    struct Structure(i32); 
    
    let number: f64 = 1.0; 
    let width: usize = 5;
    println!("{number:>width$}");

}

