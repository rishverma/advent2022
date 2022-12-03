use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let input: Vec<_> = binding
        .split("\n")
        .map(|a|
            {let items = a.len();
             let (compartment_1, compartment_2) = a.split_at(items/2);   
             let mut common_index = 0;
             for c in compartment_1.chars() {
                match compartment_2.find(c) {
                    Some(num) => common_index = num,
                    None => continue,
                };
             }
             alphabet.find(compartment_2.get(common_index..common_index+1).unwrap()).unwrap() + 1
            }   
            )
        .collect();
    let answer: usize = input.iter().sum();
    println!("{:?}", answer);
}

fn part_2() {
    let binding = fs::read_to_string("input.txt")
        .unwrap();
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let input: Vec<_> = binding
        .split("\n")
        .collect();
    let chunky_input: Vec<_> = input
        .chunks(3)
        .map(|a| {
            let mut badge = "a";
            for c in a[0].chars(){
                match a[1].find(c) {
                    Some(num) => {
                        match a[2].find(a[1].get(num..num+1).unwrap()) {
                            Some(num) => badge = a[2].get(num..num+1).unwrap(),
                            None => continue,
                        }
                    },
                    None => continue,
                }
            };
            alphabet.find(badge).unwrap() + 1
        })
        .collect();
    let answer: usize = chunky_input.iter().sum();
    println!("{:?}", answer);
}

