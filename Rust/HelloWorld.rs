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
        // f32 | f64 || Floating Point Nums 

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



}