use anyhow;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Mat {
    oct: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Mat {
    fn new(input: String) -> Self {
        let oct = input
            .split('\n')
            .filter(|line| line.len() > 0)
            .map(|line| {
                line.chars()
                    .filter_map(|n| n.to_digit(10))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<_>>();
        let width = oct[0].len();
        let height = oct.len();
        Mat { oct, width, height }
    }

    fn adjacents(&self, ci: i32, cj: i32) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                let ni = ci + i;
                let nj = cj + j;
                if (ni == ci && nj == cj)
                    || ni < 0
                    || nj < 0
                    || ni as usize >= self.width
                    || nj as usize >= self.height
                {
                    continue;
                }
                res.push((ni, nj));
            }
        }
        res.into_iter()
            .map(|(i, j)| (i as usize, j as usize))
            .collect()
    }

    fn flash(&mut self) {
        let mut updated = Vec::new();
        self.oct.iter_mut().enumerate().for_each(|(i, mut line)| {
            line.iter_mut().enumerate().for_each(|(j, mut n)| {
                let nn = *n + 1;
                if nn > 9 {
                    *n = 0;
                    updated.push((i, j));
                } else {
                    *n = nn;
                }
            })
        });

        loop {
            let mut new_updated = Vec::new();
            updated.into_iter().for_each(|(i, j)| {
                let ads = self.adjacents(i as i32, j as i32);
                ads.into_iter().for_each(|(ai, aj)| {
                    if self.oct[ai][aj] > 0 {
                        self.oct[ai][aj] += 1;
                    }
                    if self.oct[ai][aj] > 9 {
                        self.oct[ai][aj] = 0;
                        new_updated.push((ai, aj));
                    }
                });
            });

            if new_updated.len() == 0 {
                break;
            }
            updated = new_updated;
        }
    }

    fn zero_count(&self) -> usize {
        self.oct
            .iter()
            .map(|line| line.iter().filter(|&&n| n == 0).count())
            .sum()
    }
}

fn q1(mat: &mut Mat, times: usize) -> usize {
    (0..times)
        .map(|_i| {
            mat.flash();
            mat.zero_count()
        })
        .sum()
}

fn q2(mat: &mut Mat) -> usize {
    let mut loop_count = 0;
    loop {
        mat.flash();
        loop_count += 1;
        if mat.zero_count() == mat.width * mat.height {
            return loop_count;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;

    let q1_res = q1(&mut Mat::new(buffer.clone()), 100);
    println!("{}", q1_res);

    let q2_res = q2(&mut Mat::new(buffer.clone()));
    println!("{}", q2_res);

    Ok(())
}
