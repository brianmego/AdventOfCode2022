use std::iter::IntoIterator;

#[derive(Debug, Clone)]
struct SectionList(Vec<u8>);
impl TryFrom<&str> for SectionList {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (min, max) = value.split_once('-').expect("Expected a - delimiter");
        let min: u8 = min.parse()?;
        let max: u8 = max.parse()?;
        let mut range = Vec::new();
        for i in min..=max {
            range.push(i)
        }
        Ok(Self(range))
    }

}
impl IntoIterator for SectionList {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl SectionList {

    fn fully_intersects(&self, section_list: SectionList) -> bool {
        for i in section_list.into_iter() {
            if !self.0.contains(&i) {
                return false;
            }
        }
        true
    }

    fn partially_intersects(&self, section_list: SectionList) -> bool {
        for i in section_list.into_iter() {
            if self.0.contains(&i) {
                return true;
            }
        }
        false
    }
}

fn main() -> color_eyre::Result<()> {
    let inp = include_str!("../data/inp1.txt");
    let mut full_intersection_count = 0;
    let mut partial_intersection_count = 0;
    for line in inp.lines() {
        let (grp1, grp2) = line.split_once(',').expect("Expecting a , delimiter");
        let sec1 = SectionList::try_from(grp1)?;
        let sec2 = SectionList::try_from(grp2)?;
        if sec1.fully_intersects(sec2.clone()) || sec2.fully_intersects(sec1.clone()) {
            full_intersection_count += 1;
        }
        if sec1.partially_intersects(sec2.clone()) || sec2.partially_intersects(sec1.clone()) {
            partial_intersection_count += 1;
        }
    }
    println!("Full Intersections: {full_intersection_count}");
    println!("Partial Intersections: {partial_intersection_count}");
    Ok(())
}
