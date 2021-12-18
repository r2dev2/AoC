use std::result::Result;

#[derive(Copy, Clone, Debug)]
struct PairNumber {
    num: i32,
    depth: i32,
}

fn parse_snail_num(line: &str) -> Vec<PairNumber> {
    let mut snail_number = Vec::<PairNumber>::new();
    let mut depth = 0;

    for c in line.chars() {
        match c.to_digit(10) {
            Some(num) => snail_number.push(PairNumber { num: num as i32, depth }),
            None => depth += match c {
                '[' => 1,
                ']' => -1,
                _ => 0
            }
        }
    }

    snail_number
}

fn reduce(num: &Vec<PairNumber>) -> Vec<PairNumber> {
    let mut new_num: Vec<PairNumber> = num.into_iter().map(|x| *x).collect();
    let mut had_gamer_moment = true;

    while had_gamer_moment {
        let old_num = new_num.clone();
        let mut to_add = 0;
        let mut is_exploding = false;
        let mut new_num_1 = Vec::<PairNumber>::new();
        new_num = Vec::<PairNumber>::new();
        had_gamer_moment = false;
        for n in old_num {
            if is_exploding {
                to_add = n.num;
                is_exploding = false;
            } else if !had_gamer_moment && n.depth > 4 {
                let nl = new_num_1.len();
                if nl > 0 {
                    new_num_1[nl - 1].num += n.num;
                }
                new_num_1.push(PairNumber { num: 0, depth: n.depth - 1 });
                is_exploding = true;
                had_gamer_moment = true;
            } else {
                new_num_1.push(PairNumber { num: n.num + to_add, depth: n.depth });
                to_add = 0;
            }
        }

        for n in new_num_1 {
            if !had_gamer_moment && n.num > 9 {
                let left_num = n.num / 2;
                let right_num = n.num - left_num;
                new_num.push(PairNumber { num: left_num, depth: n.depth + 1 });
                new_num.push(PairNumber { num: right_num, depth: n.depth + 1 });
                had_gamer_moment = true;
            } else {
                new_num.push(n.clone());
            }
        }
    }

    new_num
}

fn add(num1: &Vec<PairNumber>, num2: &Vec<PairNumber>) -> Vec<PairNumber> {
    num1
        .into_iter()
        .chain(num2.into_iter())
        .map(|n| PairNumber { num: n.num, depth: n.depth + 1 })
        .collect()
}


fn magnitude(num: &Vec<PairNumber>) -> i32 {
    let mut to_calc: Vec<PairNumber> = num.into_iter().map(|x| x.clone()).collect();
    let mut max_depth = to_calc.clone().into_iter().map(|x| x.depth).max().unwrap();
    while max_depth > 1 {
        let mut new_calc: Vec<PairNumber> = vec![];
        let mut previous: Option<PairNumber> = None;
        for n in to_calc.clone() {
            if n.depth == max_depth {
                previous = match previous {
                    Some(pn) => {
                        new_calc.push(PairNumber { num: pn.num * 3 + n.num * 2, depth: n.depth - 1 });
                        None
                    },
                    None => {
                        Some(n)
                    }
                }
            } else {
                new_calc.push(n.clone());
            }
        }
        if let Some(d) = to_calc.clone().into_iter().map(|x| x.depth + 0).max() {
            max_depth = d;
        }

        to_calc = new_calc;
    }
    to_calc[0].num * 3 + to_calc[1].num * 2
}

fn p1(nums: &Vec<Vec<PairNumber>>) -> Result<i32, &'static str> {
    let res = nums
        .into_iter()
        .map(|x| x.clone())
        .reduce(|num1, num2| {
            let added = add(&num1, &num2);
            let res = reduce(&added);
            res
        });
    match res {
        Some(r) => {
            Ok(magnitude(&r))
        },
        None => Err("Bru")
    }
}

fn p2(nums: &Vec<Vec<PairNumber>>) -> Result<i32, &'static str> {
    let res = nums
        .into_iter()
        .flat_map(|n1| nums.into_iter().map(move |n2| magnitude(&reduce(&add(&n1, &n2)))))
        .max();
    Ok(res.unwrap())
}

fn main() -> Result<(), &'static str> {
    let sample = include_str!("./nums.txt");
    let snail_nums = sample.lines().map(parse_snail_num).collect::<Vec<Vec<PairNumber>>>();
    println!("Part 1: {}", p1(&snail_nums)?);
    println!("Part 2: {}", p2(&snail_nums)?);
    Ok(())
}
