const PRIORITY: [&str; 52] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];

fn get_priority(item: &str) -> usize {
    let index = PRIORITY
        .iter()
        .position(|&x| x == item)
        .unwrap();
    return index + 1;
}

fn get_duplicate_item(first_half: &str, second_half: &str) -> Option<String> {
    for c in first_half.chars() {
        if second_half.contains(c) {
            return Some(c.to_string());

        };
    }
    return None;
}

fn get_badge(group: &Vec<&str>) -> Option<String> {
    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            return Some(c.to_string());
        }
    }
    return None;
}

fn main() {
    let input = include_str!["../input/input.txt"];

    let mut lines: Vec<&str> = input
        .lines()
        .map(|line| line)
        .collect();

    let mut groups: Vec<Vec<&str>> = vec![];
    while lines.len() > 0 {
        let group = lines.split_off(lines.len() - 3);
        groups.push(group);
    }

    let priorities: Vec<usize> = groups
        .iter()
        .map(|group| get_badge(group).unwrap())
        .map(|badge| get_priority(&badge))
        .collect();

    // let priorities: Vec<usize> = input
    //     .lines()
    //     .map(|line| {
    //         let split = line.split_at(line.len()/2);
    //         let dup_item = get_duplicate_item(split.0, split.1).unwrap();
    //         return get_priority(&dup_item);
    //     })
    //     .collect();

    println!("{:?}", priorities.iter().sum::<usize>());
}
