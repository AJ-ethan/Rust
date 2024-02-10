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

// use std::io;

// fn main() {
//     println!("Please enter a greeting:");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Failed to read input");

//     // use of match expression to pattern match against variable "name"
//     match name.trim().to_lowercase().as_str() {
//         "good bye" => println!("Sorry to see you go."),
//         "hello" => println!("Hi, nice to meet you!"),
//         "good night" => println!("sojaa benstokes"),
//         _ => println!("I can't find a greeting, good bye."),
//     }
// }



fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// When you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
// fn borrow_str(s: &mut String) {
//     println!("innner {}",s);
//     s.push_str("nnnnnnnnnnn");
// }

// fn main() {
//     let mut my_vec = vec![1, 2, 3, 4, 5];
//     let my_int = 10;
//     let my_string = String::from("Hello, world!");
//     let mut m_string = String::from("djsfhsdjkf fjdasl!");
//     // this compiles no problem!
//     own_integer(my_int);
//     println!("{}", my_int);

//     own_string(my_string); // take ownership of my_string
//     // this is using my_string which has also moved and is invalid
//     //println!("{:?}", my_string); // this will not compile!

//     own_vec(my_vec);
//     // but this is using my_vec which was borrowed (moved) and yet is now invalid
//     //println!("{:?}", my_vec); // this will not compile!

//     println!("{}",m_string);
//     borrow_str(&mut m_string);
//     println!("{}",m_string);
// }

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.






#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    email:String,
    age: u8,
}

fn person(first_name:String,last_name:String)-> String{
    let email = "empty".to_string();
    let age = 0;
    let person_obj= Person {
        first_name,
        last_name,
        email,
        age,
    };
    let n = person_obj.first_name+&person_obj.last_name;
    n
}

fn main() {
    let person_obj= Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@doe.com".to_string(),
        age: 25,
    };
    println!("{:?}", person_obj);

    let person_result = person("Jhn".to_string(),"doe".to_string());
    println!("{:?} result",person_result);
}