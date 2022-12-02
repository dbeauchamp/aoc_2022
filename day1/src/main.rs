use std::fs;

fn main() {
    let input = get_input();

    let calories: Vec<&str> = input
        .lines()
        .map(|line| line)
        .collect();

    let mut grouped_calories: Vec<Vec<i32>> = vec![];

    let mut curr: Vec<i32> = vec![];
    for c in &calories {
        if *c == "" {
            grouped_calories.push(curr);
            curr = Vec::new();
        } else {
            curr.push(c.parse().unwrap());
        }

    }

    let totals: Vec<i32> = grouped_calories
        .iter()
        .map(|group| group.iter().sum())
        .collect();

    println!("{:?}", totals.iter().max().unwrap());
}

fn get_input() -> String {
    let path = "./input/input.txt";
    let input = fs::read_to_string(path).unwrap();
    input
}
