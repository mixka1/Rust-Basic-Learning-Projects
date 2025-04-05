//Ownership, Borrowing and References
//Author: Kaiden Mix

//Ownership
// Ownership introduced by Rust to solve memory safety issues and 
//high-peformance at the same time
// Every value has a single owner [Every variable has on value, and 
//it is its sole owner].
//
//  Ownership Rules
//1. Each value in Rust has an owner
//2. There can only be one owner at a time.
//3. When the owner goes out of scope, the value is dropped.


// Example: Each value in rust has a variable that's its owner.
fn main(){
    let e1 = String::from("RUST"); //s1 is the owner of the string "RUST"
    let len = calculate_length(&e1); //Passing a reference to that owner with &
    println!("Length of '{}' is {}.", e1, len);
    example2();
    example3();
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

// Example: There can only be one owner at a time.
fn example2(){
    let e2 = String::from("RUST For Example 2");
    let s2 = e2;
    //println!("{}",e2); //This would cause an error because e2 no longer owns the string
    println!("{}",s2);
}

//Example: When the owner goes out of scope the value will be dropped.
fn example3(){
    let e3 = String::from("RUST For Example 3");
    let s3 = calculate_length(&e3);
    println!("Length of '{}' is {}.", e3, s3);
}

//fn printLost(2: &string){
    //println!("{}", &e3); //Generates not found in scope error.
//}