use crate::{Solution, SolutionPair};
use std::{error::Error, fs::read_to_string, str::FromStr};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Action {
    type Err = Box<dyn Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("ERROR: Action identifiers have exactly one letter.".into());
        }
        match s.chars().next().unwrap() {
            'A' => Ok(Action::Rock),
            'X' => Ok(Action::Rock),
            'B' => Ok(Action::Paper),
            'Y' => Ok(Action::Paper),
            'C' => Ok(Action::Scissors),
            'Z' => Ok(Action::Scissors),
            _ => Err("ERROR: Unknown Action indentifier".into()),
        }
    }
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl FromStr for Outcome {
    type Err = Box<dyn Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("ERROR: Outcome identifiers have exactly one letter.".into());
        }
        match s.chars().next().unwrap() {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("ERROR: Unknown Outcome indentifier".into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    player_1: Action,
    player_2: Action,
}

impl Game {
    fn play(&self) -> u64 {
        let shape_score = self.player_2 as u64;
        let game_score = match self {
            Game { player_1, player_2 } if player_1 == player_2 => 3,
            Game {
                player_1: Action::Rock,
                player_2: Action::Paper,
            } => 6,
            Game {
                player_1: Action::Paper,
                player_2: Action::Scissors,
            } => 6,
            Game {
                player_1: Action::Scissors,
                player_2: Action::Rock,
            } => 6,
            _ => 0,
        };
        game_score + shape_score
    }

    fn from_str_2(s: &str) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let mut parts = s.split_ascii_whitespace();
        let action = parts.next().ok_or("ERROR: Input too short".to_string())?;
        let outcome_ident = parts.next().ok_or("ERROR: Input too short".to_string())?;

        let player_1: Action = action.parse()?;
        let outcome = outcome_ident.parse()?;

        Ok(match (player_1, outcome) {
            (_, Outcome::Draw) => Game {
                player_1,
                player_2: player_1,
            },
            (Action::Rock, Outcome::Win) => Game {
                player_1,
                player_2: Action::Paper,
            },
            (Action::Rock, Outcome::Loss) => Game {
                player_1,
                player_2: Action::Scissors,
            },
            (Action::Paper, Outcome::Win) => Game {
                player_1,
                player_2: Action::Scissors,
            },
            (Action::Paper, Outcome::Loss) => Game {
                player_1,
                player_2: Action::Rock,
            },
            (Action::Scissors, Outcome::Win) => Game {
                player_1,
                player_2: Action::Rock,
            },
            (Action::Scissors, Outcome::Loss) => Game {
                player_1,
                player_2: Action::Paper,
            },
        })
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let p1 = parts.next().ok_or("ERROR: Input too short".to_string())?;
        let p2 = parts.next().ok_or("ERROR: Input too short".to_string())?;
        Ok(Game {
            player_1: p1.parse()?,
            player_2: p2.parse()?,
        })
    }
}

fn part_1(input: &str) -> u64 {
    let games: Vec<Game> = input.lines().map(|s| s.parse().unwrap()).collect();
    games.iter().map(|g| g.play()).sum()
}

fn part_2(input: &str) -> u64 {
    let games: Vec<Game> = input
        .lines()
        .map(|s| Game::from_str_2(s).unwrap())
        .collect();
    games.iter().map(|g| g.play()).sum()
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = "input/day02.txt";
    let input = read_to_string(INPUT).unwrap();
    let sol1: u64 = part_1(&input);
    let sol2: u64 = part_2(&input);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_part_1() {
    let input = "A Y\nB X\nC Z";
    let sol = part_1(input);
    assert_eq!(sol, 15)
}

#[test]
fn test_part_2() {
    let input = "A Y\nB X\nC Z";
    let sol = part_2(input);
    assert_eq!(sol, 12)
}
