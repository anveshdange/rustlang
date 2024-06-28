use std::fmt; // Import the "fmt" module 

// Define a structure named "List" containing a "Vec"
#[derive(Debug)] 
struct List(Vec<i32>); 

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing 
        // and create a reference a `vec` 
        let vec = &self.0;
        write!(f, "["); 

        // iterate over `v` in `vec` while enumerating the iteration 
        // count in `count` 
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{} ",count ,v)?;
        }
        write!(f, "]")

    }
}

fn main() {
    let v = List(vec![1,2,3]); 
    println!("This is the Display View: {}", v);
    println!("This is the Debug View: {:?}", v);
}