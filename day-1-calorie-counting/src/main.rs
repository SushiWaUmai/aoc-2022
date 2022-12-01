use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|x| x.to_string().parse::<i32>().ok())
            .fold(Vec::new() as Vec<Vec<i32>>, |mut result, x| {
                if result.is_empty() {
                    result.push(Vec::new())
                }

                match x {
                    Some(val) => {
                        result.last_mut().unwrap().push(val);
                    }
                    None => {
                        result.push(Vec::new());
                    }
                }

                result
            })
            .into_iter()
            .map(|x| x.iter().sum::<i32>())
            .max()
            .unwrap()
    );
}
