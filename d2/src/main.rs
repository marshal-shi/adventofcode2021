use anyhow;
use std::io::{stdin, Read};

fn string_to_vec(input: String) -> Vec<(String, i32)> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|vec| (vec[0].to_owned(), vec[1].parse::<i32>().unwrap()))
        .collect::<Vec<(String, i32)>>()
}

fn q1(cmds: &[(String, i32)]) -> i32 {
    let mut h = 0;
    let mut v = 0;
    cmds.iter().for_each(|(cmd, step)| {
        if cmd == "forward" {
            h += step;
        } else if cmd == "up" {
            v -= step
        } else if cmd == "down" {
            v += step
        }
    });
    h * v
}

fn q2(cmds: &[(String, i32)]) -> i32 {
    let mut hpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    cmds.iter().for_each(|(cmd, step)| {
        if cmd == "forward" {
            hpos += step;
            depth += step * aim;
        } else if cmd == "up" {
            aim -= step
        } else if cmd == "down" {
            aim += step
        }
    });
    hpos * depth
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let cmds = string_to_vec(buffer);

    let q1_res = q1(&cmds);
    println!("{}", q1_res);

    let q2_res = q2(&cmds);
    println!("{}", q2_res);
    Ok(())
}
