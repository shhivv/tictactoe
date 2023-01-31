#![warn(clippy::pedantic, clippy::nursery)]

use anyhow::bail;
use std::{array, fmt::Display, io::stdin};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Marker {
    O,
    X,
    Empty,
}

#[derive(Debug)]
struct Position([Marker; 9]);

impl Position {
    const fn new() -> Self {
        Self([Marker::Empty; 9])
    }

    fn from(notation: &str) -> anyhow::Result<Self> {
        let notation = notation.split('|').collect::<Vec<&str>>();
        let mut tokens = vec![];

        if notation.len() != 9 {
            bail!("Failed to process notation");
        }

        for c in notation {
            tokens.push(match c {
                "" => Marker::Empty,
                "x" => Marker::X,
                "o" => Marker::O,
                _ => bail!("Invalid character"),
            });
        }
        Ok(Self(array::from_fn(|i| tokens[i])))
    }

    fn winning(&self) -> Option<Marker> {
        let winning_positions = vec![
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for pos in winning_positions {
            if self.0[pos[0]] == self.0[pos[1]]
                && self.0[pos[1]] == self.0[pos[2]]
                && self.0[pos[0]] != Marker::Empty
            {
                return Some(self.0[pos[0]]);
            }
        }

        None
    }

    fn player(&self) -> Marker {
        let x = self.0.iter().filter(|&x| *x == Marker::X);
        let o = self.0.iter().filter(|&o| *o == Marker::O);

        if x.count() <= o.count() {
            Marker::X
        } else {
            Marker::O
        }
    }

    fn draw(&self) -> bool {
        let empty = self.0.iter().filter(|&e| *e == Marker::Empty);

        if self.winning().is_none() && empty.count() == 0 {
            return true;
        }
        false
    }
}

impl Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::O => "o",
                Self::X => "x",
                Self::Empty => " ",
            }
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [m0, m1, m2, m3, m4, m5, m6, m7, m8] = &self.0;
        write!(
            f,
            r#"
{m0}|{m1}|{m2}
{m3}|{m4}|{m5}
{m6}|{m7}|{m8}        
        "#
        )
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
                    println!("{:?}", position);
                }
            } else if command.starts_with("go") {
                println!("{}", position);
                if let Some(marker) = position.winning() {
                    println!("winner: {:?}", marker);
                    continue;
                } else if position.draw() {
                    println!("draw");
                    continue;
                }
                println!("plays: {:?}", position.player());
            }
        }
    }
}
