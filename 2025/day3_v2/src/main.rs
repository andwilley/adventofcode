use std::{
    io::{self, BufRead, Cursor},
    mem::swap,
};

// The insight is to go backwards.
pub(crate) fn joltage<F>(reader: io::BufReader<Cursor<&str>>, get_joltage: F) -> io::Result<u64>
where
    F: Fn(Vec<u32>) -> u64,
{
    let mut ans = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        ans += get_joltage(trimmed.chars().map(|n| n.to_digit(10).unwrap()).collect());
    }
    Ok(ans)
}

pub(crate) fn sift<const N: usize>(nums: Vec<u32>) -> u64 {
    let back = &nums[nums.len() - N..];
    let front = &nums[..nums.len() - N];
    let mut ans = [0; N];
    ans.copy_from_slice(back);
    for n in front.iter().rev() {
        let mut cur = *n;
        for i in 0..N {
            if cur < ans[i] {
                break;
            };
            swap(&mut cur, &mut ans[i]);
        }
    }
    let base: u64 = 10;
    let mut res: u64 = 0;
    for (i, n) in ans.iter().rev().enumerate() {
        res += base.pow(i as u32) * (*n as u64);
    }
    res
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = joltage(reader, sift::<12>)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{joltage, sift};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "\
        987654321111111
        811111111111119
        234234234234278
        818181911112111";

    const ONE: &str = "987654321111111";
    const TWO: &str = "811111111111119";
    const THREE: &str = "234234234234278";
    const FOUR: &str = "818181911112111";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = joltage(reader, sift::<2>).expect("");
        assert_eq!(ans, 357);
    }

    #[test]
    fn part2_1() {
        let reader = io::BufReader::new(Cursor::new(ONE));
        let ans = joltage(reader, sift::<12>).expect("");
        assert_eq!(ans, 987654321111);
    }

    #[test]
    fn part2_2() {
        let reader = io::BufReader::new(Cursor::new(TWO));
        let ans = joltage(reader, sift::<12>).expect("");
        assert_eq!(ans, 811111111119);
    }

    #[test]
    fn part2_3() {
        let reader = io::BufReader::new(Cursor::new(THREE));
        let ans = joltage(reader, sift::<12>).expect("");
        assert_eq!(ans, 434234234278);
    }

    #[test]
    fn part2_4() {
        let reader = io::BufReader::new(Cursor::new(FOUR));
        let ans = joltage(reader, sift::<12>).expect("");
        assert_eq!(ans, 888911112111);
    }
}
