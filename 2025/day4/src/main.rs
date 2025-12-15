use std::{
    collections::HashMap,
    io::{self, BufRead, Cursor},
    ops::{Add, Div, Rem},
};

pub(crate) fn forklift<F>(reader: io::BufReader<Cursor<&str>>, part: F) -> io::Result<i32>
where
    F: Fn(&mut Vec<Point<i32>>, &mut HashMap<Point<i32>, Option<i32>>, (i32, i32)) -> i32,
{
    let mut points: Vec<Option<Point<i32>>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut index: i32 = -1;
    let mut rolls: HashMap<Point<i32>, Option<i32>> = HashMap::new();
    for (line_no, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        height += 1;
        if line_no == 0 {
            width = line.len();
        }
        let trimmed = line.trim();
        points.extend(trimmed.chars().map(|char| {
            index += 1;
            let point = Point::from_index(index, width as i32);
            if char == '@' {
                rolls.insert(point, Some(0));
                Some(point)
            } else {
                rolls.insert(point, None);
                None
            }
        }));
    }
    let mut removes: Vec<Point<i32>> = points
        .iter()
        .filter_map(|pos_roll| {
            pos_roll.map(|roll| {
                let count = EIGHT_CELLS
                    .iter()
                    .filter_map(|offset| {
                        validate_grid(roll + *offset, (height as i32, width as i32))
                            .and_then(|y| points[to_index(y, width as i32)])
                    })
                    .count();
                PointCount { p: roll, count }
            })
        })
        .filter(|p| p.count < 4)
        .map(|p| p.p)
        .collect();
    Ok(part(&mut removes, &mut rolls, (height as i32, width as i32)))
}

pub(crate) fn part_1(
    removes: &mut Vec<Point<i32>>,
    _: &mut HashMap<Point<i32>, Option<i32>>,
    _: (i32, i32),
) -> i32 {
    removes.len() as i32
}

pub(crate) fn part_2(
    _: &mut Vec<Point<i32>>,
    _: &mut HashMap<Point<i32>, Option<i32>>,
    _: (i32, i32),
) -> i32 {
    10
}

const EIGHT_CELLS: [Point<i32>; 8] = [
    Point { y: 1, x: 0 },   // down
    Point { y: -1, x: 0 },  // up
    Point { y: 0, x: 1 },   // right
    Point { y: 0, x: -1 },  // left
    Point { y: 1, x: 1 },   // down right
    Point { y: 1, x: -1 },  // down left
    Point { y: -1, x: 1 },  // up right
    Point { y: -1, x: -1 }, // up left
];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point<T> {
    y: T,
    x: T,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct PointCount<T> {
    p: Point<T>,
    count: usize,
}

impl<T: Copy + Div<Output = T> + Rem<Output = T>> Point<T> {
    pub(crate) fn from_index(i: T, w: T) -> Self {
        to_grid(i, w)
    }
}

// test

impl<T: Copy + Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    #[inline]
    fn add(self, rhs: Point<T>) -> Point<T> {
        Point {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

/// Assumes a valid grid within the index.
fn to_index(p: Point<i32>, w: i32) -> usize {
    (p.y * w + p.x % w) as usize
}

/// (y, x)
fn to_grid<T: Copy + Div<Output = T> + Rem<Output = T>>(i: T, w: T) -> Point<T> {
    Point { y: i / w, x: i % w }
}

fn validate_grid(p: Point<i32>, (h, w): (i32, i32)) -> Option<Point<i32>> {
    if p.x >= w || p.x < 0 || p.y >= h || p.y < 0 {
        return None;
    }
    Some(p)
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = forklift(reader, part_2)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{forklift, part_1};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "\
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = forklift(reader, part_1).expect("");
        assert_eq!(ans, 13);
    }

    // #[test]
    // fn part2() {
    //     let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
    //     let ans = forklift(reader, part_2).expect("");
    //     assert_eq!(ans, 43);
    // }
}
