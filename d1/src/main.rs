use std::io::{stdin, Read};

use anyhow;

fn string_to_vec(input: String) -> Vec<i32> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn q1(nums: &[i32]) -> usize {
    nums.windows(2)
        .filter(|two_nums| two_nums[0] < two_nums[1])
        .count()
}

fn q2(nums: &[i32]) -> usize {
    let wins = nums
        .windows(3)
        .map(|three| three.iter().sum())
        .collect::<Vec<i32>>();

    q1(&wins)
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
