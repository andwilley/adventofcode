use std::io::{self, BufRead, Cursor};

#[derive(Debug)]
struct Equation {
    operator: Op,
    operands: Vec<Vec<char>>,
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

const BASE: u32 = 10;

pub(crate) fn math<F>(reader: io::BufReader<Cursor<&str>>, part: F) -> io::Result<u64>
where
    F: Fn(&[Equation]) -> u64,
{
    let mut operands: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<Op> = Vec::new();
    let mut lines = reader.lines().peekable();
    let mut equations: Vec<Equation> = Vec::new();
    while let Some(line_result) = lines.next() {
        let line = line_result?;
        let vals: Vec<char> = line.chars().collect();
        if lines.peek().is_some() {
            operands.push(vals);
        } else {
            operators = line
                .split_whitespace()
                .map(|n| match n.trim() {
                    "+" => Op::Add,
                    "*" => Op::Multiply,
                    unknown => panic!("unsupported operator: {unknown}"),
                })
                .collect();
        }
    }
    let rows = operands.len();
    let mut index = 0;
    for o in operators {
        let mut acc = vec![Vec::new(); rows];
        while let Some(slice) = next_slice(&operands, index) {
            for j in 0..rows {
                acc[j].push(slice[j]);
            }
            index += 1;
        }
        index += 1;
        equations.push(Equation {
            operator: o,
            operands: acc,
        });
    }
    Ok(part(&equations))
}

fn next_slice(input: &[Vec<char>], index: usize) -> Option<Vec<char>> {
    if index >= input[0].len() {
        return None;
    }
    let rows = input.len();
    let mut all_spaces = true;
    let mut tmp = Vec::with_capacity(rows);
    for level in 0..rows {
        let x = &input[level][index];
        tmp.push(*x);
        if *x != ' ' {
            all_spaces = false;
        }
    }
    if all_spaces { None } else { Some(tmp) }
}

fn to_int(v: &[i64]) -> i64 {
    let mut res: i64 = 0;
    for (i, n) in v.iter().rev().enumerate() {
        res += (BASE.pow(i as u32) * *n as u32) as i64;
    }
    res
}

#[allow(dead_code)]
pub(crate) fn part_1(eqs: &[Equation]) -> u64 {
    let mut ans = 0;
    for eq in eqs {
        let nums = eq.operands.iter().map(|l| {
            l.iter()
                .filter_map(|&n| n.to_digit(BASE).map(|d| d as i64))
                .collect::<Vec<i64>>()
        });
        match eq.operator {
            Op::Multiply => ans += nums.fold(1, |acc, x| acc * to_int(&x)),
            Op::Add => ans += nums.fold(0, |acc, x| acc + to_int(&x)),
        }
    }
    ans as u64
}

#[allow(dead_code)]
pub(crate) fn part_2(eqs: &[Equation]) -> u64 {
    let mut ans = 0;
    for eq in eqs {
        let mut nums: Vec<Vec<i64>> = Vec::new();
        for i in 0..eq.operands[0].len() {
            let mut n: Vec<i64> = Vec::new();
            for j in 0..eq.operands.len() {
                if let Some(v) = eq.operands[j][i].to_digit(BASE) {
                    n.push(v as i64)
                }
            }
            nums.push(n);
        }
        match eq.operator {
            Op::Multiply => ans += nums.iter().fold(1, |acc, x| acc * to_int(x)),
            Op::Add => ans += nums.iter().fold(0, |acc, x| acc + to_int(x)),
        }
    }
    ans as u64
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = math(reader, part_2)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{math, part_1, part_2};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "123 328  51 64 
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
        assert_eq!(ans, 3263827);
    }
}
