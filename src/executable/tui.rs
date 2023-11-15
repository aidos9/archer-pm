#[cfg(feature = "with-tui")]
use crossterm::terminal;

const DEFAULT_LINE_WIDTH: usize = 64;
const SEPARATOR_LINE_STR: &'static str = "~";

#[cfg(not(feature = "with-tui"))]
pub fn print_separator_line() {
    default_print_separator_line();
}

#[cfg(feature = "with-tui")]
pub fn print_separator_line() {
    if let Ok((w, _)) = terminal::size() {
        for _ in 0..w {
            print!("{}", SEPARATOR_LINE_STR);
        }

        println!();
    } else {
        default_print_separator_line();
    }
}

fn default_print_separator_line() {
    for _ in 0..DEFAULT_LINE_WIDTH {
        print!("{}", SEPARATOR_LINE_STR);
    }

    println!();
}

#[cfg(feature = "with-tui")]
pub fn print_section_title(t: &str) {
    if let Ok((w, _)) = terminal::size() {
        print_centred_text(t, w as usize);
    } else {
        default_print_section_title(t);
    }
}

#[cfg(not(feature = "with-tui"))]
pub fn print_section_title(t: &str) {
    default_print_section_title(t);
}

fn default_print_section_title(t: &str) {
    if t.len() > DEFAULT_LINE_WIDTH {
        println!("{}", t);
    } else {
        print_centred_text(t, DEFAULT_LINE_WIDTH);
    }
}

fn print_centred_text(t: &str, available_width: usize) {
    let left_spacing = (available_width - t.len()) / 2;

    for _ in 0..left_spacing {
        print!("{}", SEPARATOR_LINE_STR);
    }

    print!("{}", t);

    for _ in 0..available_width - left_spacing - t.len() {
        print!("{}", SEPARATOR_LINE_STR);
    }

    println!();
}
