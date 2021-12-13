use anyhow;
use std::io::{stdin, Read};

fn folder_idx(n: usize, f: usize) -> usize {
    if n < f {
        n
    } else {
        f - (n - f)
    }
}

#[derive(Debug)]
struct Dots {
    lst: Vec<(usize, usize)>,
    folds: Vec<(char, usize)>,
}

impl Dots {
    fn new(input: String) -> Self {
        let lines = input
            .split('\n')
            .filter(|line| line.len() > 0)
            .collect::<Vec<&str>>();

        let mut lst = Vec::new();
        let mut folds = Vec::new();
        for line in lines {
            if line.starts_with("fold") {
                let sp = line.split('=').collect::<Vec<&str>>();
                let c = sp[0].chars().last().unwrap();
                let n = sp[1].parse::<usize>().unwrap();
                folds.push((c, n));
            } else {
                let sp = line.split(',').collect::<Vec<&str>>();
                let i = sp[0].parse::<usize>().unwrap();
                let j = sp[1].parse::<usize>().unwrap();
                lst.push((i, j));
            }
        }

        Dots { lst, folds }
    }

    fn folder_once(
        lst: &Vec<(usize, usize)>,
        folder: (char, usize),
    ) -> Vec<(usize, usize)> {
        let mut new_lst = Vec::new();
        for (i, j) in lst {
            let (ni, nj) = if folder.0 == 'x' {
                (folder_idx(*i, folder.1), *j)
            } else {
                (*i, folder_idx(*j, folder.1))
            };

            if !new_lst.contains(&(ni, nj)) {
                new_lst.push((ni, nj));
            }
        }
        new_lst
    }
}

fn q1(dots: &Dots) -> usize {
    let lst = Dots::folder_once(&dots.lst, dots.folds[0]);
    lst.len()
}

fn q2(dots: &Dots) {
    let mut lst = dots.lst.clone();
    for f in &dots.folds {
        lst = Dots::folder_once(&lst, *f);
    }
    let largest_x = lst.iter().map(|(i, _j)| i).max().unwrap();
    let largest_y = lst.iter().map(|(_i, j)| j).max().unwrap();

    let mut ps = vec![vec!['.'; *largest_y + 1]; *largest_x + 1];
    for (i, j) in lst {
        ps[i][j] = '#';
    }
    for line in ps {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let dots = Dots::new(buffer);

    let q1_res = q1(&dots);
    println!("{}", q1_res);

    q2(&dots);

    Ok(())
}
