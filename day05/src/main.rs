use std::fs::{self};

fn source_to_stacks(source: &str) -> Vec<Vec<char>> {
    let mut stacks : Vec<Vec<char>> = Vec::new();

    /*
    we can tell which stack something goes in by how far across it is
    in the input. We don't depend on the numbers. But the numbers let
    us know that we're done.
    so:
    [.] [.] [.] ...

    We can't do a simple split - we have to look at all of the positions.
    */
    for line in source.lines() {
        if line.find("[") != None {
            for (idx, chr) in line.char_indices() {
                if (idx + 3) % 4 == 0 && chr != ' ' {
                    let stack_idx = (idx - 1) / 4;
                    while stack_idx >= stacks.len() {
                        stacks.push(Vec::from([]));
                    }
                    stacks[stack_idx].insert(0, chr);
                }
            }
        }
    }
    stacks
}

fn source_to_moves(source: &str) -> Vec<[usize;3]> {
    let mut moves : Vec<[usize;3]> = Vec::new();

    for line in source.lines() {
        if line.find("move") != None {
            let words: Vec<&str> = line.splitn(6, ' ').collect();
            let count: usize = words[1].parse().unwrap();
            let from: usize = words[3].parse().unwrap();
            let to: usize = words[5].parse().unwrap();
            println!("{} : {} - {} - {}", line, count, from, to);
            moves.push([count, from - 1, to - 1]);
        }
    }
    moves
}

fn source_to_setup(source: &str) -> (Vec<Vec<char>>, Vec<[usize;3]>) {
    let (stack_setup, move_setup) = source.split_once("\n\n").unwrap();
    (source_to_stacks(stack_setup), source_to_moves(move_setup))
}

fn move_things(stacks: &mut Vec<Vec<char>>, moves: Vec<[usize;3]>) -> &Vec<Vec<char>> {
    for [count, from, to] in moves {
        for _ in 0..count {
            let popped = stacks[from].pop().unwrap();
            stacks[to].push(popped);
        }
    }
    stacks
}

fn move_things_keep_order(stacks: &mut Vec<Vec<char>>, moves: Vec<[usize;3]>) -> &Vec<Vec<char>> {
    for [count, from, to] in moves {
        let from_len = stacks[from].len();
        let moved_cargo = &stacks[from].split_off(from_len - count);

        stacks[to].extend_from_slice(moved_cargo);
    }
    stacks
}

fn top_cargo(stacks: &Vec<Vec<char>>) -> String {
    let mut tops : Vec<char> = Vec::new();

    for stack in stacks {
        tops.push(*stack.last().unwrap());
    }
    tops.iter().collect()
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().to_string()
}

pub fn main() {
    let source = read_file("./input.txt");
    let (mut stacks, moves) = source_to_setup(&source);
    // move_things(&mut stacks, moves); // Part 1
    move_things_keep_order(&mut stacks, moves); // Part 2
    println!("Final top cargo: {}", top_cargo(&stacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_to_stacks() {
        let source = "    [D]\n[N] [C]\n[Z] [M] [P]\n 1   2   3";
        assert_eq!(source_to_stacks(source), Vec::from([
            Vec::from(['Z', 'N']),
            Vec::from(['M', 'C', 'D']),
            Vec::from(['P'])
        ]));
    }

    #[test]
    fn test_source_to_moves() {
        let source = "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        assert_eq!(source_to_moves(source), Vec::from([
            [1, 1, 0],
            [3, 0, 2],
            [2, 1, 0],
            [1, 0, 1]
        ]));
    }

    #[test]
    fn test_top_cargo() {
        let source = "    [D]\n[N] [C]\n[Z] [M] [P]\n 1   2   3";
        let stacks = source_to_stacks(source);
        assert_eq!(top_cargo(&stacks), "NDP");
    }

    #[test]
    fn test_move_things() {
        let moves: Vec<[usize;3]> = Vec::from([
            [1, 1, 0],
            [3, 0, 2],
            [2, 1, 0],
            [1, 0, 1]
        ]);
        let mut stacks: Vec<Vec<char>> = Vec::from([
            Vec::from(['Z', 'N']),
            Vec::from(['M', 'C', 'D']),
            Vec::from(['P'])
        ]);

        move_things(&mut stacks, moves);
        assert_eq!(top_cargo(&stacks), "CMZ");
    }
}