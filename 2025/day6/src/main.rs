use std::io::{self, BufRead, Cursor};

pub(crate) fn math<F>(reader: io::BufReader<Cursor<&str>>, part: F) -> io::Result<u64>
where
    F: Fn(&Vec<Vec<u64>>, &Vec<String>) -> u64,
{
    let mut operands: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    let mut lines = reader.lines().peekable();
    while let Some(line_result) = lines.next() {
        let line_untrimmed = line_result?;
        let line = line_untrimmed.trim();
        let vals = line.split_whitespace().map(|n| n.trim());
        if lines.peek().is_some() {
            operands.push(vals.map(|n| n.parse().unwrap()).collect());
        } else {
            operators = vals.map(|s| s.to_string()).collect();
        }
    }
    Ok(part(&operands, &operators))
}

#[allow(dead_code)]
pub(crate) fn part_1(operands: &Vec<Vec<u64>>, operators: &Vec<String>) -> u64 {
    let mut ans = 0;
    for i in 0..operators.len() {
        match operators[i].as_str() {
            "*" => ans += operands.iter().fold(1, |acc, x| acc * x[i]),
            "+" => ans += operands.iter().fold(0, |acc, x| acc + x[i]),
            unknown_op => panic!("unsupported operator {unknown_op}"),
        }
    }
    ans
}

#[allow(dead_code)]
pub(crate) fn part_2(_: &Vec<Vec<u64>>, _: &Vec<String>) -> u64 {
    10
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = math(reader, part_1)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{math, part_1, part_2};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "\
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   + ";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = math(reader, part_1).expect("");
        assert_eq!(ans, 4277556);
    }

    #[test]
    fn part2() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = math(reader, part_2).expect("");
        assert_eq!(ans, 10);
    }
}
