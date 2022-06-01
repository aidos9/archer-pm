use std::io::{stdin, stdout, Write};
use std::process::exit;

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

pub fn package_path(name: Option<String>, path: Option<String>) -> String {
    if name.is_some() || path.is_none() {
        eprintln!("Error: Manager is not enabled.");
        exit(1);
    }

    return path.unwrap();
}
