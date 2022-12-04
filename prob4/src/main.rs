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
            {let secs: Vec<_> = a
                .split(",")
                .map(|b| 
                    {let c: Vec<_> = b.split("-").collect();
                    (c[0].parse::<usize>().unwrap()..c[1].parse::<usize>().unwrap()+1).collect::<Vec<_>>()
                }
                )
                .collect();
            match secs[0].iter().all(|item| secs[1].contains(item)) || secs[1].iter().all(|item| secs[0].contains(item))  {
                true => 1,
                false => 0,
            }
            }
        )
        .collect();
    let answer: usize = input.iter().sum();
    println!("{:?}", answer);
}

fn part_2() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();

    let input: Vec<_> = binding
        .split("\n")
        .map(|a|
            {let secs: Vec<_> = a
                .split(",")
                .map(|b| 
                    {let c: Vec<_> = b.split("-").collect();
                    (c[0].parse::<usize>().unwrap()..c[1].parse::<usize>().unwrap()+1).collect::<Vec<_>>()
                }
                )
                .collect();
            match secs[0].iter().any(|item| secs[1].contains(item)) || secs[1].iter().any(|item| secs[0].contains(item))  {
                true => 1,
                false => 0,
            }
            }
        )
        .collect();
    let answer: usize = input.iter().sum();
    println!("{:?}", answer);
}

