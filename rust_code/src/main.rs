// importing necessary libraries
use std::collections::HashMap;
use std::env; // for handling command line arguments



fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();

    // define a default value for the fibonacci number
    let default_n = 10;

    // parse the command line argument if provided, otherwise use the default value
    let n: u32 = if args.len() > 1 {
        // try to convert the argument to an integer
        // if it fails, print an error message and use the default value
        // `{}` is a placeholder
        args[1].parse().unwrap_or_else(|_| {
            println!("invalid number provided. using default value: {}", default_n);
            default_n
        })
    } 
    else {
        // if no argument is provided, use the default value
        default_n
    };

    // create a new hashmap for memoization so it can scale for higher numbers
    let mut memo: HashMap<u32, u64> = HashMap::new();

    // call the fibonacci function and print the result
    println!("fibonacci({}) = {}", n, fibonacci(n, &mut memo));
}



// the function to compute the nth fibonacci number
// takes an integer and a mutable reference to a hashmap
fn fibonacci(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    // check if the result is already in the memoization hashmap
    // `if let` is  similar to checking `if key in dict` in python
    if let Some(&result) = memo.get(&n) {
        // return the result if found
        return result;
    }

    // base cases for fibonacci sequence
    // similar to the base cases in a recursive python function
    let result = if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        // recursive case: compute the result and store it in the hashmap
        fibonacci(n - 1, memo) + fibonacci(n - 2, memo)
    };

    // insert the computed result into the hashmap
    // equivalent to `dict[key] = value` in python
    memo.insert(n, result);

    // return the result
    result
}