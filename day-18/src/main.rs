use std::{
    cell::RefCell,
    fmt::{self, format},
    fs::read_to_string,
    iter::Peekable,
    rc::Rc,
    str::Chars,
};

fn main() {
    let pair = read("input.txt");
    println!("part1 solution: {}", pair.calculate_magnitude());
}

#[derive(Debug)]
enum PairItem {
    Value(Rc<RefCell<Item>>),
    Pair(Vec<PairItem>),
}
impl PairItem {
    pub fn reduce_till_end(&mut self) {
        let mut i = 0;
        while self.reduce() {
            self.print_values();
            i += 1;
        }
        println!("\ncompleted after {:?} steps", i);
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
                let new_item_ref = Rc::new(RefCell::new(Item {
                    value: 0,
                    level,
                    left: prev.clone(),
                    right: next.clone(),
                }));
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
                    let left = Item {
                        value: item.value / 2,
                        level: item.level + 1,
                        left: item.left.clone(),
                        right: None,
                    };
                    let left_ref = Rc::new(RefCell::new(left));
                    let right = Item {
                        value: (item.value + 1) / 2,
                        level: item.level + 1,
                        left: None,
                        right: item.right.clone(),
                    };
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

    pub fn bump_level(&mut self, return_last: bool) -> Rc<RefCell<Item>> {
        match self {
            PairItem::Value(item_ref) => {
                let mut first_item_ref = item_ref.clone();
                first_item_ref.borrow_mut().level += 1;
                let mut curr_ref = first_item_ref.clone();
                while curr_ref.borrow().right.is_some() {
                    curr_ref = curr_ref.clone().borrow().right.clone().unwrap();
                    curr_ref.borrow_mut().level += 1;
                }
                if return_last {
                    curr_ref.clone()
                } else {
                    first_item_ref.clone()
                }
            }
            PairItem::Pair(pairs) => pairs[0].bump_level(return_last),
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

    pub fn add(mut self, mut other: Self) -> Self {
        let last_p1 = self.bump_level(true);
        let first_p2 = other.bump_level(false);
        last_p1.borrow_mut().right = Some(first_p2.clone());
        first_p2.borrow_mut().left = Some(last_p1);
        let mut p = Self::Pair(vec![self, other]);
        p.reduce_till_end();
        p
    }

    pub fn print_values(&self) {
        match self {
            PairItem::Value(item_ref) => {
                let mut curr_ref = item_ref.clone();
                let mut s = format!(
                    "  v:{} (l:{})  ",
                    curr_ref.borrow().value,
                    curr_ref.borrow().level
                );

                while curr_ref.borrow().right.is_some() {
                    curr_ref = curr_ref.clone().borrow().right.clone().unwrap();
                    s.push_str(&format!(
                        "  v:{} (l:{})  ",
                        curr_ref.borrow().value,
                        curr_ref.borrow().level
                    ));
                }
                println!("{}", s);
            }
            PairItem::Pair(pairs) => pairs[0].print_values(),
        }
    }
}

struct Item {
    value: usize,
    level: usize,
    left: Option<Rc<RefCell<Item>>>,
    right: Option<Rc<RefCell<Item>>>,
}
// impl fmt::Display for Item {
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str(&format!(
//             "Item {{ value: {}, level: {}, left: {}, right: {} }}",
//             self.value,
//             self.level,
//             match &self.left {
//                 None => "None".into(),
//                 Some(item_ref) => {
//                     let item = item_ref.borrow();
//                     format!("Some ({:?})", item.value)
//                 }
//             },
//             match &self.right {
//                 None => "None".into(),
//                 Some(item_ref) => {
//                     let item = item_ref.borrow();
//                     format!("{:?}", item.value)
//                 }
//             }
//         ))
//     }
// }
impl fmt::Debug for Item {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!(
            "Item {{ value: {}, level: {} }}",
            self.value, self.level,
        ))
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
                    let item = Item {
                        value: (iter.next().unwrap() as u8 - b'0') as usize,
                        level,
                        left: prev.clone(),
                        right: None,
                    };
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

fn read(filename: &str) -> PairItem {
    let mut pairs = read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| parse_pair(&mut l.chars().peekable(), &mut None, 0))
        .collect::<Vec<_>>();

    let p1 = pairs.remove(0);
    let p2 = pairs.remove(0);

    pairs
        .into_iter()
        .fold(p1.add(p2), |prev, pair| prev.add(pair))
}

#[cfg(test)]
mod tests {
    use crate::{parse_pair, read};

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
        let pair = read("test-input2.txt");
        pair.print_values();
        assert_eq!(pair.calculate_magnitude(), 4140);
    }
}
