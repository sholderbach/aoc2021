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
        pos.update(line);
    }
    pos
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Position {
    depth: i64,
    x: i64,
    aim: i64,
}

impl Position {
    fn update(&mut self, command: &str) {
        let parts: Vec<&str> = command.trim().split_ascii_whitespace().collect();
        let command = parts[0];
        let param = parts[1].parse::<i64>().unwrap();

        match command {
            "forward" => {
                self.x += param;
                self.depth += self.aim * param;
            }
            "down" => {
                self.aim += param;
            }
            "up" => {
                self.aim -= param;
            }
            _ => {
                panic!("Unknown command")
            }
        }
    }

    fn product(&self) -> i64 {
        self.depth * self.x
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[test]
fn works() {
    assert_eq!(dead_reckoning(TEST_INPUT).product(), 900);
}
