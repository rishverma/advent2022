use std::fs;
use std::collections::HashMap;

fn main() {
    part_1();
}

fn part_1() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();
    
    let input = binding.split("\n").collect::<Vec<_>>();
    let mut dir_size = HashMap::new();
    let mut dir_arch = Vec::new();
    let mut full_dir_arch = Vec::new();
    for line in input {
        let line_decomp: Vec<_> = line.split(" ").collect();
        match line_decomp[0] {
            "$" => {match line_decomp[1]{
                "cd" => {
                    if line_decomp[2] != ".." {
                        dir_arch.push(line_decomp[2]);
                        let full_dir = dir_arch.join("-");
                        full_dir_arch.push(full_dir.clone());
                        dir_size.insert(full_dir.clone(),0);
                        
                    } else {
                        dir_arch.pop();
                        full_dir_arch.pop();
                    }},
                "ls" => continue,
                _ => println!("something went wrong"),
            }},
            "dir" => continue,
            num => {
                for current_directory in &full_dir_arch {
                    dir_size.insert(current_directory.to_string(),num.parse::<u32>().unwrap()+dir_size[current_directory]);
                }
            },
        }
    }
    let mut part1_answer = 0;
    for (_key,val) in &dir_size {
        if val <= &100000 {
            part1_answer += val;
        }
    }
    println!("{:?}", part1_answer);
    
    let remove_space = &dir_size["/"]-40000000;

    let mut smallest_removable = 70000000;
    for (_key,val) in &dir_size {
        if val > &remove_space && val < &smallest_removable {
            smallest_removable = *val;
        }
    }
    println!("{:?}", smallest_removable );
}
