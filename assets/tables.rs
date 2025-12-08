// Data “tables” as functions and constants in Rust

pub const POOL: &str = "!@#$%^&*()[]{}";

pub fn attempts_str(n: u8) -> &'static str {
    match n {
        5 => "█ █ █ █ █",
        4 => "█ █ █ █",
        3 => "█ █ █",
        2 => "█ █",
        1 => "█",
        0 => "",
        _ => "", // or panic! if you want to treat others as invalid
    }
}

pub fn likeness_str(n: u8) -> &'static str {
    match n {
        5 => "█ █ █ █ █",
        4 => "█ █ █ █",
        3 => "█ █ █",
        2 => "█ █",
        1 => "█",
        0 => "",
        _ => "",
    }
}

pub fn modifier_str(n: u8) -> &'static str {
    match n {
        3 => "This shouldn't happen",
        2 => "Dud Removed",
        1 => "Another Chance!",
        0 => "MUTATE",
        _ => "This shouldn't happen", // or "" / panic!
    }
}

pub fn acceptable_combination(open: char) -> Option<char> {
    match open {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

