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
    let (name, age) = cat;
		// Just a Less concise way of doing this
		let name 	= cat.1; 
		let age 	= cat.2;
    	println!("{} is {} years old.", name, age);
	/* Apparently you don't need to assign a type (eg str, i32) 
	// when you do this??? I would think that you do but.. guess not
	*/
}