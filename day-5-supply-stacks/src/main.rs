use std::fs;

fn main() {
    let str_data = fs::read_to_string("./input.txt").unwrap();

    let data_vec = str_data.split("\n\n").collect::<Vec<&str>>();
    let (stack_str_data, instruction_str_data) = (data_vec[0], data_vec[1]);

    let stack_len = (String::from(stack_str_data.lines().collect::<Vec<&str>>()[0]).len() + 1) / 4;
    let commands: Vec<(i32, usize, usize)> = instruction_str_data
        .lines()
        .map(|x| x.split(" ").map(|y| String::from(y)).into_iter())
        .into_iter()
        .map(|x| x.map(|y| y.parse::<i32>().ok()))
        .map(|x| x.filter(|y| y.is_some()))
        .map(|x| x.map(|y| y.unwrap()).collect::<Vec<i32>>())
        .map(|x| (x[0], x[1] as usize, x[2] as usize))
        .collect::<Vec<(i32, usize, usize)>>();

    let mut stack_values: Vec<String> = stack_str_data.lines().map(|x| String::from(x)).collect();
    stack_values.reverse();
    let stack_values: Vec<Vec<char>> = stack_values[1..]
        .to_vec()
        .iter()
        .map(|x| {
            x.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .map(|c| c.chars().nth(1).unwrap())
                .collect::<Vec<char>>()
        })
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..stack_len {
        stacks.push(vec![]);
    }

    for layer in stack_values.iter() {
        for (i, v) in layer.iter().enumerate() {
            if *v != ' ' {
                stacks[i].push(*v);
            }
        }
    }

    for (number, from, to) in commands {
        for _ in 0..number {
            let tmp = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(tmp);
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let result = stacks.iter().map(|x| x[0]).collect::<String>();


    println!("{:?}", result)
}
