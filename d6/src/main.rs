use anyhow;
use std::collections::HashMap;
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

fn q1(nums: &Vec<i32>, days: u32) -> usize {
    let mut nums = nums.clone();
    for _i in 0..days {
        nums.iter_mut().for_each(|n| *n -= 1);
        let add_nums = nums.iter().filter(|n| **n < 0).count();
        nums.extend_from_slice(&vec![8; add_nums]);
        nums.iter_mut().filter(|n| **n < 0).for_each(|n| *n = 6);
    }
    nums.len()
}

fn base(num: i32, days: i32, cache: &mut HashMap<(i32, i32), usize>) -> usize {
    if cache.contains_key(&(num, days)) {
        *cache.get(&(num, days)).unwrap()
    } else {
        let r = if days - num > 0 {
            base(6, days - num - 1, cache) + base(8, days - num - 1, cache)
        } else {
            1
        };
        cache.insert((num, days), r);
        r
    }
}

fn q2(nums: &Vec<i32>, days: i32) -> usize {
    let mut cache: HashMap<(i32, i32), usize> = HashMap::new();
    nums.iter().map(|n| base(*n, days, &mut cache)).sum()
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_line(&mut buffer)?;
    let nums = string_to_vec(buffer);

    let q1_res = q1(&nums, 18);
    println!("{}", q1_res);

    let q2_res = q2(&nums, 256);
    println!("{}", q2_res);

    Ok(())
}
