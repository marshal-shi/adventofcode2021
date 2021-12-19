use anyhow;
use std::io::{stdin, Read};

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

fn num(b: &[u8]) -> usize {
    b.iter()
        .enumerate()
        .map(|(i, &x)| (x as usize) << (b.len() - i - 1))
        .sum()
}

fn q1(mut bits: &[u8]) -> (usize, &[u8]) {
    let mut ver = num(&bits[0..3]);
    let typ = num(&bits[3..6]);
    bits = &bits[6..];
    if typ == 4 {
        (0..)
            .take_while(|_| {
                let cont = bits[0] == 1;
                bits = &bits[5..];
                cont
            })
            .count();
        return (ver, bits);
    }

    let split = bits.split_at(1);
    bits = split.1;
    if split.0[0] == 0 {
        let len = num(&bits[0..15]);
        bits = &bits[15..];
        let (mut payload, extra) = bits.split_at(len);
        bits = extra;
        while !payload.is_empty() {
            let packet = q1(payload);
            payload = packet.1;
            ver += packet.0;
        }
    } else {
        let split = bits.split_at(11);
        bits = split.1;
        ver += (0..num(split.0))
            .map(|_| {
                let packet = q1(bits);
                bits = packet.1;
                packet.0
            })
            .sum::<usize>();
    }
    (ver, bits)
}

fn q2(mut bits: &[u8]) -> (Vec<usize>, &[u8]) {
    let typ = num(&bits[3..6]);
    bits = &bits[6..];
    if typ == 4 {
        let mut num_bits = vec![];
        (0..)
            .take_while(|_| {
                let cont = bits[0] == 1;
                num_bits.extend(&bits[1..5]);
                bits = &bits[5..];
                cont
            })
            .count();
        return (vec![num(&num_bits).into()], bits);
    }

    let mut nums = vec![];
    let split = bits.split_at(1);
    bits = split.1;
    if split.0[0] == 0 {
        let len = num(&bits[0..15]) as usize;
        bits = &bits[15..];
        let (mut payload, extra) = bits.split_at(len);
        bits = extra;
        while !payload.is_empty() {
            let packet = q2(payload);
            payload = packet.1;
            nums.extend_from_slice(&packet.0);
        }
    } else {
        let split = bits.split_at(11);
        bits = split.1;
        (0..num(split.0)).for_each(|_| {
            let packet = q2(bits);
            bits = packet.1;
            nums.extend_from_slice(&packet.0);
        });
    }

    match typ {
        0 => (vec![nums.iter().sum()], bits),
        1 => (vec![nums.iter().product()], bits),
        2 => (vec![*nums.iter().min().unwrap()], bits),
        3 => (vec![*nums.iter().max().unwrap()], bits),
        5 => (vec![(nums[0] > nums[1]) as usize], bits),
        6 => (vec![(nums[0] < nums[1]) as usize], bits),
        7 => (vec![(nums[0] == nums[1]) as usize], bits),
        _ => unreachable!(),
    }
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let bits = string_to_bits(buffer);

    let q1_res = q1(&bits);
    println!("{:?}", q1_res.0);

    let q2_res = q2(&bits);
    println!("{}", q2_res.0[0]);

    Ok(())
}
