use std::cmp::{max, min};
use std::collections::HashSet;
use std::result::Result;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Region {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    priority: u32,
}

impl Region {
    fn from_line(line: &str, priority: u32) -> Result<Region, &'static str> {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("Did not find only 2 parts");
        }

        let on = parts[0] == "on";
        let nums = line
            .split(|c: char| c != '-' && !c.is_numeric())
            .flat_map(|s| s.parse::<i32>())
            .collect::<Vec<i32>>();
        
        if nums.len() != 6 {
            return Err("Did not find only 6 numbers");
        }

        Ok(Region {
            on,
            x: (nums[0], nums[1]),
            y: (nums[2], nums[3]),
            z: (nums[4], nums[5]),
            priority,
        })
    }

    fn clamp(&self) -> Self {
        Region {
            x: (self.x.0.clamp(-50, 50), self.x.1.clamp(-50, 50)),
            y: (self.y.0.clamp(-50, 50), self.y.1.clamp(-50, 50)),
            z: (self.z.0.clamp(-50, 50), self.z.1.clamp(-50, 50)),
            on: self.on,
            priority: self.priority
        }
    }

    fn magnitude(&self) -> usize {
        ((self.x.1 - self.x.0 + 1) as usize)
            * ((self.y.1 - self.y.0 + 1) as usize)
            * ((self.z.1 - self.z.0 + 1) as usize)
        // ((self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)) as usize
    }

    fn intersects_with(&self, other: &Self) -> bool {
        one_is_in(self.x, other.x) && one_is_in(self.y, other.y) && one_is_in(self.z, other.z)
    }

    fn is_inside_of(&self, other: &Self) -> bool {
        other.x.0 <= self.x.0 && self.x.1 <= other.x.1
            && other.y.0 <= self.y.0 && self.y.1 <= other.y.1
            && other.z.0 <= self.z.0 && self.z.1 <= other.z.1
    }

    // other's state overrides self's state
    fn merge_with(&self, other: &Self) -> Vec<Self> {
        if self.priority > other.priority {
            return other.merge_with(self);
        }
        if self.is_inside_of(other) {
            return vec![other.clone()];
        }

        let mut res = vec![];

        let mut xs = [self.x.0, self.x.1, other.x.0, other.x.1];
        let mut ys = [self.y.0, self.y.1, other.y.0, other.y.1];
        let mut zs = [self.z.0, self.z.1, other.z.0, other.z.1];
        xs.sort();
        ys.sort();
        zs.sort();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    // let region = Region { on: false, x: (xs[i], xs[i + 1]), y: (ys[j], ys[j + 1]), z: (zs[k], zs[k + 1]) };
                    let region = Region {
                        on: false,
                        x: endpoints(&xs, i),
                        y: endpoints(&ys, j),
                        z: endpoints(&zs, k),
                        priority: 0,
                    };
                    if region.is_inside_of(other) {
                        res.push(Region { on: other.on, priority: other.priority, ..region });
                    } else if region.is_inside_of(self) {
                        res.push(Region { on: self.on, priority: self.priority, ..region });
                    }
                }
            }
        }

        combine_regions(&res)
    }
}

fn combine_regions(regions: &Vec<Region>) -> Vec<Region> {
    let mut res: Vec<Region> = regions.clone();
    let mut had_gamer_moment = true;

    while had_gamer_moment {
        had_gamer_moment = false;
        let mut combined: Vec<Region> = vec![];
        for region in res.clone() {
            let mut added = false;
            for mut possible in &mut combined {
                // let possible_points = possible.get_points();
                // let r_points = region.get_points();
                if !(possible.is_inside_of(&region) || region.is_inside_of(&possible)) || possible.priority != region.priority {
                    continue;
                }
                possible.x.0 = min(region.x.0, possible.x.0);
                possible.y.0 = min(region.y.0, possible.y.0);
                possible.z.0 = min(region.z.0, possible.z.0);
                possible.x.1 = max(region.x.1, possible.x.1);
                possible.y.1 = max(region.y.1, possible.y.1);
                possible.z.1 = max(region.z.1, possible.z.1);
                added = true;
                had_gamer_moment = true;
                break;
            }
            if !added {
                combined.push(region.clone());
            }
        }
        res = combined;
    }

    res
}

#[inline]
fn endpoints(points: &[i32; 4], i: usize) -> (i32, i32) {
    match i {
        0 => (points[i], max(points[i + 1] - 1, points[i])),
        1 => (points[i], points[i + 1]),
        2 => (min(points[i] + 1, points[i + 1]), points[i + 1]),
        _ => (0, 0)
    }
}

#[inline]
fn one_is_in(first: (i32, i32), second: (i32, i32)) -> bool {
    is_in(first.0, second.0, second.1) || is_in(first.1, second.0, second.1) || is_in(second.0, first.0, first.1) || is_in(second.1, first.0, first.1)
}

#[inline]
fn is_in(num_to_test: i32, lower: i32, upper: i32) -> bool {
    lower <= num_to_test && num_to_test <= upper
}

fn get_first_intersecting(instruction_regions: &HashSet<Region>, splits: &HashSet<Region>) -> Option<(Region, Region)> {
    instruction_regions
        .clone()
        .iter()
        .flat_map(|r| splits.iter().map(move |s| (r, s)))
        .find(|(r, s)| r.intersects_with(s))
        .map(|(r, s)| (r.clone(), s.clone()))
}

fn solve(instructions: &Vec<Region>) -> Result<usize, &'static str> {
    let regions = instructions
        .iter()
        .fold(HashSet::<Region>::new(), |previous, region| {
            let mut new_regions = HashSet::<Region>::new();
            let mut instruction_regions = HashSet::<Region>::new();
            instruction_regions.insert(region.clone());

            
            for region in previous {
                let intersecting = instruction_regions
                    .iter()
                    .filter(|r| r.intersects_with(&region))
                    .cloned()
                    .collect::<Vec<Region>>();

                if intersecting.len() == 0 {
                    new_regions.insert(region.clone());
                    continue;
                }


                let mut splits = HashSet::<Region>::new();
                splits.insert(region);

                while let Some((instruction, intersecting_split_of_region)) = get_first_intersecting(&instruction_regions, &splits) {
                    instruction_regions.remove(&instruction);
                    splits.remove(&intersecting_split_of_region);
                    for split in intersecting_split_of_region.merge_with(&instruction) {
                        if split.priority == instruction.priority {
                            instruction_regions.insert(split);
                        }
                        else {
                            splits.insert(split);
                        }
                    }
                }

                for r in splits {
                    if r.on {
                        new_regions.insert(r);
                    }
                }

                instruction_regions = combine_regions(&instruction_regions.iter().cloned().collect()).iter().cloned().collect();
            }

            for region in instruction_regions {
                if region.on {
                    new_regions.insert(region.clone());
                }
            }

            new_regions
        });

    let cubes = regions.iter().map(Region::magnitude).sum();

    Ok(cubes)
}

fn part1(instructions: &Vec<Region>) -> Result<usize, &'static str> {
    let clamped_region = Region {
        on: false,
        priority: 0,
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50)
    };

    solve(&instructions
        .iter()
        .filter(|r| clamped_region.intersects_with(r))
        .map(|r| r.clamp())
        .collect()
    )
}

fn part2(instructions: &Vec<Region>) -> Result<usize, &'static str> {
    solve(instructions)
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./instructions.txt");
    let instructions = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| Region::from_line(line, i as u32))
        .collect::<Vec<Region>>();

    println!("Part 1: {}", part1(&instructions)?);
    println!("Part 2: {}", part2(&instructions)?);

    Ok(())
}
