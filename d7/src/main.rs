use anyhow;
use std::io::{stdin, Read};

fn string_to_vec(mut input: String) -> Vec<i32> {
    if input.ends_with('\n') {
        input.remove(input.len() - 1); // '\n' at the end
    }
    input
        .split(',')
        .filter_map(|n| n.parse::<i32>().ok())
        .collect()
}

fn q1(nums: &Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    nums.sort();
    if nums.len() % 2 == 0 {
        let left_median = nums[nums.len() / 2];
        let right_median = nums[nums.len() / 2 + 1];

        let left_median_fuel =
            nums.iter().map(|n| (n - left_median).abs()).sum();
        let right_median_fuel =
            nums.iter().map(|n| (n - right_median).abs()).sum();
        if left_median_fuel < right_median_fuel {
            left_median_fuel
        } else {
            right_median_fuel
        }
    } else {
        let median = nums[(nums.len() + 1) / 2];
        nums.iter().map(|n| (n - median).abs()).sum()
    }
}

fn q2(nums: &Vec<i32>) -> i32 {
    // not ideal solution.
    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();

    let mut res = nums
        .iter()
        .map(|n| (1..=(n - min).abs()).sum::<i32>())
        .sum();

    for i in (min + 1)..=*max {
        let t = nums.iter().map(|n| (1..=(n - i).abs()).sum::<i32>()).sum();
        if t < res {
            res = t;
        }
    }
    res
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_line(&mut buffer)?;
    let nums = string_to_vec(buffer);

    let q1_res = q1(&nums);
    println!("{}", q1_res);

    let q2_res = q2(&nums);
    println!("{}", q2_res);

    Ok(())
}
