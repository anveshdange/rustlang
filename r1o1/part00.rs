
// ALGEBRAIC DATATYPE IN RUST 
// The datatype of multiple exclusive options is called an "Algebraic datatype"
// Rust has such a type called Enum to store such datatypes 

// A enum for "a number or nothing" could look as follows: 
enum NumberorNothing {
    Number(i32), // i32 is type of signed 32 bit integer 
    Nothing
}

use self::NumberorNothing::{Number, Nothing};

// Function to calculate the minimum of a vector 
fn vec_min(vec: Vec<i32>) -> NumberorNothing {
    let mut min = Nothing; 
    for e1 in vec {
        match min {
            Nothing => {min = Number(e1);}, 
            Number(n) => {
                let new_min = min_i32(n, e1);
                min = Number(new_min);
            }
        }
    }
    return min;
}

// Function that computes the minimum of two numbers 
fn min_i32(a:i32 ,b:i32) -> i32 {
    if a<b {return a;}
    else {return b;}
}

// !vec is a macro that constructs a constant Vec<_> with the given elements. 
fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 1, 9, 27]
}

// Function that prints number or nothing 
fn print_number_or_nothing(m: NumberorNothing) {
    match m {
        Nothing => {println!("The number is: <Nothing>");}, 
        Number(n) => {println!("The number is: {}", n);}
    };
}

// writing the main function 
fn main() {
    let vec = read_vec(); 
    let min = vec_min(vec); 
    print_number_or_nothing(min);
}


