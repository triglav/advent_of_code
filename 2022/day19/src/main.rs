use std::{collections::HashMap, io, str::FromStr};

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_costs: u32,
    clay_robot_ore_costs: u32,
    obsidian_robot_ore_costs: u32,
    obsidian_robot_clay_costs: u32,
    geode_robot_ore_costs: u32,
    geode_robot_obsidian_costs: u32,
}

fn parse_number<T>(s: &str) -> Result<T, <T as FromStr>::Err>
where
    T: FromStr,
{
    s.trim_matches(|c: char| !c.is_digit(10)).parse::<T>()
}

fn parse_blueprint(s: &str) -> Blueprint {
    let t: Vec<_> = s.split(':').flat_map(|s| s.split('.')).collect();

    let id = parse_number::<u32>(t.get(0).unwrap()).unwrap();
    let ore_robot_ore_costs = parse_number::<u32>(t.get(1).unwrap()).unwrap();
    let clay_robot_ore_costs = parse_number::<u32>(t.get(2).unwrap()).unwrap();

    let t2: Vec<_> = t.get(3).unwrap().split("and").collect();
    let obsidian_robot_ore_costs = parse_number::<u32>(t2.get(0).unwrap()).unwrap();
    let obsidian_robot_clay_costs = parse_number::<u32>(t2.get(1).unwrap()).unwrap();

    let t3: Vec<_> = t.get(4).unwrap().split("and").collect();
    let geode_robot_ore_costs = parse_number::<u32>(t3.get(0).unwrap()).unwrap();
    let geode_robot_obsidian_costs = parse_number::<u32>(t3.get(1).unwrap()).unwrap();

    Blueprint {
        id,
        ore_robot_ore_costs,
        clay_robot_ore_costs,
        obsidian_robot_ore_costs,
        obsidian_robot_clay_costs,
        geode_robot_ore_costs,
        geode_robot_obsidian_costs,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum BuildTask {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum BuildStatus {
    NotEnoughResources,
    NotPossibleToBuild,
    CanBuild,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct System {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    time: u32,

    task: Option<BuildTask>,
    history: Vec<(u32, u32)>,
}

impl System {
    fn new(time: u32) -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            time,
            task: None,
            history: vec![],
        }
    }

    fn tick(&mut self) -> bool {
        if self.time <= 0 {
            return false;
        }

        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        self.time -= 1;
        self.time > 0
    }

    fn can_build_ore_robot(&mut self, b: &Blueprint) -> BuildStatus {
        if self.ore >= b.ore_robot_ore_costs {
            BuildStatus::CanBuild
        } else if self.ore_robots == 0 {
            BuildStatus::NotPossibleToBuild
        } else {
            BuildStatus::NotEnoughResources
        }
    }

    fn build_ore_robot(&mut self, b: &Blueprint) {
        assert_eq!(self.can_build_ore_robot(b), BuildStatus::CanBuild);
        self.ore_robots += 1;
        self.ore -= b.ore_robot_ore_costs;
        self.history.push((self.time, 0));
    }

    fn can_build_clay_robot(&mut self, b: &Blueprint) -> BuildStatus {
        if self.ore >= b.clay_robot_ore_costs {
            BuildStatus::CanBuild
        } else if self.ore_robots == 0 {
            BuildStatus::NotPossibleToBuild
        } else {
            BuildStatus::NotEnoughResources
        }
    }

    fn build_clay_robot(&mut self, b: &Blueprint) {
        assert_eq!(self.can_build_clay_robot(b), BuildStatus::CanBuild);
        self.clay_robots += 1;
        self.ore -= b.clay_robot_ore_costs;
        self.history.push((self.time, 1));
    }

    fn can_build_obsidian_robot(&mut self, b: &Blueprint) -> BuildStatus {
        if self.ore >= b.obsidian_robot_ore_costs && self.clay >= b.obsidian_robot_clay_costs {
            BuildStatus::CanBuild
        } else if self.ore_robots == 0 || self.clay_robots == 0 {
            BuildStatus::NotPossibleToBuild
        } else {
            BuildStatus::NotEnoughResources
        }
    }

    fn build_obsidian_robot(&mut self, b: &Blueprint) {
        assert_eq!(self.can_build_obsidian_robot(b), BuildStatus::CanBuild);
        self.obsidian_robots += 1;
        self.ore -= b.obsidian_robot_ore_costs;
        self.clay -= b.obsidian_robot_clay_costs;
        self.history.push((self.time, 2));
    }

    fn can_build_geode_robot(&mut self, b: &Blueprint) -> BuildStatus {
        if self.ore >= b.geode_robot_ore_costs && self.obsidian >= b.geode_robot_obsidian_costs {
            BuildStatus::CanBuild
        } else if self.ore_robots == 0 || self.obsidian_robots == 0 {
            BuildStatus::NotPossibleToBuild
        } else {
            BuildStatus::NotEnoughResources
        }
    }

    fn build_geode_robot(&mut self, b: &Blueprint) {
        assert_eq!(self.can_build_geode_robot(b), BuildStatus::CanBuild);
        self.geode_robots += 1;
        self.ore -= b.geode_robot_ore_costs;
        self.obsidian -= b.geode_robot_obsidian_costs;
        self.history.push((self.time, 2));
    }

    fn can_build(&mut self, b: &Blueprint) -> BuildStatus {
        match self.task {
            Some(BuildTask::OreRobot) => self.can_build_ore_robot(b),
            Some(BuildTask::ClayRobot) => self.can_build_clay_robot(b),
            Some(BuildTask::ObsidianRobot) => self.can_build_obsidian_robot(b),
            Some(BuildTask::GeodeRobot) => self.can_build_geode_robot(b),
            None => panic!("Missing task"),
        }
    }

    fn build(&mut self, b: &Blueprint) {
        match self.task {
            Some(BuildTask::OreRobot) => self.build_ore_robot(b),
            Some(BuildTask::ClayRobot) => self.build_clay_robot(b),
            Some(BuildTask::ObsidianRobot) => self.build_obsidian_robot(b),
            Some(BuildTask::GeodeRobot) => self.build_geode_robot(b),
            None => panic!("Missing task"),
        };
        self.task = None;
    }
}

fn simulate(start: System, blueprint: &Blueprint) -> u32 {
    let mut todo = vec![start];
    let mut best = HashMap::<u32, u32>::new();
    let mut max_geodes = 0;
    while let Some(mut s) = todo.pop() {
        if s.task == None {
            for new_task in [
                BuildTask::GeodeRobot,
                BuildTask::ObsidianRobot,
                BuildTask::ClayRobot,
                BuildTask::OreRobot,
            ] {
                let mut s2 = s.clone();
                s2.task = Some(new_task);
                todo.push(s2);
            }
            continue;
        }

        if let Some(t) = best.get(&(s.geode_robots + 1)) {
            if s.time < *t {
                continue;
            }
        }
        match s.can_build(blueprint) {
            BuildStatus::CanBuild => {
                if s.tick() {
                    s.build(blueprint);
                    if s.geode_robots > 0 {
                        if let Some(t) = best.get(&s.geode_robots) {
                            if s.time > *t {
                                best.insert(s.geode_robots, s.time);
                            }
                        } else {
                            best.insert(s.geode_robots, s.time);
                        }
                    }
                    todo.push(s);
                    continue;
                }
            }
            BuildStatus::NotEnoughResources => {
                if s.tick() {
                    todo.push(s);
                    continue;
                }
            }
            BuildStatus::NotPossibleToBuild => continue,
        }
        if s.geode > max_geodes {
            max_geodes = s.geode;
        }
    }
    max_geodes
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap());
    let blueprints: Vec<_> = lines.map(|l| parse_blueprint(l.as_str())).collect();

    let r1: u32 = blueprints
        .iter()
        .map(|b| {
            let s = System::new(24);
            let geodes = simulate(s, &b);
            b.id * geodes
        })
        .sum();
    println!("{:?}", r1);

    let r2: u32 = blueprints
        .iter()
        .take(3)
        .map(|b| {
            let s = System::new(32);
            let time = simulate(s, &b);
            time
        })
        .product();
    println!("{:?}", r2);
}
