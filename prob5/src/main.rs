use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();
    
    let start_pos = binding.split("\n\n").collect::<Vec<_>>()[0];
    let moves = binding.split("\n\n").collect::<Vec<_>>()[1];

    let mut stack = parse_start_pos(start_pos);
    for move_line in moves.split("\n") {
        stack = parse_move_line(move_line,stack);
    }
    let answer = top_of_stack(stack);
    println!("{:?}", answer);
}

fn part_2() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();
    
    let start_pos = binding.split("\n\n").collect::<Vec<_>>()[0];
    let moves = binding.split("\n\n").collect::<Vec<_>>()[1];

    let mut stack = parse_start_pos(start_pos);
    for move_line in moves.split("\n") {
        stack = parse_move_line_9001(move_line,stack);
    }
    let answer = top_of_stack(stack);
    println!("{:?}", answer);
}

fn parse_start_pos (start: &str) -> Vec<Vec<char>> {
    let mut r = vec![Vec::new();9];
    let positions = vec![1,5,9,13,17,21,25,29,33];
    for stack in start.split("\n") {
        for (pos, e)  in positions.iter().enumerate() {
            match stack.chars().nth(*e) {
                Some(' ') => continue,
                Some(str) => r[pos].push(str),
                None => continue,
            }
        }
    }
    r
}

fn top_of_stack (stack: Vec<Vec<char>>) -> String {
    let mut top = Vec::new();
    for col in stack {
        top.push(col[0]);
    }
    top.iter().collect()
}



fn parse_move_line (move_line: &str, mut stack: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let line: Vec<_> = move_line.split(" ").collect();
    let num_containers: usize = line[1].parse().unwrap();
    let from_col: usize = line[3].parse().unwrap();
    let to_col: usize = line[5].parse().unwrap();
    for _n in 1..=num_containers {
        let container = stack[from_col-1].remove(0);
        stack[to_col-1].insert(0,container)
    }
    stack
}

fn parse_move_line_9001 (move_line: &str, mut stack: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let line: Vec<_> = move_line.split(" ").collect();
    let num_containers: usize = line[1].parse().unwrap();
    let from_col: usize = line[3].parse().unwrap();
    let to_col: usize = line[5].parse().unwrap();
    let mut temp_col = Vec::new();
    for _n in 1..=num_containers {
        let container = stack[from_col-1].remove(0);
        temp_col.insert(0,container)
    }
    for _n in 1..=num_containers {
        let container = temp_col.remove(0);
        stack[to_col-1].insert(0,container)
    }
    stack
}