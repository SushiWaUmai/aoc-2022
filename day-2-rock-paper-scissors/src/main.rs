use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .fold(0, |mut result, x| {
                let data: Vec<&str> = x.split(" ").collect();
                let first = data[0];
                let second = data[1];

                match second {
                    "X" => {
                        result += 1;
                        match first {
                            "A" => result += 3,
                            "B" => {}
                            "C" => result += 6,
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    "Y" => {
                        result += 2;
                        match first {
                            "A" => result += 6,
                            "B" => result += 3,
                            "C" => {}
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    "Z" => {
                        result += 3;
                        match first {
                            "A" => {}
                            "B" => result += 6,
                            "C" => result += 3,
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    _ => {
                        unreachable!();
                    }
                }

                result
            })
    );
}
