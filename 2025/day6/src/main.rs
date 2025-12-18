use std::io::{self, BufRead, Cursor};

pub(crate) fn math<F>(reader: io::BufReader<Cursor<&str>>, part: F) -> io::Result<u64>
where
    F: Fn() -> u64,
{
    for line_result in reader.lines() {
        let line_untrimmed = line_result?;
        let line = line_untrimmed.trim();
    }
    Ok(part())
}

#[allow(dead_code)]
pub(crate) fn part_1() -> u64 {
    10
}

#[allow(dead_code)]
pub(crate) fn part_2() -> u64 {
    10
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
