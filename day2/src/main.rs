const STRATEGY_GUIDE: [usize; 3] = [3, 6, 0];
const SHAPE_POINTS:[usize; 3] = [1, 2, 3];

fn to_index(c: &str) -> usize {
    let index = match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("no index"),
    };
    return index;
}


struct Round<'a> {
    o: &'a str,
    p: &'a str,
}

impl<'a> Round<'a> {
    fn score(&self) -> usize {
        return match (self.o, self.p) {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
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
