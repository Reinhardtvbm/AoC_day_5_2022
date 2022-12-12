use std::{collections::VecDeque, fs::read_to_string, num};

fn main() {
    let file_contents = read_to_string("data.txt").unwrap();
    let lines = file_contents.lines();

    let mut number_of_bars = 0;
    let mut stack_size = 0;

    for line in lines.clone() {
        let line_elements = line.trim().split(' ').collect::<Vec<&str>>();

        match line_elements[0].parse::<i32>() {
            Ok(_) => {
                number_of_bars = (line_elements.len() + 2) / 3;

                break;
            }
            Err(_) => stack_size += 1,
        }
    }

    println!("{:#?}", number_of_bars);

    let mut stacks: Vec<VecDeque<char>> = vec![
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];
    let mut moves: Vec<(usize, usize, usize)> = Vec::new();

    for (line_no, line) in lines.enumerate() {
        if line_no < stack_size {
            // get item stacks:

            let mut n = 0;
            let mut index = 0;

            while index < line.len() {
                index = (4 * n) + 1;

                if let Some(char) = line.chars().nth(index) {
                    if char != ' ' {
                        stacks[n].push_front(char);
                    }
                }

                n += 1;
            }
        } else if line_no > stack_size + 1 {
            // get moves:

            let words = line.split(' ').collect::<Vec<&str>>();

            moves.push((
                words[1].parse::<usize>().unwrap(),
                words[3].parse::<usize>().unwrap(),
                words[5].parse::<usize>().unwrap(),
            ))
        }
    }

    for (number_items, from_stack, to_stack) in moves.iter() {
        let mut stack: Vec<char> = Vec::new();

        for _ in 0..*number_items {
            stack.push(stacks[*from_stack - 1].pop_back().unwrap());
        }

        for item in stack.into_iter().rev() {
            stacks[*to_stack - 1].push_back(item);
        }
    }

    let mut top_items = String::new();

    for stack in stacks {
        top_items.push(*stack.back().unwrap());
    }

    println!("{:#?}", top_items);
    println!("{:#?}", moves.len());
}
