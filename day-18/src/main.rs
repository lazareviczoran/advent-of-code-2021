use itertools::Itertools;
use std::{
    cell::RefCell,
    fmt::{self},
    fs::read_to_string,
    iter::Peekable,
    rc::Rc,
    str::Chars,
    time::Instant,
};

fn main() {
    let pairs = read("input.txt");
    println!("part1 solution: {}", sum_all(&pairs));
    println!("part2 solution: {}", find_max_sum_of_2(&pairs, "input.txt"));
}

fn sum_all(pairs: &[SnailfishNumber]) -> usize {
    pairs
        .iter()
        .fold(None, |acc: Option<SnailfishNumber>, pair| match acc {
            Some(item) => Some(item.add(pair)),
            None => Some(pair.clone()),
        })
        .unwrap()
        .pair
        .calculate_magnitude()
}

fn find_max_sum_of_2(pairs: &[SnailfishNumber], filename: &str) -> usize {
    let instant = Instant::now();
    let max_sum_between_2 = (0..pairs.len())
        .permutations(2)
        .map(|idx| {
            // let instant = Instant::now();
            let pairs = read(filename);
            // println!("read input in {} ms", instant.elapsed().as_millis());
            pairs[idx[0]].add(&pairs[idx[1]]).pair.calculate_magnitude()
        })
        .max()
        .unwrap();
    println!("found max in {} ms", instant.elapsed().as_millis());

    max_sum_between_2
}

#[derive(Debug, Clone)]
struct SnailfishNumber {
    pair: PairItem,
    head: Rc<RefCell<Item>>,
    tail: Rc<RefCell<Item>>,
}
impl SnailfishNumber {
    pub fn add(&self, other: &Self) -> Self {
        // let instant = Instant::now();
        let val1 = self.pair.clone();
        let val2 = other.pair.clone();
        self.tail.borrow_mut().right = Some(other.head.clone());
        other.head.borrow_mut().left = Some(self.tail.clone());

        let mut p = PairItem::Pair(vec![val1, val2]);
        p.bump_level();
        p.reduce_till_end();
        let (head, tail) = p.get_head_and_tail();
        // println!(
        //     "completed add operation in {} ms",
        //     instant.elapsed().as_millis()
        // );
        Self {
            pair: p,
            head,
            tail,
        }
    }
}

#[derive(Debug, Clone)]
enum PairItem {
    Value(Rc<RefCell<Item>>),
    Pair(Vec<PairItem>),
}
impl PairItem {
    pub fn reduce_till_end(&mut self) {
        while self.reduce() {}
    }

    pub fn reduce(&mut self) -> bool {
        self.explode() || self.split()
    }

    pub fn explode(&mut self) -> bool {
        let mut new_level = None;
        let mut prev = None;
        let mut next = None;
        if let PairItem::Pair(pairs) = self {
            match &pairs[0..2] {
                [PairItem::Value(item1_ref), PairItem::Value(item2_ref)]
                    if item1_ref.borrow().level == 4 =>
                {
                    let mut item1 = item1_ref.borrow_mut();
                    let mut item2 = item2_ref.borrow_mut();
                    new_level = Some(item1.level - 1);
                    prev = item1.left.take();
                    next = item2.right.take();
                    if let Some(prev_ref) = &mut prev {
                        prev_ref.borrow_mut().value += item1.value;
                    }
                    if let Some(next_ref) = &mut next {
                        next_ref.borrow_mut().value += item2.value;
                    }
                }
                _ => return pairs[0].explode() || pairs[1].explode(),
            }
        }
        match new_level {
            Some(level) => {
                let new_item_ref = Rc::new(RefCell::new(Item::new(
                    0,
                    level,
                    prev.clone(),
                    next.clone(),
                )));
                if let Some(prev_ref) = &mut prev {
                    prev_ref.borrow_mut().right = Some(new_item_ref.clone());
                }
                if let Some(next_ref) = &mut next {
                    next_ref.borrow_mut().left = Some(new_item_ref.clone());
                }
                *self = PairItem::Value(new_item_ref);
                true
            }
            _ => false,
        }
    }

    pub fn split(&mut self) -> bool {
        let mut pairs = vec![];
        match self {
            PairItem::Value(item_ref) => {
                let mut item = item_ref.borrow_mut();
                if item.value >= 10 {
                    let left = Item::new(item.value / 2, item.level + 1, item.left.clone(), None);
                    let left_ref = Rc::new(RefCell::new(left));
                    let right = Item::new(
                        (item.value + 1) / 2,
                        item.level + 1,
                        None,
                        item.right.clone(),
                    );
                    let right_ref = Rc::new(RefCell::new(right));
                    left_ref.borrow_mut().right = Some(right_ref.clone());
                    right_ref.borrow_mut().left = Some(left_ref.clone());

                    if let Some(prev_item) = item.left.as_mut() {
                        prev_item.borrow_mut().right = Some(left_ref.clone());
                    }
                    if let Some(next_item) = item.right.as_mut() {
                        next_item.borrow_mut().left = Some(right_ref.clone());
                    }
                    pairs.push(PairItem::Value(left_ref));
                    pairs.push(PairItem::Value(right_ref));
                }
            }
            PairItem::Pair(pairs) => {
                return pairs[0].split() || pairs[1].split();
            }
        }
        match pairs.len() {
            0 => false,
            _ => {
                *self = PairItem::Pair(pairs);
                true
            }
        }
    }

    pub fn get_head_and_tail(&self) -> (Rc<RefCell<Item>>, Rc<RefCell<Item>>) {
        match self {
            PairItem::Value(item_ref) => {
                let first_item_ref = item_ref.clone();
                let mut curr_ref = item_ref.clone();
                loop {
                    if curr_ref.borrow().right.is_none() {
                        return (first_item_ref, curr_ref);
                    }
                    curr_ref = curr_ref.clone().borrow().right.clone().unwrap();
                }
            }
            PairItem::Pair(pairs) => pairs[0].get_head_and_tail(),
        }
    }

    pub fn bump_level(&mut self) {
        let (head, _tail) = self.get_head_and_tail();
        let mut curr_ref = head;
        loop {
            curr_ref.borrow_mut().level += 1;
            if curr_ref.borrow().right.is_none() {
                break;
            }
            curr_ref = curr_ref.clone().borrow().right.clone().unwrap();
        }
    }

    pub fn calculate_magnitude(&self) -> usize {
        match self {
            PairItem::Value(item_ref) => item_ref.borrow().value,
            PairItem::Pair(pairs) => {
                3 * pairs[0].calculate_magnitude() + 2 * pairs[1].calculate_magnitude()
            }
        }
    }
}

#[derive(Clone)]
struct Item {
    value: usize,
    level: usize,
    initial_level: usize,
    left: Option<Rc<RefCell<Item>>>,
    right: Option<Rc<RefCell<Item>>>,
}
impl fmt::Debug for Item {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!(
            "Item {{ value: {}, level: {} }}",
            self.value, self.level,
        ))
    }
}
impl Item {
    pub fn new(
        value: usize,
        level: usize,
        left: Option<Rc<RefCell<Item>>>,
        right: Option<Rc<RefCell<Item>>>,
    ) -> Self {
        Self {
            value,
            level,
            initial_level: level,
            left,
            right,
        }
    }
}

fn parse_pair(
    iter: &mut Peekable<Chars>,
    prev: &mut Option<Rc<RefCell<Item>>>,
    level: usize,
) -> PairItem {
    iter.next(); // skipping [

    let pair_items = (0..2)
        .map(|_| {
            let res = match iter.peek() {
                Some('[') => parse_pair(iter, prev, level + 1),
                _ => {
                    let item = Item::new(
                        (iter.next().unwrap() as u8 - b'0') as usize,
                        level,
                        prev.clone(),
                        None,
                    );
                    let item_ref = Rc::new(RefCell::new(item));
                    if let Some(prev_item) = prev.as_mut() {
                        prev_item.borrow_mut().right = Some(item_ref.clone());
                    }
                    *prev = Some(item_ref.clone());

                    PairItem::Value(item_ref)
                }
            };
            if iter.peek() == Some(&',') {
                iter.next();
            }
            res
        })
        .collect();

    iter.next(); // skipping ]
    PairItem::Pair(pair_items)
}

fn read(filename: &str) -> Vec<SnailfishNumber> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let pair = parse_pair(&mut l.chars().peekable(), &mut None, 0);
            let (head, tail) = pair.get_head_and_tail();
            SnailfishNumber { pair, head, tail }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::{find_max_sum_of_2, parse_pair, read, sum_all};

    #[test]
    fn magnitude_test() {
        let pair = parse_pair(
            &mut "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .chars()
                .peekable(),
            &mut None,
            0,
        );

        assert_eq!(pair.calculate_magnitude(), 3488);
    }

    #[test]
    fn part1_test() {
        let pairs = read("test-input.txt");
        assert_eq!(sum_all(&pairs), 4140);
    }

    #[test]
    fn part2_test() {
        let mut pairs = read("test-input.txt");
        assert_eq!(find_max_sum_of_2(&mut pairs, "test-input.txt"), 3993);
    }
}
