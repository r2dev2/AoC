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

fn _can_hit(c: i32, vc: i32, c0: i32, cf: i32, no_hope: fn (i32, i32, i32, i32) -> bool, update_v: fn (i32) -> i32) -> bool {
    if c0 <= c && c <= cf {
        true
    } else if no_hope(c, vc, c0, cf) {
        false
    } else {
        _can_hit(c + vc, update_v(vc), c0, cf, no_hope, update_v)
    }

}

fn can_hit_x(vx: i32, target: &Target) -> bool {
    _can_hit(0, vx, target.x0, target.xf, |x, v, x0, xf| x > xf || x < x0 && v <= 0, |v| cmp::max(0, v - 1))
}

fn can_hit_y(vy: i32, target: &Target) -> bool {
    _can_hit(0, vy, target.y0, target.yf, |y, _v, y0, _yf| y < y0, |v| v - 1)
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
    let vx = optimal_vx(0, target.x0, 0).0;
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
    let possible_vy = (-100..100)
        .filter(|vy| can_hit_y(*vy, &target))
        .collect::<Vec<i32>>();
    let unique_velocities = (0..300)
        .filter(|vx| can_hit_x(*vx, &target))
        .map(|vx| possible_vy.iter().map(|vy| (vx, *vy)).collect::<Vec<(i32, i32)>>())
        .flatten()
        .filter(|vs| hits_target(Probe { x: 0, y: 0, vx: vs.0, vy: vs.1 }, &target, 0).0)
        .count();
    Ok(unique_velocities as i32)
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./target.txt");
    let target = Target::from_line(input)?;
    println!("{}", p1(&target)?); // should be 3655
    println!("{}", p2(&target)?); // should be 1447
    Ok(())
}
