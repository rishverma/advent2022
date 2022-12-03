use std::fs;

fn main() {
    part_1();
    part_2();

}

fn part_1() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();

    let input: Vec<_> = binding
        .split("\n")
        .map(|a|
            {let b = a.split(" ")
                .collect::<Vec<_>>();
             let score_played = match b[1] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
             };

            let score_result = rps_score_part1(b[0],b[1]);
            let score = score_result + score_played;
            score
            }   
            )
        .collect();

    let final_score: u32 = input.iter().sum();
    println!("{}", final_score);
}

fn part_2() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();

    let input: Vec<_> = binding
        .split("\n")
        .map(|a|
            {let b = a.split(" ")
                .collect::<Vec<_>>();
             let score_result = match b[1] {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
             };

            let score_played = rps_score_part2(b[0],b[1]);
            let score = score_result + score_played;
            score
            }   
            )
        .collect();

    let final_score: u32 = input.iter().sum();
    println!("{}", final_score);
}

fn rps_score_part1(a: &str, b: &str) -> u32 {
    match a {
        "A" => {if b == "X" {
            3
        } else if b == "Y" {
            6
        } else {
            0
        }
        }
        "B" => {if b == "Y" {
            3
        } else if b == "Z" {
            6
        } else {
            0
        }
        }
        "C" => {if b == "Z" {
            3
        } else if b == "X" {
            6
        } else {
            0
        }
        }
        _ => 10
    }
}

fn rps_score_part2(a: &str, b: &str) -> u32 {
    match a {
        "A" => {if b == "X" { //need a loss against rock
            3 //played scissors
        } else if b == "Y" { //need a draw
            1 //played rock
        } else {
            2 //played paper
        }
        }
        "B" => {if b == "X" { //need a loss against paper
            1 //played rock
        } else if b == "Y" { //need a draw
            2 //played paper
        } else {
            3 //played scissors
        }
        }
        "C" => {if b == "X" { //need a loss against scissors
            2 //played paper
        } else if b == "Y" {
            3 //played scissors
        } else {
            1 //played rock
        }
        }
        _ => 10
    }
}