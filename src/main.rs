fn main() {
    println!("Hello, world!");

    // call below functions
    print_numbers();
    print_numbers_for();
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

// create a struct that holds a uid and a name
struct User {
    uid: u32,
    name: String,
}


// a function to instantiate the struct
fn create_user(uid: u32, name: String) -> User {
    User {
        uid,
        name,
    }
}

// create an if/else statement
fn check_user(user: User) {
    if user.uid == 0 {
        println!("This is the root user");
    } else {
        println!("This is a regular user");
    }
}


// create a loop that prints the numbers 1-10 using loop   
fn print_numbers() {
    let mut i = 1;
    loop {
        println!("{}", i);
        i += 1;
        if i > 10 {
            break;
        }
    }
}


// create a for loop that prints the numbers 1-10 using for
fn print_numbers_for() {
    for i in 1..11 {
        println!("{}", i);
    }
}
