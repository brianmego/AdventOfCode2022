use itertools::{Itertools, Chunk};

struct Rucksack {
    compartment1: String,
    compartment2: String,
    raw: String,
}
impl Rucksack {
    fn new(compartment1: &str, compartment2: &str) -> Self {
        Self {
            compartment1: compartment1.to_string(),
            compartment2: compartment2.to_string(),
            raw: format!("{}{}", compartment1, compartment2),
        }
    }

    fn common_item(&self) -> Option<char> {
        let mut common_char: Option<char> = None;
        for c in self.compartment1.clone().into_bytes() {
            if self.compartment2.clone().into_bytes().contains(&c) {
                common_char = Some(c as char);
                break;
            }
        }
        common_char
    }
}

struct RucksackGroup {
    rucksacks: Vec<Rucksack>,
}

impl RucksackGroup {
    fn common_item(&self) -> Option<char> {
        for c in self.rucksacks[0].raw.clone().into_bytes() {
            let mut is_present_in_all = true;
            for sack in &self.rucksacks[1..] {
                if !sack.raw.clone().into_bytes().contains(&c) {
                    is_present_in_all = false;
                    break;
                }
            }
            if is_present_in_all {
                return Some(c as char);
            }
        }
        None
    }
}


fn get_item_priority(c: char) -> usize {
    let priority_order = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    priority_order.find(c).expect("char {c} should be in list")
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let (compartment1, compartment2) = s.split_at(s.len() / 2);
        Rucksack::new(compartment1, compartment2)
    }
}

fn main() -> color_eyre::Result<()> {
    let inp = include_str!("../data/inp1.txt");
    let mut priority_sum: u32 = 0;
    for common_char in inp.lines().map(|line| Rucksack::from(line).common_item()) {
        let common_char = common_char.unwrap();
        priority_sum += get_item_priority(common_char) as u32;
    }
    dbg!(priority_sum);

    let mut grouping_score: u32 = 0;
    for chunk in &inp.lines().chunks(3) {
        let mut rsg = RucksackGroup {rucksacks: vec![]};
        for line in chunk {
            rsg.rucksacks.push(Rucksack::from(line));
        }
        let common_char = rsg.common_item().unwrap();
        grouping_score += get_item_priority(common_char) as u32;
    }
    dbg!(grouping_score);

    Ok(())
}
