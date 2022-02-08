// Hey! I'm learning rust! I started learning on new years of 2022 (1/1/22)
// This is basically going to be a really big commment file that's basically a guide to rust stuff as I go through rustlings!

fn main() {
	let mut year: i32 = 15; 
	println!("Year is currently {}", year);
		year = 16;
		println!("Year is currently {}", year);
	// let = var creation
	// mut = mutable, this var can be changed later (default = off)
	// year = var name
	// : i32    = year will be a 32 bit signed int (see std::* modules)
		// i8 | i16 | i32 | i64 | i128  || Signed
		// u8 | u16 | u32 | u64 | u128  || un-Signed 
		// na |	na	| f32 | f64  		|| Floating Point Nums 
		

	let words: &str = "Hello!";
		println!("{}", words); // essentially the same as println(words)
		
	let myArray = vec![1, 2, 3, 4];
		println!("{}", myArray.len());
	// creates an array of variable but predef size 
	// also initializes it with premade varsage
	// .len() = will return the length of the array

	let mut myArray2 = Vec::new(); // init var array with no defined size
		myArray2.push(192); // add the num 192 to myArray[0]
		myArray2.push(128); // add the num 128 to myArray[1]
		println!("{:?}", myArray2); // prints the output of that variable || Difference with :#? and :?

	let myArray3 = vec![0; 5]; // Fill array of certain length with charcter 
		// 0 = character to fill the array with
		// 5 = the length 
		println!("{:?}", myArray3);


	let a = [1, 2, 3, 4, 5];
//           0  1  2  3  4 
	let nice_slice = &a[1..4];
	/* This creates an array with 1-5 as variables
	// it's order is listed above
	// &a = we're borrowing the variable a
	//	* &i = borrowed immutable i
	//	* &mut i = borrowed mutable i
	//
	// &a[startVal..endVal+1] 
	// This means get vars from the start value to the
	// end value + 1 (our end value is 3, so put 4) */ 


	let cat = ("Furry McFurson", 3.5);
	let (name, age) = cat; // This is also kinda a crappy struct.
		// Just a Less concise way of doing this
		let name 	= cat.1; 
		let age 	= cat.2;
		println!("{} is {} years old.", name, age);
	/* Apparently you don't need to assign a type (eg str, i32) 
	// when you do this??? I would think that you do but.. guess not
	*/

	struct Dog {
		name: String, 
		age: i32,
		breed: String,
	}

	let puppy = dog {"Frankie".to_string(), 2, "foxhound".to_string()};
	//Still understanding the diff between String and &str


/* OWNERSHIP BASICS */
// Basically showing how memory ownership works;

let name = "joe".to_string(); 	// Creation of Name with data "joe"
let a = name;					// var a now owns data "joe" and name is empty
// let b = name;				// This will not work because name is empty 
let b = a.clone();				// Duplicates a's data as it's own. 
								// Now if a's value changes b doesn't care

/*           [–– name ––] [––– a –––][–––– b ––––]
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+
stack frame │   │   │   │ • │ 8 │ 6 │ • │ 8 │ 6 │
            +–––+–––+–––+–│–+–––+–––+–│–+–––+–––+
                          │           │
              +–––––––––––+           +–––––––+
              │                               │
            +–V–+–––+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
// Courtesy of thoughtram.io/ownership-in-rust */ 

fn greet_broken(name: String) { // This steals the variable, making it a single use fn }
   greet_broken(name);
// greet_broken(name); // This wont compile because name was used once already

fn greet(name: &String) {		// This borrows the variable so greet can be reused infinitely }
   greet(&name);
   greet(&name); // completely valid because name is borrowed



}






/*
Points of confusion:
    move_semantics2.rs 	-> I might need someone to explain that a little better to me 
                       	-> while I got the right answer I don't entirely understand what I did

	structs1.rs			-> confused with the diff between "String" and "&Str"
						-> temp solution is just use "foobar".to_string();
						Vec<Conrad, [Ludgate<u8>]>: 
							str is the same as [u8] (that is, an undefined length of contiguous bytes stored somewhere in memory) but with some extra utf8 validation.
							When you have a &str, now the size is known since its stored in the 'fat pointer'
							String is an owned string that works more like a Vec<u8> rather than a slice, which means it can reallocate and be mutated a lot more
						https://stackoverflow.com/questions/47640550/what-is-a-in-rust-language


    &str which is a 'string reference'. It's non-resizable and it's mutability is limited.
    String which is an 'owned string'. It's resizable and can be mutated simply.

*/