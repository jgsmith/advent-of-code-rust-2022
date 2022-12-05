use std::ops::Range;
use std::fs::{self};

fn is_contained(range1: &Range<u32>, range2: &Range<u32>) -> bool {
    range1.start >= range2.start && range1.end <= range2.end ||
    range2.start >= range1.start && range2.end <= range1.end
}

fn overlaps(range1: &Range<u32>, range2: &Range<u32>) -> bool {
    range1.start <= range2.end && range1.end >= range2.start ||
    range2.start <= range1.end && range2.end >= range1.start
}

fn count_superfluous_elves(list: &Vec<[Range<u32>;2]>) -> u32 {
    let mut count = 0;
    for group in list {
        if is_contained(&group[0], &group[1]) {
            count += 1;
        }
    }
    count
}

fn count_overlapping_elves(list: &Vec<[Range<u32>;2]>) -> u32 {
    let mut count = 0;
    for group in list {
        if overlaps(&group[0], &group[1]) {
            count += 1;
        }
    }
    count
}

fn get_ranges_from_lines(source: &str) -> Vec<[Range<u32>;2]> {
    let mut ranges : Vec<[Range<u32>;2]> = Vec::new();

    for line in source.lines() {
        let line = line.trim();
        let (first, second) = line.split_once(',').unwrap();
        let (first_start_s, first_end_s) = first.split_once('-').unwrap();
        let (second_start_s, second_end_s) = second.split_once('-').unwrap();
        let first_start: u32 = first_start_s.parse().unwrap();
        let first_end: u32 = first_end_s.parse().unwrap();
        let second_start: u32 = second_start_s.parse().unwrap();
        let second_end: u32 = second_end_s.parse().unwrap();

        ranges.push([
            Range { start: first_start, end: first_end },
            Range { start: second_start, end: second_end }
        ]);
    }
    ranges
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().to_string()
}

pub fn main() {
    let source = read_file("./input.txt");
    let ranges = get_ranges_from_lines(&source);
    let count = count_superfluous_elves(&ranges);
    println!("Superflous elves: {}", count);

    let overlap_count = count_overlapping_elves(&ranges);
    println!("Overlapping elves: {}", overlap_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_contained() {
        assert!(is_contained(&(6..6), &(4..6)));
        assert!(is_contained(&(2..8), &(3..7)));
        assert!(!is_contained(&(2..4), &(6..8)));
        assert!(!is_contained(&(2..6), &(4..8)));
    }

    #[test]
    fn test_overlaps() {
        assert!(overlaps(&(6..6), &(4..6)));
        assert!(overlaps(&(2..8), &(3..7)));
        assert!(!overlaps(&(2..4), &(6..8)));
        assert!(overlaps(&(2..6), &(4..8)));
    }
}