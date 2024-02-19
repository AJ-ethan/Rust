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

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => continue,
        }
    }
    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}

 