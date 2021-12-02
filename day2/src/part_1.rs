use std::ops::Add;
use std::fs;

fn main() {
    let commands = fs::read_to_string("input").unwrap();
    let pos = dead_reckoning(&commands);
    println!("{:?}", pos);
    println!("{}", pos.product());
}


fn dead_reckoning(commands: &str) -> Position {
    let mut pos = Position::default();
    for line in commands.lines() {
        let delta = Position::from_str_command(line).unwrap();
        pos = pos + delta;
    }
    pos
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Position {
    depth: i64,
    x: i64,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            depth:self.depth + rhs.depth,
            x: self.x + rhs.x,
        }
    }
}

impl Position {
    fn from_str_command(command: &str) -> Option<Position> {
        let parts: Vec<&str> =command.trim().split_ascii_whitespace().collect();
        let command = parts.get(0)?; 
        let param = parts.get(1)?.parse::<i64>().ok()?;

        match *command {
            "forward" => Some(Position{depth:0, x: param}),
            "down" => Some(Position{depth: param, x: 0}),
            "up" => Some(Position{depth: -param, x: 0}),
            _ => None

        }

    }

    fn product(&self) -> i64 {
        self.depth * self.x
    }
}

const test_input: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[test]
fn works() {
    assert_eq!(dead_reckoning(test_input), Position{depth: 10, x: 15});

    assert_eq!(dead_reckoning(test_input).product(), 150);
}