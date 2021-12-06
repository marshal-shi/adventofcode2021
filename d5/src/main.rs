use anyhow;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn new(input: &str) -> Self {
        let pts = input
            .split(" -> ")
            .filter(|l| l.len() > 0)
            .map(|s| {
                s.split(',')
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();
        Line {
            start: (pts[0][0], pts[0][1]),
            end: (pts[1][0], pts[1][1]),
        }
    }

    // Better to use iterator
    fn touched_pts(&self, only_hv: bool) -> Vec<(i32, i32)> {
        if self.in_count(only_hv) {
            let s_range: Vec<i32> = if self.start.0 < self.end.0 {
                (self.start.0..=self.end.0).collect()
            } else {
                (self.end.0..=self.start.0).rev().collect()
            };
            let e_range: Vec<i32> = if self.start.1 < self.end.1 {
                (self.start.1..=self.end.1).collect()
            } else {
                (self.end.1..=self.start.1).rev().collect()
            };
            let mut res = Vec::new();
            if self.start.0 == self.end.0 || self.start.1 == self.end.1 {
                for x in &s_range {
                    for y in &e_range {
                        res.push((*x, *y));
                    }
                }
            } else {
                for (x, y) in s_range.iter().zip(e_range.iter()) {
                    res.push((*x, *y));
                }
            }
            res
        } else {
            vec![]
        }
    }

    fn in_count(&self, only_hv: bool) -> bool {
        let hv = self.start.0 == self.end.0 || self.start.1 == self.end.1;
        if only_hv {
            hv
        } else {
            hv || ((self.start.0 - self.end.0).abs()
                == (self.start.1 - self.end.1).abs())
        }
    }
}

fn read_lines(input: String) -> Vec<Line> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| Line::new(line))
        .collect::<Vec<Line>>()
}

fn count(lines: &Vec<Line>, only_hv: bool) -> usize {
    let mut scores: HashMap<(i32, i32), i32> = HashMap::new();
    lines.iter().for_each(|line| {
        let pts = line.touched_pts(only_hv);
        for pt in pts {
            let c = scores.entry(pt).or_insert(0);
            *c += 1;
        }
    });

    scores.iter().filter(|(_k, v)| **v > 1).count()
}

fn q1(lines: &Vec<Line>) -> usize {
    count(lines, true)
}

fn q2(lines: &Vec<Line>) -> usize {
    count(lines, false)
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let lines = read_lines(buffer);

    let q1_res = q1(&lines);
    println!("{}", q1_res);

    let q2_res = q2(&lines);
    println!("{}", q2_res);

    Ok(())
}
