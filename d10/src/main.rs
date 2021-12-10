use anyhow;
use std::collections::VecDeque;
use std::io::{stdin, Read};

fn string_to_vec(input: String) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

fn check(vec: &Vec<Vec<char>>) -> (Vec<char>, Vec<VecDeque<char>>) {
    let mut inmatch = Vec::new();
    let mut remaining = Vec::new();
    for line in vec {
        let mut dq = VecDeque::new();
        let mut has_inmatch = false;
        for each_char in line {
            match (each_char, dq.back()) {
                ('<' | '{' | '(' | '[', _) => dq.push_back(*each_char),
                (current_c, Some(c)) => match (current_c, c) {
                    (')', '(') | (']', '[') | ('}', '{') | ('>', '<') => {
                        dq.pop_back();
                    }
                    _ => {
                        inmatch.push(*current_c);
                        has_inmatch = true;
                        break;
                    }
                },
                (current_c, None) => {
                    inmatch.push(*current_c);
                    has_inmatch = true;
                    break;
                }
            }
        }
        if dq.len() > 0 && !has_inmatch {
            remaining.push(dq);
        }
    }
    (inmatch, remaining)
}

fn q1(inmatch: Vec<char>) -> u32 {
    inmatch
        .into_iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

fn q2(remaining: Vec<VecDeque<char>>) -> u64 {
    let mut scores = remaining
        .into_iter()
        .map(|mut dq| {
            let mut res: u64 = 0;
            while let Some(c) = dq.pop_back() {
                let cs = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
                res = res * 5 + cs;
            }
            res
        })
        .collect::<Vec<u64>>();
    scores.sort();
    if scores.len() % 2 == 0 {
        scores[scores.len() / 2]
    } else {
        scores[(scores.len() - 1) / 2]
    }
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let vec = string_to_vec(buffer);
    let (inmatch, remaining) = check(&vec);

    let q1_res = q1(inmatch);
    println!("{}", q1_res);

    let q2_res = q2(remaining);
    println!("{}", q2_res);

    Ok(())
}
