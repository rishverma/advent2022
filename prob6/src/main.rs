use std::fs;
use std::collections::HashSet;

fn main() {
    find_message(4);
    find_message(14);
}

fn find_message (distinct_length: usize) {
    let binding = fs::read_to_string("input.txt")
        .unwrap();
    
    let mut input: Vec<_> = binding.split("").collect();
    input.remove(0);
    input.pop();

    for (pos, _e) in input.iter().enumerate() {
        if pos > distinct_length-2 {
            let input_slice: HashSet<_> = input[pos-(distinct_length-1)..=pos]
                .iter()
                .cloned()
                .collect();
            if input_slice.len() == distinct_length {
                println!("{:?}, {:?}", input_slice,pos+1);
                break;
            }
            
        }

    }
}
