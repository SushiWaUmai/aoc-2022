use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|x| x.split(","))
            .map(|x| x.map(|y| y.split("-")))
            .map(|x| x.map(|y| y.map(|z| z.parse::<i32>().unwrap())))
            .map(|x| x.map(|y| y.collect::<Vec<i32>>()))
            .map(|x| x.map(|y| (y[0], y[1])).collect::<Vec<(i32, i32)>>())
            .map(|x| (x[0], x[1]))
            .fold(0, |mut result, x| {
                let ((x1, x2), (y1, y2)) = x;
                if (x1 <= y1 && y2 <= x2) || (y1 <= x1 && x2 <= y2) {
                    result += 1;
                }
                result
            })
    );
}
