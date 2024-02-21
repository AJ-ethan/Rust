// fn main() {
//     let maybe_number: Option<Option<()>> = None;
//     let maybe_number = Some(42);
//     if let Some(number) = maybe_number {
//         println!("The number is {:?}", number);
//     } else {
//         println!("There is no number");
//     }
// }


//STRING INTERNAL

// fn main() {
//     let sentence = "the quick brown fox jumps over the lazy dog".to_string();
//     // Use slicing to get the first three characters of the sentence
//     println!("{}", &sentence[0..4]);

//     // concatenate using format!
//     let description = format!("Title: Quick story\n{}", sentence);
//     println!("{}", description);

//     // iterate over the characters in the sentence
//     for c in sentence.chars() {
//         match c {
//             'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
//             _ => continue,
//         }
//     }
//     // Split and collect into a vector
//     //let words: Vec<&str> = sentence.split_whitespace().collect();
//     let words = sentence.split(' ').collect::<Vec<&str>>();
//     println!("{:?}", words);


//     let reversed = sentence.chars().rev().collect::<String>();
//     println!("{}", reversed);
// }


fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    let third_value = vec[2];
    //println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    //println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    // match vec.first() {
    //     Some(first_value) => println!("The first value in the vector is: {}", first_value),
    //     None => println!("The vector is empty!"),
    // }
}
