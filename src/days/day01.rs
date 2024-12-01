use std::collections::HashMap;

fn p1(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    lists
        .0
        .iter()
        .zip(lists.1.iter())
        .fold(0, |acc, t| acc + t.0.abs_diff(*t.1))
}

fn p2(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    let map: HashMap<u32, u32> = lists.1.iter().fold(HashMap::new(), |mut acc, x| {
        acc.entry(*x).and_modify(|c| *c += 1).or_insert(1);
        acc
    });
    lists
        .0
        .iter()
        .fold(0, |acc, x| acc + (x * map.get(x).unwrap_or(&0)))
}
pub fn solve(input: String) -> (u32, u32) {
    let mut lists: (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let r: Vec<&str> = l.split("   ").collect();
            return (
                r.first().expect("bad line").parse::<u32>().unwrap(),
                r.last().expect("bad line").parse::<u32>().unwrap(),
            );
        })
        .unzip();
    lists.0.sort();
    lists.1.sort();
    let lists = lists;
    let p1 = p1(&lists);
    let p2 = p2(&lists);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(
            solve(String::from(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            (11, 31)
        );
    }
}
