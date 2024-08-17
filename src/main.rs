use std::env;
use std::io;
use std::process;

fn check_alphanumeric(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' => true,
        '_' => true,
        _ => check_digit(c),
    }
}

fn check_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn check_in_set(c: char, set: &str) -> bool {
    set.contains(c)
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        r"\d" => return input_line.chars().any(check_digit),
        r"\w" => return input_line.chars().any(check_alphanumeric),
        _ => if pattern.len() == 2 && pattern.starts_with('[') && pattern.ends_with(']') {
           return false
        } else if pattern.len() > 2 && pattern.starts_with('[') && pattern.ends_with(']') {
            if pattern.chars().nth(1) == Some('^') {
                let set = &pattern[2..pattern.len() - 1];
                return !input_line.chars().any(|c| check_in_set(c, set))
            } else {
                let set = &pattern[1..pattern.len() - 1];
                return input_line.chars().any(|c| check_in_set(c, set))
            }
        } else {
            match pattern.len() {
                1 => return input_line.contains(pattern),
                _ => panic!("Unhandled pattern: {}", pattern)
            }
        },
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
