use std::cmp;
use std::result::Result;

#[derive(Debug)]
struct Target {
    x0: i32,
    xf: i32,
    y0: i32,
    yf: i32,
}

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Target {
    fn from_line(input: &str) -> Result<Target, &'static str> {
        let nums: Vec<i32> = input
            .split(|c: char| c != '-' && !c.is_numeric())
            .map(|s| s.parse::<i32>())
            .flat_map(|x| x)
            .collect();
        if nums.len() != 4 {
            Err("A different amount of numbers than expected were found")
        } else {
            Ok(Target {
                x0: nums[0],
                xf: nums[1],
                y0: nums[2],
                yf: nums[3],
            })
        }
    }
}

fn optimal_vx(vx: i32, target: i32, dist: i32) -> (i32, i32) {
    if dist >= target {
        (vx - 1, dist)
    } else {
        optimal_vx(vx + 1, target, dist + vx)
    }
}

fn hits_target(probe: Probe, target: &Target, highest_y: i32) -> (bool, i32) {
    if probe.vy < 0 && probe.y < target.y0 {
        (false, highest_y)
    } else if target.x0 <= probe.x && probe.x <= target.xf && target.y0 <= probe.y && probe.y <= target.yf {
        (true, highest_y)
    } else {
        hits_target(Probe {
            x: probe.x + probe.vx,
            y: probe.y + probe.vy,
            vx: cmp::max(probe.vx - 1, 0),
            vy: probe.vy - 1
        }, target, cmp::max(probe.y + probe.vy, highest_y))
    }
}

fn p1(target: &Target) -> Result<i32, &'static str> {
    let (vx, _dist) = optimal_vx(0, target.x0, 0);
    let mut highest_y = 0;
    for vy in 0..100 {
        let (hits, hy) = hits_target(Probe { x: 0, y: 0, vx, vy }, &target, 0);
        if hits {
            highest_y = hy;
        }
    }
    Ok(highest_y)
}

fn p2(target: &Target) -> Result<i32, &'static str> {
    let mut unique_velocities = 0;
    for vy in -100..100 {
        for vx in 0..300 {
            if hits_target(Probe { x: 0, y: 0, vx, vy }, &target, 0).0 {
                unique_velocities += 1;
            }
        }
    }
    Ok(unique_velocities)
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./target.txt");
    let target = Target::from_line(input)?;
    println!("{}", p1(&target)?);
    println!("{}", p2(&target)?);
    Ok(())
}
