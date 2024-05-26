use std::io;

fn main() {
    loop {
        let mut word = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        let word = String::from(word.trim());

        println!("{}", translate_to_pig_latin(&word));
    }
}

fn translate_to_pig_latin(word: &String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u']; // change to Set

    if word.len() < 1 {
        return String::new();
    }

    let mut chars_iter = word.chars();
    let first_char = chars_iter.next().unwrap();

    if vowels.contains(&first_char) {
        let mut s = word.clone();
        s.push_str("-hay");
        return s;
    } else {
        let mut s = String::from(chars_iter.as_str());
        if chars_iter.count() > 0 {
            s.push('-');
        }
        s.push(first_char);
        s.push_str("ay");

        return s;
    }
}
