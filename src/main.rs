fn main() {
    println!("Hello, world!");
}

// create a function that takes a vector of u8 and returns a string
fn vec_to_string(v: Vec<u8>) -> String {
    // create a new string
    let mut s = String::new();
    // iterate over the vector
    for i in v {
        // push the character to the string
        s.push(i as char);
    }
    // return the string
    s
}