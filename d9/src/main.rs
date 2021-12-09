use anyhow;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Mat {
    nums: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl Mat {
    fn new(input: String) -> Self {
        let nums: Vec<Vec<u32>> = input
            .split('\n')
            .filter(|line| line.len() > 0)
            .map(|line| {
                line.chars()
                    .filter_map(|n| n.to_digit(10))
                    .collect::<Vec<_>>()
            })
            .collect();

        let rows = nums.len();
        let cols = nums[0].len();
        Mat { nums, rows, cols }
    }

    fn find_lowest(&self) -> Vec<(u32, usize, usize)> {
        let mut res = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut valid_idx = Vec::new();
                if i > 0 {
                    valid_idx.push((i - 1, j));
                }
                if i + 1 < self.rows {
                    valid_idx.push((i + 1, j));
                }
                if j > 0 {
                    valid_idx.push((i, j - 1));
                }
                if j + 1 < self.cols {
                    valid_idx.push((i, j + 1));
                }

                if valid_idx
                    .into_iter()
                    .all(|(idx, jdx)| self.nums[idx][jdx] > self.nums[i][j])
                {
                    res.push((self.nums[i][j], i, j));
                }
            }
        }
        res
    }

    fn find_basins(&self) -> Vec<usize> {
        let lowest = self.find_lowest();
        let mut basin_size = Vec::new();
        for (_, idx, jdx) in lowest {
            let mut idxes = vec![(idx, jdx)];
            let mut last_len = idxes.len();
            // For each idx find the adjacent and insert into set
            // if the last set length is same with current length
            // nothing added in the loop, then skip out
            loop {
                for idx_i in 0..idxes.len() {
                    let (i, j) = idxes[idx_i];
                    let mut valid_idx = Vec::new();
                    if i > 0 && self.nums[i - 1][j] < 9 {
                        valid_idx.push((i - 1, j));
                    }
                    if i + 1 < self.rows && self.nums[i + 1][j] < 9 {
                        valid_idx.push((i + 1, j));
                    }
                    if j > 0 && self.nums[i][j - 1] < 9 {
                        valid_idx.push((i, j - 1));
                    }
                    if j + 1 < self.cols && self.nums[i][j + 1] < 9 {
                        valid_idx.push((i, j + 1));
                    }
                    valid_idx.into_iter().for_each(|ij| {
                        if !idxes.contains(&ij) {
                            idxes.push(ij);
                        }
                    })
                }
                if idxes.len() == last_len {
                    break;
                }
                last_len = idxes.len();
            }
            basin_size.push(idxes.len());
        }
        basin_size
    }
}

fn q1(mat: &Mat) -> u32 {
    mat.find_lowest().into_iter().map(|(n, _, _)| n + 1).sum()
}

fn q2(mat: &Mat) -> usize {
    let mut bss = mat.find_basins();
    bss.sort();
    let bss = bss.into_iter().rev().collect::<Vec<usize>>();
    bss[0] * bss[1] * bss[2]
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let mat = Mat::new(buffer);

    let q1_res = q1(&mat);
    println!("{}", q1_res);

    let q2_res = q2(&mat);
    println!("{}", q2_res);

    Ok(())
}
