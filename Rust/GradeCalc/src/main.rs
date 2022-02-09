#![allow(non_snake_case)]
#![allow(unused_parens)]
//Deps
use std::io;
use std::str::FromStr;

fn main() {
	let mut examCount: i32;
	let mut assignCount: i32;
	let mut finalTaken: bool = false;

	//Intro
	println!("This program will calculate your grades!");

	// 							    EXAMS										//
	println!("How many exams have you taken?");
		let mut strEC = String::new();			// tmp string var
		io::stdin() 							// Input
			.read_line(&mut strEC)				// send input to strEC
			.expect("Please Enter a number");	// potential error
	examCount = strEC.trim().parse().unwrap();	// Sanitize back into examCount
	

	let mut examGrades = vec![0, examCount];
	if(examCount == 0){
		println!("Well then you have no test grades lol");

	} else if (examCount <= 1) {
		println!("Go ahead and enter your test grade: ");
		examGrades = read_values().expect("user didn't input list");
		// array input

	} else {
		println!("Go ahead and enter your {} test grades", examCount);
		examGrades = read_values().expect("user didn't input list");
		// array input
		
	}
	if (examCount <= 4) {finalTaken = true;}
	



		
	// 							    Assignments									//
	println!("How many Assignments have you taken?");
		let mut strAC = String::new();			// tmp string var
		io::stdin() 							// Input
			.read_line(&mut strAC)				// send input to strAC
			.expect("Please Enter a number");	// potential error
	assignCount = strAC.trim().parse().unwrap();	// Sanitize back into examCount
	

	let mut asgnGrades = vec![0, assignCount];
	if(assignCount == 0){
		println!("Well then you have no assignment grades lol");

	} else if (assignCount <= 1) {
		println!("Go ahead and enter your assignment grade: ");
		asgnGrades = read_values().expect("user didn't input list");
		// array input

	} else {
		println!("Go ahead and enter your {} assignment grades", assignCount);
		asgnGrades = read_values().expect("user didn't input list");
		// array input
		
	}
		// 							CALCULATION									//
	
		
}

fn read_values<T: FromStr>() -> Result<Vec<T>, T::Err> {
			//read generic type from string,
			// -> output result that is a generic vector/array
			// or a generic error T::Err
			let mut s = String::new();
			io::stdin()                           //input stuff
			.read_line(&mut s)                    //input changes var s
			.expect("could not read from stdin"); // possible error message
			
			s.trim()            // Trim leading/ending whitespace
			.split_whitespace() // split into separate vars @ whitespace
								// this converts S into an interator (sequence of vals)
			.map(|word| word.parse()) 
			//.parse converts string slice into desired type
			//.unwrap() is basically "error & quit"
				// unwrap isn't needed because we have T::Err

			.collect() // converts a list of values into a container type. 
					   // In this case a Vec
		}

