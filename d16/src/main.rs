use anyhow;
use std::io::stdin;

fn string_to_bits(input: String) -> Vec<u8> {
    let bchars = input
        .chars()
        .filter(|n| n.is_ascii_hexdigit())
        .map(|n| {
            let v = n.to_digit(16).unwrap();
            format!("{:04b}", v)
        })
        .collect::<String>();
    bchars
        .chars()
        .map(|n| {
            let v = n.to_digit(2).unwrap();
            v as u8
        })
        .collect()
}

fn num(bits: &[u8]) -> usize {
    let s = bits.iter().map(|n| format!("{}", n)).collect::<String>();
    usize::from_str_radix(&s, 2).unwrap()
}

fn q1(bits: &[u8]) -> (usize, usize) {
    // return version sum and total length
    if bits.len() == 0 || bits.iter().all(|n| *n == 0) {
        return (0, 0);
    }

    let version = num(&bits[0..3]);
    let id = num(&bits[3..6]);

    let (version, length) = if id == 4 {
        let mut section_start_idx = 6;
        loop {
            let first_bit = bits[section_start_idx];
            section_start_idx += 5;
            if first_bit == 0 {
                break;
            }
        }
        (version, section_start_idx)
    } else {
        match bits[6] {
            0 => {
                let mut data_start_idx = 7 + 15;
                let len = num(&bits[7..data_start_idx]);
                let total_len = data_start_idx + len;
                let mut versions: Vec<usize> = Vec::new();
                while data_start_idx < total_len {
                    let sub_packets = &bits[data_start_idx..total_len];
                    let (v, l) = q1(sub_packets);
                    data_start_idx += l;
                    versions.push(v);
                }
                let versions_sum: usize = versions.iter().sum();
                (version + versions_sum, total_len)
            }
            1 => {
                let data_start_idx = 7 + 11;
                let times = num(&bits[7..data_start_idx]);
                let mut length = 0;
                let mut versions: Vec<usize> = Vec::new();
                for _i in 0..times {
                    let (v, l) = q1(&bits[(data_start_idx + length)..]);
                    versions.push(v);
                    length += l;
                }
                let versions_sum: usize = versions.iter().sum();
                (version + versions_sum, data_start_idx + length)
            }
            _ => panic!("Should not be here!"),
        }
    };

    (version, length)
}

fn q2(bits: &[u8]) -> (Vec<usize>, usize) {
    // return value list and length
    if bits.len() == 0 || bits.iter().all(|n| *n == 0) {
        return (vec![], 0);
    }

    let id = num(&bits[3..6]);

    let (value_list, length): (Vec<usize>, usize) = if id == 4 {
        let mut section_start_idx = 6;
        let mut values: Vec<u8> = Vec::new();
        loop {
            let first_bit = bits[section_start_idx];
            values.extend(&bits[section_start_idx + 1..section_start_idx + 5]);
            section_start_idx += 5;
            if first_bit == 0 {
                break;
            }
        }
        let value = num(&values);
        return ([value].to_vec(), section_start_idx);
    } else {
        match bits[6] {
            0 => {
                let mut data_start_idx = 7 + 15;
                let len = num(&bits[7..data_start_idx]);
                let total_len = data_start_idx + len;
                let mut values: Vec<usize> = Vec::new();
                while data_start_idx < total_len {
                    let sub_packets = &bits[data_start_idx..total_len];
                    let (v, l) = q2(sub_packets);
                    data_start_idx += l;
                    values.extend(v);
                }
                (values, total_len)
            }
            1 => {
                let data_start_idx = 7 + 11;
                let times = num(&bits[7..data_start_idx]);
                let mut length = 0;
                let mut values: Vec<usize> = Vec::new();
                for _i in 0..times {
                    let (v, l) = q2(&bits[(data_start_idx + length)..]);
                    values.extend(v);
                    length += l;
                }
                (values, data_start_idx + length)
            }
            _ => panic!("Should not be here!"),
        }
    };

    let value = match id {
        0 => value_list.iter().sum(),
        1 => value_list.iter().fold(1, |acc, x| acc * x),
        2 => *value_list.iter().min().unwrap(),
        3 => *value_list.iter().max().unwrap(),
        4 => {
            return (value_list, length);
        }
        5 => (value_list[0] > value_list[1]) as usize,
        6 => (value_list[0] < value_list[1]) as usize,
        7 => (value_list[0] == value_list[1]) as usize,
        _ => panic!("Should not be here!"),
    };

    (vec![value], length)
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let bits = string_to_bits(buffer);

    let q1_res = q1(&bits);
    println!("Q1: {:?}", q1_res.0);

    let q2_res = q2(&bits);
    println!("Q2: {:?}", q2_res.0[0]);

    Ok(())
}
