use std::collections::HashSet;
use std::fs::{self};

fn find_marker_pos(source: &str) -> usize {
    let length = source.len();
    let mut position: usize = 0;

    for i in 3..length {
        // look at i-3..i to see if all the characters are different
        let mut seen_chars: HashSet<char> = HashSet::new();
        let mut good = true;
        for chr in source.get(i-3..i+1).unwrap().chars() {
            if seen_chars.contains(&chr) {
                good = false;
            }
            seen_chars.insert(chr);
        }
        if good && position == 0 {
            position = i;
        }
    }
    position + 1
}

fn find_message_pos(source: &str) -> usize {
    let length = source.len();
    let mut position: usize = 0;

    for i in 13..length {
        // look at i-3..i to see if all the characters are different
        let mut seen_chars: HashSet<char> = HashSet::new();
        let mut good = true;
        for chr in source.get(i-13..i+1).unwrap().chars() {
            if seen_chars.contains(&chr) {
                good = false;
            }
            seen_chars.insert(chr);
        }
        if good && position == 0 {
            position = i;
        }
    }
    position + 1
}


fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().to_string()
}

pub fn main() {
    let source = read_file("./input.txt");
    println!("Marker Position: {}", find_marker_pos(&source));
    println!("Message Position: {}", find_message_pos(&source));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker_pos() {
        assert_eq!(find_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);

    }
}