use std::result::Result;

fn expand_img(img: &Vec<Vec<char>>, amount: i32) -> Vec<Vec<char>> {
    let mut new_img = Vec::new();
    let img_len = img[0].len() as i32;

    for _ in 0..amount {
        new_img.push((0..(amount * 2 + img_len)).map(|_i| '.').collect());
    }

    for line in img {
        let mut new_line = (0..amount).map(|_i| '.').collect::<Vec<char>>();
        new_line.extend(line.iter());
        new_line.extend((0..amount).map(|_i| '.'));
        new_img.push(new_line);
    }

    for _ in 0..amount {
        new_img.push((0..(amount * 2 + img_len)).map(|_i| '.').collect());
    }

    new_img
}

fn parse(input: &str) -> Option<(Vec<char>, Vec<Vec<char>>)> {
    let mut lines = input.lines();
    let enhancement_algo = lines.next()?.chars().collect::<Vec<char>>();
    lines.next();
    let img = lines.map(|s| s.trim_end().chars().collect::<Vec<char>>()).collect();
    Some((enhancement_algo, expand_img(&img, 200)))
}

fn solve(input: &str, amount: i32) -> Result<i32, &'static str> {
    if let Some((enhancement_algo, mut img)) = parse(input) {
        for _ in 0..amount {
            let max_i = img.len() - 1;
            let max_j = img[0].len() - 1;
            img = (1..max_i)
                .map(|i| (1..max_j).map(move |j| (i, j))
                    .map(|loc| ((loc.0 - 1)..(loc.0 + 2))
                        .flat_map(move |i| ((loc.1 - 1)..(loc.1 + 2)).map(move |j| (i, j)))
                        .map(|loc| img[loc.0][loc.1])
                        .enumerate()
                        .fold(0, |num, (i, val)| {
                            num + match val {
                                '#' => i32::pow(2, (8 - i) as u32),
                                _ => 0
                            }
                        })
                    )
                    .map(|enhancement_idx| enhancement_algo[enhancement_idx as usize])
                    .collect::<Vec<char>>()
                )
                .collect();
        }

        Ok(img.iter().flatten().filter(|c| **c == '#').map(|_| 1).sum())
    } else {
        Err("Couldn't parse input")
    }
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./img.txt");

    println!("Part 1: {}", solve(&input, 2)?);
    println!("Part 2: {}", solve(&input, 50)?);

    Ok(())
}
