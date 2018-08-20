use std::collections::HashMap;
use std::io;

pub fn get_user_input() -> String {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line.");
    return user_input.trim().to_string();
}

fn main() {
    let dictionary: HashMap<&str, &str> = [
        ("akin", "συγγενής"),
        ("aspect", "πτυχή"),
        ("decree", "διάταγμα"),
        ("inexorable", "αδυσώπητος"),
        ("masterly", "απαράμιλλος"),
        ("alluring", "δελεαστικός, θελκτικός"),
    ].iter().cloned().collect();


    loop {
        println!("Type term to search:");
        let term = get_user_input();
        match dictionary.get(&term.as_str()) {
            Some(value) => println!("{}\n", value),
            None => println!("Not in the dictionary.\n"),
        }
    }
}
