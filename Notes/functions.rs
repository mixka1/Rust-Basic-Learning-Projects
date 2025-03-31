//Functions
//Author: Kaiden Mix

//Notes:

//In Rust, the main function is the entry point for the program,
//it is called automatically when the program starts. Every 
//program must have a main function unless its a library.

fn main() {
    greet();
    tell_height(182);
    human_id("Joel", 55, 182.0);
    let _math = {
        let price = 5;
        let qty = 10;
        price * qty  //Notice there is no semicolon here, when an expression
                     // end in a mathmatical operator, you can remove the semicolon 
                     //as long as it is the last line in the expression. You can also just do
                     // return price * qty; as well if you really wanted to
    };
    println!("Result is: {}", _math);
    let y: i32 = add(4,6);
    println!("Result is: {}", y);
}

//Regular functions do not exectute automatically. 
//They must be called explicitly
fn greet() {
    println!("Hello from the greet function!");
}

//Inserting input values
fn tell_height(height: u32){
    println!("My height is {}", height)
}

//Inputting more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}

//Returns a value
fn add(a: i32, b: i32) -> i32 {
    a + b
}
