use core::fmt;
use std::{
    cmp::Ordering::{self, Less},
    collections::VecDeque,
    fmt::Debug,
    io::{self, BufRead, Cursor},
};

// This was way too slow for 12, when you look at the big, duh.
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

pub(crate) fn part_1<const N: usize>(nums: Vec<u32>) -> u64 {
    let mut tree = Tree::<_, N>::new(u32::DEFAULT);
    for num in nums {
        tree.insert(num);
    }
    let ans = tree.max();
    let base: u64 = 10;
    let mut res: u64 = 0;
    for (i, n) in ans.iter().rev().enumerate() {
        res += base.pow(i as u32) * (*n as u64);
    }
    res
}

trait Default {
    const DEFAULT: Self;
}

impl Default for i32 {
    const DEFAULT: Self = 0;
}

impl Default for u32 {
    const DEFAULT: Self = 0;
}

impl Default for i64 {
    const DEFAULT: Self = 0;
}

impl Default for usize {
    const DEFAULT: Self = 0;
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Node<T: Ord + Copy + Default> {
    value: T,
    /** Depth of the deepest leaf node among children of this node, relative to the root. */
    deepest_leaf: usize,
    /** List of child nodes. Sorted desc by value. */
    children: VecDeque<Node<T>>,
}

impl<T: Ord + Copy + Default + Debug> Node<T> {
    pub(crate) fn new(val: T, deepest_leaf: usize) -> Self {
        Node {
            value: val,
            deepest_leaf,
            children: VecDeque::new(),
        }
    }

    pub(crate) fn cmp_max_child_to(&self, other: T) -> Option<Ordering> {
        self.children.get(0).map(|child| child.value.cmp(&other))
    }

    fn fmt_with_depth(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        let indent = "  ".repeat(depth);

        writeln!(
            f,
            "{:?}- {:?} (Deepest Leaf: {:?})",
            indent, self.value, self.deepest_leaf
        )?;

        for child in &self.children {
            child.fmt_with_depth(f, depth + 1)?;
        }

        Ok(())
    }
}

impl<T: Debug> fmt::Display for Node<T>
where
    T: Ord + Copy + Default + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_depth(f, 0)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Tree<T: Ord + Copy + Default, const MAX_DEPTH: usize> {
    root: Node<T>,
}

impl<T: Ord + Copy + Default + fmt::Debug, const MAX_DEPTH: usize> Tree<T, MAX_DEPTH> {
    pub fn new(root_value: T) -> Self {
        Tree {
            root: Node::new(root_value, 0),
        }
    }

    pub(crate) fn insert(&mut self, val: T) -> usize {
        Self::insert_internal(val, &mut self.root, /*depth=*/ 0)
    }

    fn insert_internal(val: T, node: &mut Node<T>, depth: usize) -> usize {
        if depth == MAX_DEPTH {
            return depth;
        }
        let mut deepest_child = depth;
        if node.cmp_max_child_to(val).map_or(true, |res| res == Less) {
            node.children.push_front(Node::new(val, depth + 1));
            deepest_child = depth + 1;
        }
        for (index, child) in &mut node.children.iter_mut().enumerate() {
            if index == 0 && deepest_child > depth {
                continue;
            }
            // this both inserts into child nodes and returns their depth. we cant insert into the
            // new node, but we do need its depth
            // probably splitting this up into another check is more sensible
            deepest_child =
                std::cmp::max(Self::insert_internal(val, child, depth + 1), deepest_child)
        }
        node.deepest_leaf = deepest_child;
        // we can prune a bit
        let max_child_node = node.children.get(0).cloned();
        let mut index = 0;
        node.children.retain(|child| {
            let cur_index = index;
            index += 1;
            if max_child_node
                .clone()
                .map_or(false, |n| child.deepest_leaf < n.deepest_leaf)
                && cur_index > 0
            {
                return false;
            }
            true
        });
        return deepest_child;
    }

    pub(crate) fn max(&self) -> [T; MAX_DEPTH] {
        *Self::max_internal(&self.root, 0, &mut [T::DEFAULT; MAX_DEPTH])
    }

    fn max_internal<'a>(
        node: &'a Node<T>,
        depth: usize,
        ans: &'a mut [T; MAX_DEPTH],
    ) -> &'a mut [T; MAX_DEPTH] {
        let first_complete = node
            .children
            .iter()
            .find(|child| child.deepest_leaf == MAX_DEPTH);
        match first_complete {
            Some(child) => {
                // update ans
                ans[depth] = child.value;
                Self::max_internal(child, depth + 1, ans)
            }
            None => ans,
        }
    }
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = joltage(reader, part_1::<12>)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{joltage, part_1};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "\
        987654321111111
        811111111111119
        234234234234278
        818181911112111";
    const TEST_INPUT_1: &str = "\
        987654321111111";
    const TEST_INPUT_2: &str = "\
        811111111111119";
    const TEST_INPUT_3: &str = "\
        234234234234278";
    // const TEST_INPUT_4: &str = "\
    //     818181911112111";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = joltage(reader, part_1::<2>).expect("");
        assert_eq!(ans, 357);
    }

    #[test]
    fn part2() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = joltage(reader, part_1::<12>).expect("");
        assert_eq!(ans, 3121910778619);
    }
    #[test]
    fn part2_1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT_1));
        let ans = joltage(reader, part_1::<12>).expect("");
        assert_eq!(ans, 987654321111);
    }
    #[test]
    fn part2_2() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT_2));
        let ans = joltage(reader, part_1::<12>).expect("");
        assert_eq!(ans, 811111111119);
    }
    #[test]
    fn part2_3() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT_3));
        let ans = joltage(reader, part_1::<12>).expect("");
        assert_eq!(ans, 434234234278);
    }
    #[test]
    fn part2_4() {
        let reader = io::BufReader::new(Cursor::new("818911"));
        let ans = joltage(reader, part_1::<5>).expect("");
        assert_eq!(ans, 88911);
    }
}
