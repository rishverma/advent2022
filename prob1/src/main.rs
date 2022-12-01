use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("calories.txt") {

        let mut total_calories = 0;
        let mut calorie_list = Vec::new();
        for line in lines {
            
            if let Ok(calories) = line {
                let calorie: u32 = match calories.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        calorie_list.push(total_calories);
                        total_calories = 0;
                        continue
                        },
                };
                total_calories += calorie;                
            }
        }
        calorie_list.sort();
        let sum_calorie: u32 = calorie_list.iter().rev().take(3).sum();
        println!("part 1: {:?}", calorie_list.last().unwrap());
        println!("part 2: {:?}",sum_calorie);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}