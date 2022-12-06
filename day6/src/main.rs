fn get_unique_window_end(inp: &str, size: usize) -> Option<usize> {
    let mut start: usize = 0;
    let v: Vec<char> = inp.chars().collect();
    while start < v.len() - size {
        let end: usize = start + size;
        let mut window = v[start..end].to_vec();
        window.sort();
        window.dedup();
        if window.len() == size {
            return Some(end);
        }
        start += 1;
    }
    None
}

fn main() {
    let inp = include_str!("../data/inp1.txt");
    let window_start = get_unique_window_end(inp, 14);
    dbg!(window_start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_four() {
        let actual = get_unique_window_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        assert_eq!(actual, Some(7));
        let actual = get_unique_window_end("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        assert_eq!(actual, Some(5));
        let actual = get_unique_window_end("nppdvjthqldpwncqszvftbrmjlhg", 4);
        assert_eq!(actual, Some(6));
        let actual = get_unique_window_end("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
        assert_eq!(actual, Some(10));
        let actual = get_unique_window_end("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
        assert_eq!(actual, Some(11));
    }

    #[test]
    fn test_size_14() {
        let actual = get_unique_window_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
        assert_eq!(actual, Some(19));
        let actual = get_unique_window_end("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
        assert_eq!(actual, Some(23));
        let actual = get_unique_window_end("nppdvjthqldpwncqszvftbrmjlhg", 14);
        assert_eq!(actual, Some(23));
        let actual = get_unique_window_end("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
        assert_eq!(actual, Some(29));
        let actual = get_unique_window_end("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);
        assert_eq!(actual, Some(26));
    }
}
