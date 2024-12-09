use std::fs;
use regex::Regex;

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Unable to compile regex");
    multiply(input, &re)
}

fn multiply(input: &str, re: &Regex) -> usize {
    let mut sum: usize = 0;
    for (_, [lhs, rhs]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += lhs.parse::<usize>().expect("Expected a usize") * rhs.parse::<usize>().expect("Expected a usize");
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut input = input.to_owned();
    let do_ = Regex::new(r"do\(\)").expect("Could not compile regex");
    let dont = Regex::new(r"don't\(\)").expect("Could not compile regex");
    let mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Unable to compile regex");
    let mut sum: usize = 0;
    let mut cursor: usize;
    let mut active: bool = true;
    loop {
        if active {
            let mat = dont.find(&input);
            match mat {
                Some(mat) => {
                    cursor = mat.range().end;
                    sum += multiply(&input[..cursor], &mul);
                    input= input[cursor..].to_owned();
                    active = false;
                }
                None => {
                    sum += multiply(&input, &mul);
                    return sum;
                }
            }
        } else {
            let mat = do_.find(&input);
            match mat {
                Some(mat) => {
                    cursor = mat.range().end;
                    input = input[cursor..].to_owned();
                    active = true;
                }
                None => return sum
            }
        }
    }
}

fn load(path: &str) -> String {
    fs::read_to_string(path).expect("File not found")
}

fn main() {
    let code = load("input.txt");
    println!("Part 1: {}", part1(&code));
    println!("Part 2: {}", part2(&code));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = load("test.txt");
        assert_eq!(part2(&data), 48);
    }
}
