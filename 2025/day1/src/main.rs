use std::io::{self, BufRead, Cursor};

fn count_zeros(reader: io::BufReader<Cursor<&str>>) -> io::Result<i64> {
    let mut cur = 50;
    let mut ans = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let mut chars = line.chars();
        let dir = chars.nth(0).expect("Should exist");
        let mvmt: i64 = chars.collect::<String>().parse().unwrap();
        let mv = if dir == 'L' { -1 * mvmt } else { mvmt };
        let mut zero_passes = 0;
        cur = {
            let raw = cur + mv;
            if raw <= 0 {
                zero_passes = (if cur == 0 { 0 } else { 1 }) + (raw.abs() / 100);
            } else if raw >= 100 {
                zero_passes = raw / 100;
            };
            raw.rem_euclid(100)
        };
        ans += zero_passes;
    }
    Ok(ans)
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = count_zeros(reader)?;

    println!("{ans}");

    return Ok(());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fn() {}
}
