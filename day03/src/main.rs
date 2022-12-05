use std::collections::HashSet;
use std::fs::{self};

static LOWER_A: u16 = 97;
static UPPER_A: u16 = 65;

fn divide_rucksack(contents: &str) -> (&str, &str) {
    let length = contents.len();
    contents.split_at(length / 2)
}

fn rucksack_item_set(partition: &str) -> HashSet<char> {
    let mut set = HashSet::new();

    for ch in partition.chars() {
        set.insert(ch);
    }
    set
}

fn rucksack_sigil(partition1: HashSet<char>, partition2: HashSet<char>) -> u16 {
    let mut b = [0; 2];
    let mut intersection = partition1.intersection(&partition2);
    let common_char = intersection.next().unwrap();
    let result = common_char.encode_utf16(&mut b);
    result[0]
}

fn sigil_priority(sigil: u16) -> u16 {
    if sigil >= LOWER_A {
        sigil - LOWER_A + 1
    }
    else {
        sigil - UPPER_A + 27
    }
}

fn source_to_priorities(source: &str) -> Vec<u16> {
    let mut priorities : Vec<u16> = Vec::new();

    for line in source.lines() {
        let line = line.trim();
        let division = divide_rucksack(line);
        let set0 = rucksack_item_set(&division.0);
        let set1 = rucksack_item_set(&division.1);
        let sigil = rucksack_sigil(set0, set1);
        priorities.push(sigil_priority(sigil));
    }
    priorities
}

fn source_to_group_priorities(source: &str) -> Vec<u16> {
    let mut priorities : Vec<u16> = Vec::new();
    let mut lines = source.lines().peekable();
    let mut b = [0; 2];

    while lines.peek() != None {
        let sets = [rucksack_item_set(lines.next().unwrap()),
                    rucksack_item_set(lines.next().unwrap()),
                    rucksack_item_set(lines.next().unwrap())];
        let intersection = sets
            .iter()
            .skip(1)
            .fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });
        let common_char = intersection.iter().next().unwrap();
        let result = common_char.encode_utf16(&mut b);
        priorities.push(sigil_priority(result[0]));
    }
    priorities
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().to_string()
}

pub fn main() {
    let source = read_file("./input.txt");
    let priorities = source_to_priorities(&source);
    let priority_sum: u16 = priorities.iter().sum();
    println!("total priority: {}", priority_sum);

    let badge_priorities = source_to_group_priorities(&source);
    let badge_sum: u16 = badge_priorities.iter().sum();
    println!("total badges: {}", badge_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigil_priority() {
        assert_eq!(sigil_priority(112), 16); // p -> 16
        assert_eq!(sigil_priority(76), 38); // L -> 38
        assert_eq!(sigil_priority(80), 42); // P -> 42
        assert_eq!(sigil_priority(118), 22); // v -> 118
        assert_eq!(sigil_priority(116), 20); // t -> 20
        assert_eq!(sigil_priority(115), 19); // s -> 19
    }

    #[test]
    fn test_divide_rucksack() {
        assert_eq!(divide_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    }

    #[test]
    fn test_rucksack_sigil() {
        let division = divide_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp");
        let set0 = rucksack_item_set(&division.0);
        let set1 = rucksack_item_set(&division.1);
        let sigil = rucksack_sigil(set0, set1);
        assert_eq!(sigil, 112);
    }
}