use std::collections::{BTreeSet, HashMap, HashSet};

type Coord = (i32, i32, i32);

fn parse_coord_line(line: &str) -> Option<Coord> {
    let coords: Vec<i32> = line
        .trim_end()
        .split(',')
        .map(|l| l.parse::<i32>())
        .flatten()
        .collect();
    match coords.len() {
        3 => Some((coords[0], coords[1], coords[2])),
        _ => None
    }
}

fn parse_scanners(input: &str) -> Vec<Vec<Coord>> {
    let mut scanners = vec![];
    let mut current_scanner: Vec<Coord> = vec![];
    for line in input.lines() {
        if line.starts_with("---") {
            if current_scanner.len() > 0 {
                scanners.push(current_scanner);
            }
            current_scanner = vec![];
        } else if let Some(coords) = parse_coord_line(&line) {
            current_scanner.push(coords);
        }
    }
    if current_scanner.len() > 0 {
        scanners.push(current_scanner);
    }
    scanners
}

fn transform_coord(c: &Coord, rearrange: i32, neg: i32) -> Coord {
    let a = match rearrange {
        0 => (c.0, c.1, c.2),
        1 => (c.0, c.2, c.1),
        2 => (c.1, c.0, c.2),
        3 => (c.1, c.2, c.0),
        4 => (c.2, c.0, c.1),
        5 => (c.2, c.1, c.0),
        _ => (0, 0, 0),
    };

    match neg {
        0 => (a.0, a.1, a.2),
        1 => (-a.0, -a.1, a.2),
        2 => (a.0, -a.1, -a.2),
        3 => (-a.0, a.1, -a.2),
        4 => (-a.0, a.1, a.2),
        5 => (a.0, -a.1, a.2),
        6 => (a.0, a.1, -a.2),
        7 => (-a.0, -a.1, -a.2),
        _ => (0, 0, 0),
    }
}

fn sub(c1: &Coord, c2: &Coord) -> Coord {
    (c1.0 - c2.0, c1.1 - c2.1, c1.2 - c2.2)
}

fn add(c1: &Coord, c2: &Coord) -> Coord {
    (c1.0 + c2.0, c1.1 + c2.1, c1.2 + c2.2)
}

fn matched_beacons(scanner1: &Vec<Coord>, scanner2: &Vec<Coord>) -> Option<(i32, i32, Vec<(Coord, Coord)>)> {
    for r in 0..6 {
        for n in 0..8 {
            for base_c1 in scanner1 {
                for base_c2 in scanner2 {
                    let c1_delta: HashMap<Coord, Coord> = scanner1
                        .iter()
                        .map(|c| (transform_coord(&sub(c, &base_c1), 0, 0), c.clone()))
                        .collect();
                    let matched: Vec<(Coord, Coord)> = scanner2
                        .iter()
                        .map(|c| (transform_coord(&sub(c, &base_c2), r, n), c.clone()))
                        .filter_map(|cc| {
                            match c1_delta.get(&cc.0) {
                                Some(c1_coord) => Some((*c1_coord, cc.1)),
                                None => None
                            }
                        })
                        .collect();
                    if matched.len() >= 12 {
                        // println!("Used ({}, {})", r, n);
                        return Some((r, n, matched));
                    }
                }
            }
        }
    }
    None
}

fn sol(input: &str) -> Result<(i32, i32), &'static str> {
    let scanners = parse_scanners(&input);
    let mut cont = true;
    let mut to_visit = (0..scanners.len()).collect::<HashSet<_>>();
    let mut detected_beacons: HashSet<Coord> = scanners[0].iter().cloned().collect();
    let mut detected_scanners = HashSet::<Coord>::new();
    // let mut detected_beacons: HashSet<(i32, Coord)> = HashSet::new();// scanners[0].iter().collect();

    // while !to_visit.is_empty() {
    while cont {
        cont = false;
        println!("Need to visit {} scanners", to_visit.len());
        for i in to_visit.clone() {
            let s = &scanners[i];
            let d_v: Vec<Coord> = detected_beacons.iter().cloned().collect();

            if let Some((r, n, matched)) = matched_beacons(&d_v, &s) {
                // for x in matched.iter() {
                //     println!("{:?}\t{:?}", x.0, x.1);
                // }
                // println!("");
                let x = matched[0];
                let diff = sub(&x.0, &transform_coord(&x.1, r, n));
                // println!("Scanner {} at {:?}", i, diff);
                for c in s {
                    let cc = add(&transform_coord(c, r, n), &diff);
                    // print!("Attempting insert of {:?} -> {:?} - ", c, cc);
                    if detected_beacons.insert(cc) {
                        // println!("[SUCCESS]");
                        cont = true;
                    } else {
                        // println!("[DUPLICATE]");
                    }
                }
                detected_scanners.insert(diff);
                to_visit.remove(&i);
            }
        }
    }

    let max_dist = detected_scanners
        .iter()
        .flat_map(|s1| detected_scanners.iter().map(move |s2| sub(s1, s2)))
        .map(|diff| diff.0.abs() + diff.1.abs() + diff.2.abs())
        .max();

    // println!("Detected {} unique beacons", detected_beacons.len());
    // println!("Got {} coords", scanners.iter().flatten().map(|_x| 1).sum::<i32>());

    Ok((detected_beacons.len() as i32, max_dist.unwrap()))
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./scanners.txt");
    println!("{:?}", sol(&input)?);
    Ok(())
}
