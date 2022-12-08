use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|x| String::from(x))
            .map(|mut x| {
                let length = x.len();
                let y = String::split_off(&mut x, length / 2);
                (x, y)
            })
            .map(|(x, y)| {
                let mut found: Option<u8> = None;
                for cx in x.as_bytes() {
                    for cy in y.as_bytes() {
                        if cx == cy {
                            found = Some(*cy);
                            break;
                        }
                    }
                    if let Some(_) = found {
                        break;
                    }
                }

                found.unwrap()
            })
            .map(|x| {
                if x > 96 {
                    return x - 96;
                }

                return x - 64 + 26;
            })
            .fold(0, |x, y| x as i32 + y as i32)
    );
}
