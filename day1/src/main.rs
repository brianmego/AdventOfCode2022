use std::fs;

fn main() {
    let inp_str = fs::read_to_string("data/inp2.txt").expect("Unable to load file");
    let mut totals = vec![];
    for group in inp_str.trim().split("\n\n") {
        let mut total = 0;
        for snack in group.split('\n') {
            total += snack.parse::<i32>().unwrap();
        }
        totals.push(total);
    }
    totals.sort();

    let largest = totals[totals.len() - 1];
    let largest_three: i32 = totals.into_iter().rev().take(3).sum();
    println!("Largest: {largest}");
    println!("Largest three: {largest_three}");
}
