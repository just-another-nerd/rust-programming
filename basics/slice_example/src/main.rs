
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b' ' { return &s[0..i]; }

    }

    &s[..]
}

fn main() {

    let s = String::from("You cannot have PTSD if you are the traumatic event :)");

    let word = first_word(&s);

    println!("The first word is: {}", word);
}
