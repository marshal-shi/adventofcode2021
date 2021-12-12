use anyhow;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cave {
    name: String,
    small: bool,
}

impl Cave {
    fn new(name: &str) -> Self {
        let small = name.chars().next().unwrap().is_ascii_lowercase();
        Cave {
            name: name.into(),
            small,
        }
    }
}

fn string_to_hm(input: String) -> HashMap<Cave, Vec<Cave>> {
    let mut res: HashMap<Cave, Vec<Cave>> = HashMap::new();
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let cs = line
                .split('-')
                .filter(|c| c.len() > 0)
                .collect::<Vec<&str>>();

            let kc = Cave::new(cs[0]);
            let vc = Cave::new(cs[1]);
            if kc.name != "end" {
                if res.contains_key(&kc) {
                    res.get_mut(&kc).unwrap().push(vc.clone());
                } else {
                    res.insert(kc.clone(), vec![vc.clone()]);
                }
            }

            if vc.name != "end" {
                if res.contains_key(&vc) {
                    res.get_mut(&vc).unwrap().push(kc);
                } else {
                    res.insert(vc, vec![kc]);
                }
            }
        });
    res
}

fn path_to_end(
    hm: &HashMap<Cave, Vec<Cave>>,
    c: Cave,
    cps: Vec<Vec<Cave>>,
    allow_two_small_cave: bool,
) -> Vec<Vec<Cave>> {
    // `Clone` looks ugly. Optimize it later
    match hm.get(&c) {
        None => cps,
        Some(lst) => {
            let mut res = vec![];
            for nc in lst {
                let mut ncps = cps.clone();
                let mut has_update = false;
                let mut atsc = allow_two_small_cave;
                for nps in ncps.iter_mut() {
                    if nc.small && nps.contains(&nc) {
                        if !allow_two_small_cave
                            || nc.name == "start"
                            || nc.name == "end"
                        {
                            continue;
                        } else {
                            atsc = false;
                        }
                    }
                    has_update = true;
                    (*nps).push(nc.clone());
                }
                if !has_update {
                    continue;
                }
                let nc_res = path_to_end(hm, nc.clone(), ncps, atsc);

                res.extend(nc_res);
            }
            res
        }
    }
}

fn q1(hm: &HashMap<Cave, Vec<Cave>>) -> usize {
    let c = Cave::new("start");
    let res = path_to_end(hm, c.clone(), vec![vec![c]], false);
    res.len()
}

fn q2(hm: &HashMap<Cave, Vec<Cave>>) -> usize {
    let c = Cave::new("start");
    let res = path_to_end(hm, c.clone(), vec![vec![c]], true);
    res.len()
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer)?;
    let hm = string_to_hm(buffer);

    let q1_res = q1(&hm);
    println!("{}", q1_res);

    let q2_res = q2(&hm);
    println!("{}", q2_res);

    Ok(())
}
