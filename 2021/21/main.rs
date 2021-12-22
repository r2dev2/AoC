use std::cmp::{max, min};
use std::collections::{HashMap};
use std::result::Result;

#[derive(Debug, Hash, Eq, PartialEq)]
struct GameState {
    player1_pos: i32,
    player2_pos: i32,
    player1_score: i32,
    player2_score: i32
}

impl GameState {
    fn get_player(&self, turn: i32) -> (i32, i32) {
        if turn == 0 {
            (self.player1_pos, self.player1_score)
        } else {
            (self.player2_pos, self.player2_score)
        }
    }

    fn next_state(&self, player_to_update: i32, pos: i32, score: i32) -> GameState {
        if player_to_update == 0 {
            GameState {
                player1_pos: pos, player1_score: score, ..*self
            }
        } else {
            GameState {
                player2_pos: pos, player2_score: score, ..*self
            }
        }
    }
}

fn parse(input: &str) -> Result<GameState, &'static str> {
    let nums: Vec<i32> = input
        .split(|c: char| !c.is_numeric()).flat_map(|c| c.parse::<i32>()).collect();
    match nums.len() {
        4 => Ok(GameState {
            player1_pos: nums[1], player2_pos: nums[3], player1_score: 0, player2_score: 0
        }),
        _ => Err("Did not parse 4 numbers")
    }
}

fn part1(input: &str) -> Result<i32, &'static str> {
    let mut state = parse(&input)?;
    let mut player = 0;
    let mut rolls = 0;
    let mut die = (1..101).cycle();
    while max(state.player1_score, state.player2_score) < 1000 {
        let movement = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
        match player {
            0 => {
                state.player1_pos = (state.player1_pos + movement - 1) % 10 + 1;
                state.player1_score += state.player1_pos;
            },
            1 => {
                state.player2_pos = (state.player2_pos + movement - 1) % 10 + 1;
                state.player2_score += state.player2_pos;
            }
            _ => {},
        }
        player = 1 - player;
        rolls += 1;
    }

    Ok(min(state.player1_score, state.player2_score) * rolls * 3)
}

fn part2(input: &str) -> Result<usize, &'static str> {
    let initial_state = parse(&input)?;
    let possible_rolls = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let mut turn = 0;
    let mut wins = [0, 0];
    let mut all_states = HashMap::<GameState, usize>::new();
    all_states.insert(initial_state, 1);

    while !all_states.is_empty() {
        let mut new_all_states = HashMap::<GameState, usize>::new();

        for (state, amount) in all_states.iter() {
            let t = turn;
            let (pos, score) = state.get_player(turn);
            let new_states = possible_rolls
                .iter()
                .map(|(sum, amt)| ((pos + sum - 1) % 10 + 1, amt * amount))
                .map(|(new_pos, amt)| (new_pos, score + new_pos, amt))
                .map(|(new_pos, new_score, amt)| (state.next_state(t, new_pos, new_score), amt));
            for (new_state, amt) in new_states {
                if new_state.get_player(turn).1 < 21 {
                    let prev_amt = new_all_states.get(&new_state).unwrap_or(&0);
                    new_all_states.insert(new_state, prev_amt + amt);
                } else {
                    wins[turn as usize] += amt;
                }
            }
        }

        turn = 1 - turn;
        all_states = new_all_states;
    }

    Ok(max(wins[0], wins[1]))
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./game.txt");
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
