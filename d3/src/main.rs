use anyhow;
use std::collections::HashSet;
use std::io::{stdin, Read};

fn string_to_vec(input: String) -> Vec<String> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.to_owned())
        .collect::<Vec<String>>()
}

fn rotate_bins(bins: &Vec<String>) -> Vec<String> {
    let length = bins[0].len();
    let mut res = Vec::new();
    for i in 0..length {
        let s = bins
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect::<String>();
        res.push(s);
    }
    res
}

fn common(line: &str, most: bool) -> char {
    let zero_count = line.chars().filter(|c| *c == '0').count();
    let one_count = line.chars().filter(|c| *c == '1').count();
    match (zero_count > one_count, most) {
        (true, true) | (false, false) => '0',
        (false, true) | (true, false) => '1',
    }
}

fn gamma_eps(rot_bins: &Vec<String>) -> (String, String) {
    let gamma_str = rot_bins
        .iter()
        .map(|line| common(line, true))
        .collect::<String>();
    let eps_str = rot_bins
        .iter()
        .map(|line| common(line, false))
        .collect::<String>();
    (gamma_str, eps_str)
}

fn q1(gamma_str: &str, eps_str: &str) -> i32 {
    let gamma = i32::from_str_radix(gamma_str, 2).unwrap();
    let eps = i32::from_str_radix(eps_str, 2).unwrap();
    gamma * eps
}

fn generator(bins: &Vec<String>, ox: bool) -> i32 {
    let mut idx = 0;
    let mut bins_bools: Vec<bool> = vec![true; bins.len()];
    while bins_bools.iter().filter(|b| **b == true).count() > 1 {
        let strs = bins
            .iter()
            .enumerate()
            .filter(|(bidx, _line)| bins_bools[*bidx])
            .map(|(_, line)| line.chars().nth(idx).unwrap())
            .collect::<String>();
        let zero_count = strs.chars().filter(|c| *c == '0').count();
        let one_count = strs.chars().filter(|c| *c == '1').count();

        let target_c = match ox {
            true => {
                if one_count >= zero_count {
                    '1'
                } else {
                    '0'
                }
            }
            false => {
                if zero_count <= one_count {
                    '0'
                } else {
                    '1'
                }
            }
        };

        bins.iter().enumerate().for_each(|(bidx, line)| {
            if bins_bools[bidx] && line.chars().nth(idx).unwrap() != target_c {
                bins_bools[bidx] = false;
            }
        });
        idx += 1;
    }

    let bins_idx = bins_bools.iter().position(|c| *c == true).unwrap();
    i32::from_str_radix(&bins[bins_idx], 2).unwrap()
}

fn q2(bins: &Vec<String>) -> i32 {
    let ox = generator(bins, true);
    let co2 = generator(bins, false);

    ox * co2
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let bins = string_to_vec(buffer);
    let rot_bins = rotate_bins(&bins);

    let (gamma_str, eps_str) = gamma_eps(&rot_bins);

    let q1_res = q1(&gamma_str, &eps_str);
    println!("{}", q1_res);

    let q2_res = q2(&bins);
    println!("{}", q2_res);

    Ok(())
}
