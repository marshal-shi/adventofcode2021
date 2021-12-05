use anyhow;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Board {
    mat: Vec<Vec<i32>>,
    marks: Vec<Vec<bool>>,
}

impl Board {
    fn new(input: &str) -> Self {
        let mat = input
            .split('\n')
            .filter(|l| l.len() > 0)
            .map(|l| {
                l.split(' ')
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();
        let cols = mat[0].len();
        let rows = mat.len();
        Board {
            mat,
            marks: vec![vec![false; cols]; rows],
        }
    }

    fn has_win(&self) -> bool {
        // Rows
        for i in 0..self.marks.len() {
            let row = &self.marks[i];
            if row.iter().all(|v| *v) {
                return true;
            }
        }
        for i in 0..self.marks[0].len() {
            let col = self.marks.iter().map(|l| l[i]).collect::<Vec<bool>>();
            if col.iter().all(|v| *v) {
                return true;
            }
        }
        false
    }

    fn drawn(&mut self, num: i32) {
        self.mat.iter().enumerate().for_each(|(ridx, l)| {
            l.iter().enumerate().for_each(|(cidx, v)| {
                if *v == num {
                    self.marks[ridx][cidx] = true;
                }
            })
        });
    }

    fn not_drawn_sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.marks.len() {
            for j in 0..self.marks[i].len() {
                if !self.marks[i][j] {
                    sum += self.mat[i][j];
                }
            }
        }
        sum
    }
}

fn input_to_struct(input: String) -> (Vec<i32>, Vec<Board>) {
    let blocks = input
        .split("\n\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>();

    let nums = blocks[0]
        .split(',')
        .filter_map(|n| n.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let boards = blocks[1..]
        .iter()
        .map(|b| Board::new(b))
        .collect::<Vec<Board>>();

    (nums, boards)
}

fn q1(nums: &Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for num in nums {
        for b in &mut *boards {
            b.drawn(*num);
            let win = b.has_win();
            if win {
                let sum = b.not_drawn_sum();
                return sum * num;
            }
        }
    }
    0
}

fn q2(nums: &Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    let mut bwins = vec![false; boards.len()];
    let mut remaining_one = false;
    loop {
        for num in nums {
            for (idx, b) in (&mut *boards).iter_mut().enumerate() {
                if bwins[idx] {
                    continue;
                }

                b.drawn(*num);
                let win = b.has_win();
                if win {
                    if remaining_one {
                        return num * b.not_drawn_sum();
                    }
                    bwins[idx] = true;
                }
            }
            remaining_one = bwins.iter().filter(|w| !*w).count() == 1;
        }
    }
    0
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let (nums, mut boards) = input_to_struct(buffer);

    let q1_res = q1(&nums, &mut boards);
    println!("{}", q1_res);

    let q2_res = q2(&nums, &mut boards);
    println!("{}", q2_res);

    Ok(())
}
