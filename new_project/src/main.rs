// fn main() {
//     println!("Hello, world!");
//     let i = 12;
//     if i<10{
//         println!("less than 10");
//     }
//     else if i>10
//        {println!("less than 10");}

//     let proceed = false;
//     if proceed {
//         println!("Proceeding");
//     } else {
//         println!("Not proceeding");
//     }

//     let height = 190;
//     if height < 180 {
//         println!("Tall");
//     } else if height > 170 {
//         println!("Average");
//     } else {
//         println!("Short");
//        }
// }



// fn sum(numbers: &[i32]) -> i32 {
//     let mut result = 0;
//     for number in numbers {
//         result += number;
//     }
//     result
// }

// fn main() {
//     // There are no variadic arguments in Rust
//     let numbers = [1, 2, 3, 4, 5];
//     let result = sum(&numbers);
//     println!("The sum is {}",  result);
// }

use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // use of match expression to pattern match against variable "name"
    match name.trim() {
        "Good Morning" => println!("Good Morning"),
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
        
    }
}

