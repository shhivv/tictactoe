use std::io::stdin;
use anyhow::bail;

#[derive(Clone, Copy, Debug)]
enum Marker {
    O,
    X,
    Empty,
}

#[derive(Debug)]
struct Position(
    Marker,
    Marker,
    Marker,
    Marker,
    Marker,
    Marker,
    Marker,
    Marker,
    Marker,
);

impl Position {
    fn new() -> Self {
        Self::_repeat(Marker::Empty)
    }

    fn _repeat(n: Marker) -> Self {
        Self(n, n, n, n, n, n, n, n, n)
    }

    fn from(notation: &str) -> anyhow::Result<Self> {
        let notation = notation.split('|').collect::<Vec<&str>>();
        let mut cnotation = vec![];

        if notation.len() != 9 {
            bail!("Failed to process notation");
        }

        for c in notation {
            cnotation.push(match c {
                "" => Marker::Empty,
                "x" => Marker::X,
                "o" => Marker::O,
                _ => bail!("Invalid character"),
            })
        }
        Ok(Self(
            cnotation[0],
            cnotation[1],
            cnotation[2],
            cnotation[3],
            cnotation[4],
            cnotation[5],
            cnotation[6],
            cnotation[7],
            cnotation[8],
        ))
    }
}

fn main() {
    println!("...");

    let mut ready = false;
    let mut position = Position::new();

    loop {
        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();

        if command.starts_with("isready") && !ready {
            println!("readyok");
            ready = true;
            continue;
        }

        if ready {
            if command.starts_with("position") {
                let mut split = command.split_ascii_whitespace();
                if split.clone().count() == 1 {
                    continue;
                }
                split.next();
                if let Ok(res) = Position::from(split.next().unwrap()) {
                    position = res;
                }
            } else if command.starts_with("go") {
                dbg!(position);
                unimplemented!("entry")
            }
        }
    }
}
