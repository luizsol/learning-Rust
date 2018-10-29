fn main() {
    let s1 = String::from("Hello World!");
    let s2 = String::from("BingBoing");
    let s3 = String::from("Wooow! That's crazy!");

    let s1_new = replace_first_word(&s1, &String::from("Goodbye"));
    let s2_new = replace_first_word(&s2, &String::from("Boing Boing"));
    let s3_new = replace_first_word(&s3, &String::from("Noooooo!"));

    println!("Old: {}\nNew: {}", s1, s1_new);
    println!("Old: {}\nNew: {}", s2, s2_new);
    println!("Old: {}\nNew: {}", s3, s3_new);
}

fn replace_first_word(string: &String, new_word: &String) -> String {
    let mut result = new_word.clone();

    for (i, &c) in string.as_bytes().iter().enumerate() {
        if c == b' ' {
            result.push_str(&string[i..]);
            break;
        }
    }

    result
}
