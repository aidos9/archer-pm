use std::io::{stdin, stdout, Write};

pub fn y_n_question(question: &str) -> bool {
    let mut s = String::new();

    while s != "y" && s != "n" {
        print!("{} (y/n) ", question);
        stdout().flush().unwrap();

        stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
    }

    return s == "y";
}
