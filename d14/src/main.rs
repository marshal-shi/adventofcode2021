use anyhow;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

fn string_to_vec(input: String) -> (Vec<char>, HashMap<(char, char), char>) {
    let lines = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.into())
        .collect::<Vec<String>>();
    let lst = lines[0].clone();
    let map = lines[1..]
        .iter()
        .map(|line| {
            let strs = line.split(" -> ").collect::<Vec<&str>>();
            (
                (
                    strs[0].chars().nth(0).unwrap(),
                    strs[0].chars().nth(1).unwrap(),
                ),
                strs[1].chars().nth(0).unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();
    (lst.chars().collect(), map)
}

fn expand(
    mut lst: Vec<char>,
    map: &HashMap<(char, char), char>,
    steps: usize,
) -> Vec<char> {
    for _s in 0..steps {
        let windows = lst.windows(2);
        let inserts = windows
            .map(|w| map.get(&(w[0], w[1])).unwrap_or(&'-'))
            .collect::<Vec<&char>>();
        let mut new_chars = vec![];
        for i in 0..inserts.len() {
            new_chars.push(lst[i]);
            new_chars.push(*inserts[i]);
        }
        new_chars.push(lst[lst.len() - 1]);
        lst = new_chars;
    }
    lst
}

fn counting(lst: &Vec<char>) -> HashMap<char, usize> {
    let mut count = HashMap::new();
    for c in lst {
        let n = count.entry(*c).or_insert(0);
        *n += 1;
    }
    count
}

fn q1(lst: Vec<char>, map: &HashMap<(char, char), char>) -> usize {
    let lst = expand(lst, map, 10);
    let count = counting(&lst);
    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();
    max - min
}

fn q2(lst: Vec<char>, map: &HashMap<(char, char), char>) -> usize {
    // After checking the list of transfer, we noticed all the order
    // of 2 char are listed in transfers.
    let mut cache = HashMap::new();
    for (k, _v) in map {
        let nlst = vec![k.0, k.1];
        let nlst = expand(nlst, map, 20);
        let count = counting(&nlst);
        cache.insert(k, (count, nlst));
    }
    // 1st 20
    let mut nlst = Vec::new();
    for lt in lst.windows(2) {
        let key = (lt[0], lt[1]);
        let (_, clst) = cache.get(&key).unwrap();
        if nlst.len() == 0 {
            nlst = clst.to_vec();
        } else {
            nlst.extend(clst[1..].to_vec());
        }
    }

    // 2nd 20
    let mut count = HashMap::new();
    for (idx, lt) in nlst.windows(2).enumerate() {
        let key = (lt[0], lt[1]);
        let (cc, clst) = cache.get(&key).unwrap();
        let clst_first = clst[0];
        for (k, v) in cc {
            let n = count.entry(*k).or_insert(0);
            *n += v;
            if clst_first == *k && idx > 0 {
                *n -= 1;
            }
        }
    }

    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();
    max - min
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let (lst, map) = string_to_vec(buffer);

    let q1_res = q1(lst.clone(), &map);
    println!("{}", q1_res);

    let q2_res = q2(lst, &map);
    println!("{}", q2_res);

    Ok(())
}
