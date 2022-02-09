fn main() {
    
        let mut line = String::new();
        println!("Enter your name :");
        std::io::stdin().read_line(&mut line).unwrap();
        println!("Hello , {}", line);
        
    

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