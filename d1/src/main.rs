use anyhow;
use std::io::{stdin, Read};

fn string_to_vec(input: String) -> Vec<i32> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
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

#[cfg(test)]
mod tests {
    use super::{q1, q2};

    #[test]
    fn q1_empty_slice() {
        let res = q1(&[]);
        assert_eq!(res, 0);
    }

    #[test]
    fn q1_one_elem() {
        let res = q1(&[1]);
        assert_eq!(res, 0);
    }

    #[test]
    fn q1_descresing() {
        let res = q1(&[5, 4, 3, 2, 1]);
        assert_eq!(res, 0);
    }

    #[test]
    fn q1_increasing() {
        let res = q1(&[1, 2, 3, 4, 5]);
        assert_eq!(res, 4);
    }

    #[test]
    fn q1_hybrid() {
        let res = q1(&[3, 2, 4, 4, 5]);
        assert_eq!(res, 2);
    }

    #[test]
    fn q2_empty_slice() {
        let res = q2(&[]);
        assert_eq!(res, 0);
    }

    #[test]
    fn q2_3_elems() {
        let res = q2(&[1, 2, 3]);
        assert_eq!(res, 0);
    }

    #[test]
    fn q2_descresing() {
        let res = q2(&[5, 4, 3, 2, 1]);
        assert_eq!(res, 0);
    }

    #[test]
    fn q2_increasing() {
        let res = q2(&[1, 2, 3, 4, 5]);
        assert_eq!(res, 2);
    }

    #[test]
    fn q2_hybrid() {
        let res = q2(&[3, 2, 4, 1, 5]);
        assert_eq!(res, 1);
    }
}
