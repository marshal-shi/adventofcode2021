use anyhow;
use std::io::{stdin, Read};

fn string_to_vec(input: String) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn q1(nums: &Vec<Vec<u32>>) -> u32 {
    let width = nums[0].len();
    let height = nums.len();
    let mut paths = vec![vec![u32::MAX - 9; width]; height];
    loop {
        let mut has_update = false;

        // Dijkstra's algo
        for i in 0..height {
            for j in 0..width {
                let cn = nums[i][j];
                let p1 = if i > 0 {
                    cn + paths[i - 1][j]
                } else {
                    u32::MAX
                };
                let p2 = if j > 0 {
                    cn + paths[i][j - 1]
                } else {
                    u32::MAX
                };
                let p3 = if i < height - 1 {
                    cn + paths[i + 1][j]
                } else {
                    u32::MAX
                };
                let p4 = if j < width - 1 {
                    cn + paths[i][j + 1]
                } else {
                    u32::MAX
                };

                let pv = if i == 0 && j == 0 {
                    0
                } else {
                    let ps = [p1, p2, p3, p4];
                    *ps.iter().min().unwrap()
                };
                if paths[i][j] != pv {
                    has_update = true;
                    paths[i][j] = pv;
                }
            }
        }

        if !has_update {
            break;
        }
    }

    paths[height - 1][width - 1]
}

fn q2(nums: &Vec<Vec<u32>>) -> u32 {
    let width = nums[0].len();
    let height = nums.len();
    let mut new_nums = vec![vec![0u32; width * 5]; height * 5];
    for i in 0..height {
        for j in 0..width {
            let n = nums[i][j];
            for li in 0..5 {
                for lj in 0..5 {
                    let mut new_n = n + li as u32 + lj as u32;
                    if new_n > 9 {
                        new_n -= 9;
                    }
                    new_nums[li * width + i][lj * height + j] = new_n;
                }
            }
        }
    }
    q1(&new_nums)
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let nums = string_to_vec(buffer);

    let q1_res = q1(&nums);
    println!("{}", q1_res);

    let q2_res = q2(&nums);
    println!("{}", q2_res);

    Ok(())
}
