use std::fs::{self};

fn calculate_elf_calories(content: String) -> Vec<i32> {
    let lines = content.lines();
    let mut elf_calories: Vec<i32> = [].to_vec();
    let mut calorie_count: i32 = 0;
    for line in lines {
        if line == "" {
            elf_calories.push(calorie_count);
            calorie_count = 0;
        }
        else {
            let calories: i32 = line.trim().parse().unwrap();
            calorie_count = calorie_count + calories;
        }
    }
    elf_calories.push(calorie_count);
    elf_calories.sort();
    elf_calories
}

fn find_max_calories(content: String) -> i32 {
    let elf_calories: Vec<i32> = calculate_elf_calories(content);
    0 + elf_calories.last().unwrap()
}

fn find_top_three_calories(content: String) -> i32 {
    let mut elf_calories: Vec<i32> = calculate_elf_calories(content);
    elf_calories.pop().unwrap() + elf_calories.pop().unwrap() + elf_calories.pop().unwrap()
}

fn read_lines(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().to_string()
}

pub fn main() {
    println!("elf with most: {}", find_max_calories(read_lines("./input.txt")));
    println!("top three elves with most: {}", find_top_three_calories(read_lines("./input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_calories() {
        let max_calories = find_max_calories(
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string());
        assert_eq!(max_calories, 24000);
    }

    #[test]
    fn test_find_top_three_max_calories() {
        let max_calories = find_top_three_calories(
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string());
        assert_eq!(max_calories, 45000);
    }
}