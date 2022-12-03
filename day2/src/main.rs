struct Round<'a> {
    o: &'a str,
    p: &'a str,
}

impl<'a> Round<'a> {
    fn score(&self) -> usize {
        // X lose
        // Y draw
        // Z win
        return match (self.o, self.p) {
            ("A", "X") => 3 + 0,
            ("A", "Y") => 1 + 3,
            ("A", "Z") => 2 + 6,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 2 + 0,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 1 + 6,
            _ => unreachable!("impossible combination")
        }
    }
}

fn main() {
    let input = include_str!["../input/input.txt"];
    let scores: Vec<usize> =input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            let round = Round {
                o: split[0],
                p: split[1],
            };
            return round.score();
        })
        .collect();

    println!("{:?}", scores.iter().sum::<usize>());
}
