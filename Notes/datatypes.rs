//Data Types
//Author: Kaiden Mix

//Notes:

//Rust is a statically typed languasge, meaning that it must know all 
//variables at compile time.

//Rather than using something like [ int var = 1 ], We must state the type
//and bits to store in memory. For example i32 denotes that we want a signed
//integer that uses 32 bits to store values.
fn main(){
	//Signed Integers can represent both positive and negative numbers
	let x: i32 = -42;
	//Unsigned Integers can only represent non-negative numbers
	let y: u64 = 100;
	println!("Signed Integer: {}", x);
	println!("Unsigned Integer: {}", y);

	//Floats
	// Only two float types, f32 and f64
	let pi: f64 = 3.14;
	println!("Value of pi: {}", pi);

	//Boolean Values: True, False
	let is_snowing: bool = true;
	println!("Is it snowing? {}", is_snowing);

	//Character Type - char
	let letter: char = 'a';
	println!("First letter of the alphabet: {}", letter);

	//	Compound Data Types
	//Comprised of arrays, tulpes, slices, and strings (slice string)

	//	Arrays
	//Must be comprised of one data, an array ONLY of integers, or ONLY of strings.
	//In rust, when setting an array we must put [Data type; Size]
	let numbers: [i32; 5] = [1,2,3,4,5];
	println!("Number Array: {:?}", numbers);
	let fruits: [&str; 3] = ["Apples", "Banana", "Orange"];
	println!("Fruit Array: {:?}", fruits);

	//Procedure to show only a certain index in an array
	println!("Fruit Array: 1st Element: {}", fruits[0]);
	println!("Fruit Array: 3rd Element: {}", fruits[2]);

	//	Tuples
	//In rust, a tuple is a collection of values of different types, where arrays can
	//only store groups of a single type. Tuples can group values that have a variety
	//of types.
	let human: (String, i32, bool) = ("Solid Snake".to_string(), 27, true);

	//There are two types of strings as you may have noticed. &str and String. &str is
	//a string slice while String is a string datatype. We can convert slices to string 
	//by using .to_string()
	println!("Description of main character from metal gear: {:?}", human);

	let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
	println!("My Mix Tuple: {:?}", my_mix_tuple);

	//	Slices:
	//
	let number_slices:&[i32] = &[1,2,3,4,5];
	println!("Number Slices: {:?}", number_slices);

	let animal_slices:&[&str] = &["Lion","Crocodile","Elephant"];
	println!("Animal Slices: {:?}", animal_slices);

	let book_slices:&[&String] = &[&"Game of Thrones".to_string(),&"Dune".to_string(),&"In Cold Blood".to_string()];
	println!("Book Slices: {:?}", book_slices);

	//	Strings (String) vs String Slices (&str)
	//Strings are growable, you can increase and decrease them. Also mutable. They are also owned
	//&str is a reference with a fixed size.
	// You should use &str when passing read-only strings, and String when you need ownership
	//and mutability

	//Adding mut allow it to be mutatable
	let mut boss: String = String::from("Kept you");
	//.push_str lets us add data to the end.
	boss.push_str(" waiting huh?");
	println!("Big Boss says {}", boss);


	//&str
	let string: String = String::from("Hello, World"); //We created an own string 
	let slice: &str = &string;  //We are now borrowing the string as a string slice (&str)
	println!("Slice Value: {}", slice);
	let sliced: &str = &string[0..5];
	println!("Sliced Value: {}", sliced);

}