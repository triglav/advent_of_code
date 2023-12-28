use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    i128, io,
};

use itertools::Itertools;

#[derive(Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn get_i128(&self, idx: usize) -> i128 {
        match idx {
            0 => self.x as i128,
            1 => self.y as i128,
            2 => self.z as i128,
            _ => panic!("Invalid index"),
        }
    }
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    id: usize,
    pos: Vec3,
    vel: Vec3,
}

impl PartialEq for Hailstone {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Hailstone {}

impl std::hash::Hash for Hailstone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

fn parse(idx: usize, line: &str) -> Hailstone {
    let t = line.split('@').collect::<Vec<_>>();
    let pos = t[0]
        .split(',')
        .map(|s| s.trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let vel = t[1]
        .split(',')
        .map(|s| s.trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    Hailstone {
        id: idx,
        pos: Vec3::new(pos[0], pos[1], pos[2]),
        vel: Vec3::new(vel[0], vel[1], vel[2]),
    }
}

fn segment_intersection(a: (Vec3, Vec3), b: (Vec3, Vec3)) -> Option<Vec3> {
    let (p1, p2) = a;
    let (q1, q2) = b;

    let a = dot(p2 - p1, q1 - p1) / (p2 - p1).norm2();
    let b = dot(p2 - p1, q2 - q1) / (p2 - p1).norm2();
    let c = (p2 - p1) * b - (q2 - q1);

    let t1 = dot(c, q1 - p1 * (1.0 - a) - p2 * a) / c.norm2();
    let t0 = a + t1 * b;

    if (0.0..=1.0).contains(&t0) && (0.0..=1.0).contains(&t1) {
        let i = p1 + (p2 - p1) * t0;
        Some(i)
    } else {
        None
    }
}

fn get_trajectory_in_boundaries_2d(
    h: &Hailstone,
    boundaries: (f64, f64),
) -> Option<(Hailstone, (Vec3, Vec3))> {
    let mut r = vec![];

    let x0 = h.pos.x;
    let y0 = h.pos.y;
    let vx = h.vel.x;
    let vy = h.vel.y;

    let x1 = boundaries.0;
    let t1 = (x1 - x0) / vx;
    let y1 = y0 + t1 * vy;
    let v1 = Vec3::new(x1, y1, 0.0);
    let is_left = (boundaries.0..=boundaries.1).contains(&y1) && t1 > 0.0;
    if is_left {
        r.push((t1, v1));
    }

    let x2 = boundaries.1;
    let t2 = (x2 - x0) / vx;
    let y2 = y0 + t2 * vy;
    let v2 = Vec3::new(x2, y2, 0.0);
    let is_right = (boundaries.0..=boundaries.1).contains(&y2) && t2 > 0.0;
    if is_right {
        r.push((t2, v2));
    }

    let y3 = boundaries.0;
    let t3 = (y3 - y0) / vy;
    let x3 = x0 + t3 * vx;
    let v3 = Vec3::new(x3, y3, 0.0);
    let is_top = (boundaries.0..=boundaries.1).contains(&x3) && t3 > 0.0;
    if is_top {
        r.push((t3, v3));
    }

    let y4 = boundaries.1;
    let t4 = (y4 - y0) / vy;
    let x4 = x0 + t4 * vx;
    let v4 = Vec3::new(x4, y4, 0.0);
    let is_bottom = (boundaries.0..=boundaries.1).contains(&x4) && t4 > 0.0;
    if is_bottom {
        r.push((t4, v4));
    }
    if r.is_empty() {
        return None;
    }
    if r.len() == 1 {
        r.push((0.0, h.pos));
    }
    assert_eq!(
        r.len(),
        2,
        "Expected 2 trajectory points, found {}",
        r.len()
    );
    r.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Some((*h, (r[0].1, r[1].1)))
}

fn find_most_used_velocities(velocities: &[i128]) -> Vec<(&i128, i32)> {
    velocities
        .iter()
        .fold(HashMap::new(), |mut acc, v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|(_, c)| *c > 2)
        .collect_vec()
}

fn find_suitable_rock_velocities(
    hailstones: &[Hailstone],
    idx: usize,
    m_vel: i128,
) -> HashSet<i128> {
    let hailstones_with_m_vel = hailstones
        .iter()
        .filter(|h| h.vel.get_i128(idx) == m_vel)
        .collect_vec();
    let velocities = (-1000..=1000)
        .filter(|&rock_vel| {
            hailstones_with_m_vel.iter().combinations(2).all(|c| {
                let a = c[0];
                let b = c[1];

                let v = (rock_vel - a.vel.get_i128(idx)).abs();
                if v == 0 {
                    return false;
                }
                (a.pos.get_i128(idx) - b.pos.get_i128(idx)).abs() % v == 0
            })
        })
        .collect::<HashSet<_>>();
    velocities
}

fn find_rock_velocity(hailstones: &[Hailstone], idx: usize) -> i128 {
    let vel = hailstones.iter().map(|h| h.vel.get_i128(idx)).collect_vec();
    let m_vel = find_most_used_velocities(&vel);

    let suitable_veloctities = m_vel
        .iter()
        .map(|v| find_suitable_rock_velocities(hailstones, idx, *v.0))
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap();
    assert_eq!(suitable_veloctities.len(), 1);
    *suitable_veloctities.iter().next().unwrap()
}

fn main() {
    let hailstones = io::stdin()
        .lines()
        .enumerate()
        .map(|(i, l)| parse(i, l.unwrap().as_str()))
        .collect::<Vec<_>>();

    let boundaries_min = 200000000000000.0;
    let boundaries_max = 400000000000000.0;

    let hailstones_2d = hailstones.iter().map(|h| Hailstone {
        id: h.id,
        pos: Vec3::new(h.pos.x, h.pos.y, 0.0),
        vel: Vec3::new(h.vel.x, h.vel.y, 0.0),
    });
    let intersections_2d = hailstones_2d
        .filter_map(|h| get_trajectory_in_boundaries_2d(&h, (boundaries_min, boundaries_max)))
        .combinations(2)
        .filter_map(|c| segment_intersection(c[0].1, c[1].1));
    let r1 = intersections_2d.count();
    println!("{:?}", r1);

    let rock_vel = Vec3 {
        x: find_rock_velocity(&hailstones, 0) as f64,
        y: find_rock_velocity(&hailstones, 1) as f64,
        z: find_rock_velocity(&hailstones, 2) as f64,
    };

    let mut i = hailstones.iter();
    let h1 = i.next().unwrap();
    let h2 = i.next().unwrap();

    let ma = (h1.vel.y - rock_vel.y) / (h1.vel.x - rock_vel.x);
    let mb = (h2.vel.y - rock_vel.y) / (h2.vel.x - rock_vel.x);
    let ca = h1.pos.y - ma * h1.pos.x;
    let cb = h2.pos.y - mb * h2.pos.x;
    let x_pos = ((cb - ca) / (ma - mb)).round();
    let y_pos = (ma * x_pos + ca).round();
    let time = (x_pos - h1.pos.x) / (h1.vel.x - rock_vel.x);
    let z_pos = h1.pos.z + (h1.vel.z - rock_vel.z) * time;

    let r2 = x_pos + y_pos + z_pos;
    println!("{}", r2);
}
