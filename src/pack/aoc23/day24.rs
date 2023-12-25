use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use itertools::Itertools;
use num::BigInt;
use num::Zero;

use crate::day::Day;
use crate::day::Solveable;

#[derive(Debug, Clone, Hash, Eq)]
struct Vec3D {
    data: (BigInt, BigInt, BigInt),
}

impl Vec3D {
    fn x(&self) -> BigInt {
        self.data.0.clone()
    }
    fn y(&self) -> BigInt {
        self.data.1.clone()
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            data: (
                self.data.0 + rhs.data.0,
                self.data.1 + rhs.data.1,
                self.data.2 + rhs.data.2,
            ),
        }
    }
}
impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            data: (
                self.data.0 - rhs.data.0,
                self.data.1 - rhs.data.1,
                self.data.2 - rhs.data.2,
            ),
        }
    }
}
impl Mul for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            data: (
                self.data.0 * rhs.data.0,
                self.data.1 * rhs.data.1,
                self.data.2 * rhs.data.2,
            ),
        }
    }
}
impl Mul<BigInt> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: BigInt) -> Self::Output {
        Vec3D {
            data: (
                self.data.0 * rhs.clone(),
                self.data.1 * rhs.clone(),
                self.data.2 * rhs,
            ),
        }
    }
}
impl Div for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            data: (
                self.data.0 / rhs.data.0,
                self.data.1 / rhs.data.1,
                self.data.2 / rhs.data.2,
            ),
        }
    }
}
impl Div<BigInt> for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: BigInt) -> Self::Output {
        Vec3D {
            data: (
                self.data.0 / rhs.clone(),
                self.data.1 / rhs.clone(),
                self.data.2 / rhs,
            ),
        }
    }
}

impl PartialEq for Vec3D {
    fn eq(&self, other: &Self) -> bool {
        self.data.0 == other.data.0 && self.data.1 == other.data.1 && self.data.2 == other.data.2
    }
}
impl PartialEq<(BigInt, BigInt, BigInt)> for Vec3D {
    fn eq(&self, other: &(BigInt, BigInt, BigInt)) -> bool {
        self.data.0 == other.0 && self.data.1 == other.1 && self.data.2 == other.2
    }
}
impl Rem<BigInt> for Vec3D {
    type Output = Vec3D;

    fn rem(self, rhs: BigInt) -> Self::Output {
        Vec3D {
            data: (
                self.data.0 % rhs.clone(),
                self.data.1 % rhs.clone(),
                self.data.2 % rhs,
            ),
        }
    }
}

fn parse_snowflake(line: &String) -> (Vec3D, Vec3D) {
    let (pos_str, v_str) = line.split_once(" @ ").unwrap();
    let pos = pos_str
        .split(", ")
        .map(|s| s.trim())
        .map(|s| s.parse::<BigInt>().unwrap())
        .collect_tuple()
        .unwrap();
    let v = v_str
        .split(", ")
        .map(|s| s.trim())
        .map(|s| s.parse::<BigInt>().unwrap())
        .collect_tuple()
        .unwrap();
    (Vec3D { data: pos }, Vec3D { data: v })
}

fn get_intersection_point_2d(p1: Vec3D, p2: Vec3D, v1: Vec3D, v2: Vec3D) -> Option<(Vec3D, Vec3D)> {
    // using cramer's rule to solve the equation in the form of
    // |v1.x, -v2.x| * |t1| = |p2.x - p1.x|
    // |v1.y, -v2.y|   |t2|   |p2.y - p1.y|
    // source: https://math.stackexchange.com/questions/406864
    // first check determinant
    let det_a = v1.x() * -v2.y() - v1.y() * -v2.x();
    if det_a == BigInt::zero() {
        return None;
    }
    let det_a1 = (p2.x() - p1.x()) * -v2.y() - (p2.y() - p1.y()) * -v2.x();
    let det_a2 = v1.x() * (p2.y() - p1.y()) - v1.y() * (p2.x() - p1.x());
    let t1 = det_a1.clone() / det_a.clone();
    let t2 = det_a2.clone() / det_a.clone();
    // The task specifies the intersections must be in the future
    if t1 < 0.into() || t2 < 0.into() {
        return None;
    }
    // I pulled the t1 into the equation to avoid integer divison weirdness
    let numerator = p1 * det_a.clone() + v1 * det_a1;
    let denominator = det_a;
    Some((
        numerator.clone() / denominator.clone(),
        numerator % denominator,
    ))
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let snowflakes = lines.iter().map(|s| parse_snowflake(s)).collect_vec();

        snowflakes
            .into_iter()
            .combinations(2)
            .filter(|v| {
                let (pos1, vel1) = v[0].clone();
                let (pos2, vel2) = v[1].clone();
                let int_opt = get_intersection_point_2d(pos1, pos2, vel1, vel2);
                if int_opt.is_none() {
                    return false;
                }
                let (intersect_pos, intersect_modulo) = int_opt.unwrap();
                // let seven = BigInt::from(7);
                let seven = BigInt::from(2) * BigInt::from(10).pow(14);
                // let twenty_seven = BigInt::from(27);
                let twenty_seven = BigInt::from(4) * BigInt::from(10).pow(14);
                if intersect_pos.x() >= seven
                    && intersect_pos.x() <= twenty_seven
                    && intersect_pos.y() >= seven
                    && intersect_pos.y() <= twenty_seven
                    && !(intersect_pos.x() == twenty_seven && intersect_modulo.x() > BigInt::zero())
                    && !(intersect_pos.y() == twenty_seven && intersect_modulo.y() > BigInt::zero())
                {
                    return true;
                }
                false
            })
            .count()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        // We need to find pos, vel s.t.
        // FORALL i=0..snowflakes.len() EXISTS t s.t. pos + vel * t = pos_i + vel_i * t (<=> t = (pos_i - pos) / (vel - vel_i))
        // pos.x + vel.x * t_i = pos_i.x + vel_i.x * t_i
        // pos.y + vel.y * t_i = pos_i.y + vel_i.y * t_i
        // pos.z + vel.z + t_i = pos_i.z + vel_i.z * t_i
        // also all t_i must be non-negtive
        // So we have 3 * i equations and (6 + i) unknowns (pos, vel, t_i)
        // We can solve this by using the first 6 equations to solve for pos, vel
        // I gave up and solved part 2 in Sympy, TODO write something cool here in rust
        "TODO".to_string()
    }
}

get_day_fn!(Part1, Part2);
