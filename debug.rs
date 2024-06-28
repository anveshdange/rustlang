#![allow(unused)]
#![allow(non_snake_case)]
// Derive the `fmt::Debug` implementation for `Structure`
// Structure is a structure that contains a single i32 
#[derive(Debug)] 
struct Structure(i32); 

// The `derive` attribute automatically created the 
// implementation required to make this `struct` printable with `fmt::Debug`
#[derive(Debug)] 
struct Deep(Structure); 

#[derive(Debug)] 
struct Person<'a> {
    name: &'a str, 
    age: u8
}


// Function to print type of variables 
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// Tesing out the generic types 
fn lol<T>(_:&T){
    println!("This is a test of generic type");
}

// Function to increment a counter 
fn increment_counter(counter: &mut usize){
    *counter += 1; // incrementing the pointer by 1
}


// The main function 
fn main() {
    let mut COUNTER: usize = 0;
    // printing with the `{:?}` is similar to with the `{]` 
    println!("{:?} months in a year,", 12);
    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:#?} will print!", Structure(3)); 

    let name = "Peter" ; 
    let age = 27; 
    let peter = Person{name, age};

    // pretty print 
    println!("{:#?}", peter);
    print_type_of(&peter);
    lol(&peter); increment_counter(&mut COUNTER); 
    lol(&name) ; increment_counter(&mut COUNTER);
    lol(&age)  ; increment_counter(&mut COUNTER);
    println!("Final Value of Counter: {}", COUNTER);
}

