use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::hash::Hash;

// Thank you stack overflow:
// https://stackoverflow.com/questions/46766560/how-to-check-if-there-are-duplicates-in-a-slice
fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn main() {
    let seq = fs::read_to_string("./input.txt").unwrap();
    let mut chars: VecDeque<char> = VecDeque::new();
    let mut result = 0;

    let distinct = 4; // use 14 in part 2

    for (i, s) in seq.chars().enumerate() {
        chars.push_back(s);

        println!("{:?}", chars);
        if chars.len() == distinct && has_unique_elements(chars.iter()) {
            result = i + 1;
            break;
        }

        if chars.len() >= distinct {
            chars.pop_front();
        }
    }

    println!("{}", result);
}
