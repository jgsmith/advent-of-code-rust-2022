use std::collections::HashMap;
use std::fs::{self};

static ROCK: i32 = 0;     // A X
static PAPER: i32 = 1;    // B Y
static SCISSORS: i32 = 2; // C Z

// see if player2 beats player1
fn score_round(opponent: &str, player: &str, plays: &HashMap<&str, i32>) -> i32 {
    let mut score : i32 = plays[player] + 1;

    if plays[opponent] == plays[player] {
        score = score + 3; // draw
    }
    if plays[player] == (plays[opponent] + 1) % 3 {
        score = score + 6; // win
    }
    score
}

fn score_run(rounds: &Vec<[&str; 2]>, plays: &HashMap<&str, i32>) -> i32 {
    let mut score = 0;
    for x in rounds {
        score = score + score_round(x[0], x[1], plays);
    }
    score
}

fn choose_plays(rounds: &Vec<[&str; 2]>) -> Vec<[&'static str; 2]> {
    let mut new_run : Vec<[&str; 2]> = Vec::new();
    let best_play: HashMap<[&str; 2], &str> = HashMap::from([
        // X -> lose, Y -> draw, Z -> win
        (["A", "X"], "Z"), (["A", "Y"], "X"), (["A", "Z"], "Y"),
        (["B", "X"], "X"), (["B", "Y"], "Y"), (["B", "Z"], "Z"),
        (["C", "X"], "Y"), (["C", "Y"], "Z"), (["C", "Z"], "X"),
    ]);
    let first_play: HashMap<&str, &str> = HashMap::from([ ("A", "A"), ("B", "B"), ("C", "C") ]);

    for play in rounds {
        let best_play = best_play[play];
        let first_play = first_play[play[0]];
        new_run.push([&first_play, &best_play])
    }
    new_run
}

fn line_to_play(line: &str) -> [&str; 2] {
    let mut split_line = line.trim().split(' ');
    let first = split_line.next().unwrap();
    let second = split_line.next().unwrap();
    [first, second]
}

fn source_to_run(source: &str) -> Vec<[&str; 2]> {
    let mut run : Vec<[&str; 2]> = Vec::new();

    for line in source.lines() {
        let line = line.trim();
        if line != "" {
            run.push(line_to_play(line));
        }
    }

    run
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().to_string()
}

pub fn main() {
    let plays : HashMap<&str, i32> = HashMap::from([
        ("A", ROCK), ("B", PAPER), ("C", SCISSORS),
        ("X", ROCK), ("Y", PAPER), ("Z", SCISSORS)
    ]);
    let source = read_file("./input.txt");
    let runs = source_to_run(&source);
    println!("total score: {}", score_run(&runs, &plays));

    let second_runs = choose_plays(&runs);
    println!("total second score: {}", score_run(&second_runs, &plays));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_round() {
        let plays : HashMap<&str, i32> = HashMap::from([
            ("A", ROCK), ("B", PAPER), ("C", SCISSORS),
            ("X", ROCK), ("Y", PAPER), ("Z", SCISSORS)
        ]);

        assert_eq!(score_round("A", "Y", &plays), 8);
        assert_eq!(score_round("B", "X", &plays), 1);
        assert_eq!(score_round("C", "Z", &plays), 6);
    }

    #[test]
    fn test_score_run() {
        let plays : HashMap<&str, i32> = HashMap::from([
            ("A", ROCK), ("B", PAPER), ("C", SCISSORS),
            ("X", ROCK), ("Y", PAPER), ("Z", SCISSORS)
        ]);

        let rounds : Vec<[&str; 2]> = Vec::from([
            ["A", "Y"], ["B", "X"], ["C", "Z"]
        ]);

        assert_eq!(score_run(&rounds, &plays), 15);
    }

    #[test]
    fn test_line_to_play() {
       assert_eq!(line_to_play("A Y"), ["A", "Y"]);
    }

    #[test]
    fn test_source_to_run() {
        let rounds : Vec<[&str; 2]> = Vec::from([
            ["A", "Y"], ["B", "X"], ["C", "Z"]
        ]);
        assert_eq!(source_to_run("A Y\nB X\n\nC Z\n"), rounds);
    }
}