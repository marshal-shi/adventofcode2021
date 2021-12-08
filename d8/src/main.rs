use anyhow;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

fn contain_str(s1: &str, s2: &str, exact: bool) -> bool {
    // whether s1 contains s2
    let set: HashSet<char> = s1.chars().collect();
    let contain = s2.chars().all(|c| set.contains(&c));
    if exact {
        contain && s1.len() == s2.len()
    } else {
        contain
    }
}

#[derive(Debug)]
struct Part(Vec<String>, Vec<String>);

impl Part {
    fn understand(&self) -> HashMap<String, i32> {
        let mut s: HashMap<String, i32> =
            self.0.iter().map(|w| (w.into(), -1)).collect();
        // rev hashmap to simple the checking
        let mut r: HashMap<i32, String> =
            (0..=9).map(|i| (i, "".into())).collect();
        // Find 1, 4, 7, 8
        self.0.iter().for_each(|w| {
            let x = s.get_mut(w).unwrap();
            let wstring = w.to_string();
            match w.len() {
                2 => {
                    *x = 1;
                    *r.get_mut(&1).unwrap() = wstring;
                }
                3 => {
                    *x = 7;
                    *r.get_mut(&7).unwrap() = wstring;
                }
                4 => {
                    *x = 4;
                    *r.get_mut(&4).unwrap() = wstring;
                }
                7 => {
                    *x = 8;
                    *r.get_mut(&8).unwrap() = wstring;
                }
                _ => {}
            }
        });

        // find 0, 6, 9
        // 9 contains 4
        // 0 contains 1 but not contain 4
        // 6 not contains 1 and 4
        self.0.iter().for_each(|w| {
            if w.len() == 6 {
                let wstring = w.to_string();
                match (
                    contain_str(w, r.get(&4).unwrap(), false),
                    contain_str(w, r.get(&1).unwrap(), false),
                ) {
                    (true, _) => {
                        *s.get_mut(w).unwrap() = 9;
                        *r.get_mut(&9).unwrap() = wstring;
                    }
                    (false, true) => {
                        *s.get_mut(w).unwrap() = 0;
                        *r.get_mut(&0).unwrap() = wstring;
                    }
                    (false, false) => {
                        *s.get_mut(w).unwrap() = 6;
                        *r.get_mut(&6).unwrap() = wstring;
                    }
                }
            }
        });

        // 2, 3 and 5
        // 5 is part of 9, but not 2
        // 3 contains 1
        self.0.iter().for_each(|w| {
            if w.len() == 5 {
                let wstring = w.to_string();
                match (
                    contain_str(r.get(&9).unwrap(), w, false),
                    contain_str(w, r.get(&1).unwrap(), false),
                ) {
                    (_, true) => {
                        *s.get_mut(w).unwrap() = 3;
                        *r.get_mut(&3).unwrap() = wstring;
                    }
                    (true, _) => {
                        *s.get_mut(w).unwrap() = 5;
                        *r.get_mut(&5).unwrap() = wstring;
                    }
                    (false, _) => {
                        *s.get_mut(w).unwrap() = 2;
                        *r.get_mut(&2).unwrap() = wstring;
                    }
                }
            }
        });

        s
    }
}

fn string_to_vec(input: String) -> Vec<Part> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let parts = line
                .split(" | ")
                .filter(|part| part.len() > 0)
                .map(|part| {
                    part.split(' ')
                        .filter(|word| word.len() > 0)
                        .map(|word| word.into())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<_>>();

            Part(parts[0].to_owned(), parts[1].to_owned())
        })
        .collect()
}

fn q1(vec: &Vec<Part>) -> usize {
    vec.iter()
        .map(|part| {
            part.1
                .iter()
                .filter(|word| [2, 3, 4, 7].contains(&word.len()))
                .count()
        })
        .sum()
}

fn q2(vec: &Vec<Part>) -> i32 {
    vec.iter()
        .map(|part| {
            let nums = part.understand();
            let num_string = part
                .1
                .iter()
                .map(|word| {
                    let mut iter = nums.iter();
                    let word_num = loop {
                        let (key, v) = iter.next().unwrap();
                        if contain_str(key, word, true) {
                            break v;
                        }
                    };
                    word_num.to_string()
                })
                .collect::<String>();
            num_string.parse::<i32>().unwrap()
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let vec = string_to_vec(buffer);
    //println!("{:?}", vec);
    let q1_res = q1(&vec);
    println!("{}", q1_res);

    let q2_res = q2(&vec);
    println!("{}", q2_res);

    Ok(())
}
