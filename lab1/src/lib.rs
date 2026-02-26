use std::cmp::Ordering;
use std::collections::BTreeSet;

pub const MIN_PARAM: i32 = -122;
pub const MAX_PARAM: i32 = 122;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonLineInput {
    pub x0: i32,
    pub y0: i32,
    pub l: i32,
    pub m: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoPointLineInput {
    pub p1: PointI,
    pub p2: PointI,
}

#[derive(Debug)]
pub enum InputError {
    OutOfRange {
        name: &'static str,
        value: i32,
    },
    DegenerateTwoPointLine {
        which: &'static str,
    },
    ZeroDirectionComponent {
        which: &'static str,
        component: &'static str,
    },
}

impl InputError {
    pub fn user_message(&self) -> (&'static str, &'static str) {
        match self {
            InputError::OutOfRange { .. } => (
                "Значення параметра поза допустимими межами.",
                "Введіть ціле число в діапазоні [-122; 122].",
            ),
            InputError::DegenerateTwoPointLine { .. } => (
                "Для прямої задано дві співпадаючі точки, пряму визначити неможливо.",
                "Задайте дві різні точки (x1,y1) та (x2,y2).",
            ),
            InputError::ZeroDirectionComponent { component, .. } => (
                "Некоректний напрямний вектор: заборонено нульові компоненти.",
                match *component {
                    "l" => "Введіть l != 0 (ціле число в діапазоні [-122;122], окрім нуля).",
                    "m" => "Введіть m != 0 (ціле число в діапазоні [-122;122], окрім нуля).",
                    _ => "Виправте дані (l та m не можуть дорівнювати 0).",
                },
            ),
        }
    }
}

pub fn validate_inputs(
    l1: TwoPointLineInput,
    l2: TwoPointLineInput,
    l3: CanonLineInput,
) -> Result<(), InputError> {
    for (name, v) in [
        ("x11", l1.p1.x),
        ("y11", l1.p1.y),
        ("x12", l1.p2.x),
        ("y12", l1.p2.y),
        ("x21", l2.p1.x),
        ("y21", l2.p1.y),
        ("x22", l2.p2.x),
        ("y22", l2.p2.y),
        ("x0", l3.x0),
        ("y0", l3.y0),
        ("l", l3.l),
        ("m", l3.m),
    ] {
        if v < MIN_PARAM || v > MAX_PARAM {
            return Err(InputError::OutOfRange { name, value: v });
        }
    }

    if l1.p1 == l1.p2 {
        return Err(InputError::DegenerateTwoPointLine {
            which: "перша"
        });
    }
    if l2.p1 == l2.p2 {
        return Err(InputError::DegenerateTwoPointLine {
            which: "друга"
        });
    }

    if l3.l == 0 {
        return Err(InputError::ZeroDirectionComponent {
            which: "третя",
            component: "l",
        });
    }
    if l3.m == 0 {
        return Err(InputError::ZeroDirectionComponent {
            which: "третя",
            component: "m",
        });
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rat {
    pub num: i64,
    pub den: i64,
}

impl Rat {
    pub fn new(mut num: i64, mut den: i64) -> Self {
        assert!(den != 0);
        if den < 0 {
            den = -den;
            num = -num;
        }
        let g = gcd_i64(num.abs(), den);
        Rat {
            num: num / g,
            den: den / g,
        }
    }

    pub fn is_int(&self) -> bool {
        self.den == 1
    }
}

impl Ord for Rat {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.num as i128 * other.den as i128;
        let rhs = other.num as i128 * self.den as i128;
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointR {
    pub x: Rat,
    pub y: Rat,
}

impl PointR {
    pub fn fmt(&self) -> String {
        format!("({}, {})", fmt_rat(self.x), fmt_rat(self.y))
    }
}

pub fn fmt_rat(r: Rat) -> String {
    if r.den == 1 {
        format!("{}", r.num)
    } else {
        format!("{}/{}", r.num, r.den)
    }
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs().max(1)
}

#[derive(Debug, Clone, Copy)]
pub struct LineABC {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

impl LineABC {
    pub fn from_two_points(p1: PointI, p2: PointI) -> Self {
        let x1 = p1.x as i64;
        let y1 = p1.y as i64;
        let x2 = p2.x as i64;
        let y2 = p2.y as i64;
        let a = y1 - y2;
        let b = x2 - x1;
        let c = x1 * y2 - x2 * y1;
        LineABC { a, b, c }
    }

    pub fn from_canonical(x0: i32, y0: i32, l: i32, m: i32) -> Self {
        let a = m as i64;
        let b = -(l as i64);
        let c = (l as i64) * (y0 as i64) - (m as i64) * (x0 as i64);
        LineABC { a, b, c }
    }

    pub fn normalized_key(&self) -> (i64, i64, i64) {
        let g = gcd_i64(gcd_i64(self.a.abs(), self.b.abs()), self.c.abs());
        let mut a = self.a / g;
        let mut b = self.b / g;
        let mut c = self.c / g;

        let sign = if a != 0 {
            a.signum()
        } else if b != 0 {
            b.signum()
        } else {
            c.signum()
        };
        if sign < 0 {
            a = -a;
            b = -b;
            c = -c;
        }
        (a, b, c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairRelation {
    ParallelDistinct,
    Coincident,
    Intersect(PointR),
}

pub fn relate(l1: LineABC, l2: LineABC) -> PairRelation {
    let d = l1.a * l2.b - l2.a * l1.b;
    if d == 0 {
        let ac = l1.a * l2.c - l2.a * l1.c;
        let bc = l1.b * l2.c - l2.b * l1.c;
        if ac == 0 && bc == 0 {
            PairRelation::Coincident
        } else {
            PairRelation::ParallelDistinct
        }
    } else {
        let x_num = l1.b * l2.c - l2.b * l1.c;
        let y_num = l2.a * l1.c - l1.a * l2.c;
        let x = Rat::new(x_num, d);
        let y = Rat::new(y_num, d);
        PairRelation::Intersect(PointR { x, y })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Placement {
    AllCoincident,
    NoIntersections,
    OnePoint(PointR),
    TwoPoints(PointR, PointR),
    ThreePoints(PointR, PointR, PointR),
}

pub fn classify(l1: LineABC, l2: LineABC, l3: LineABC) -> Placement {
    let k1 = l1.normalized_key();
    let k2 = l2.normalized_key();
    let k3 = l3.normalized_key();
    if k1 == k2 && k2 == k3 {
        return Placement::AllCoincident;
    }

    let r12 = relate(l1, l2);
    let r13 = relate(l1, l3);
    let r23 = relate(l2, l3);

    let mut points: BTreeSet<PointR> = BTreeSet::new();
    for r in [r12, r13, r23] {
        if let PairRelation::Intersect(p) = r {
            points.insert(p);
        }
    }

    match points.len() {
        0 => Placement::NoIntersections,
        1 => Placement::OnePoint(*points.iter().next().unwrap()),
        2 => {
            let mut it = points.iter();
            let p1 = *it.next().unwrap();
            let p2 = *it.next().unwrap();
            Placement::TwoPoints(p1, p2)
        }
        _ => {
            let mut it = points.iter();
            let p1 = *it.next().unwrap();
            let p2 = *it.next().unwrap();
            let p3 = *it.next().unwrap();
            Placement::ThreePoints(p1, p2, p3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_two_point(x1: i32, y1: i32, x2: i32, y2: i32) -> TwoPointLineInput {
        TwoPointLineInput {
            p1: PointI { x: x1, y: y1 },
            p2: PointI { x: x2, y: y2 },
        }
    }
    fn mk_can(x0: i32, y0: i32, l: i32, m: i32) -> CanonLineInput {
        CanonLineInput { x0, y0, l, m }
    }

    fn build(l1: TwoPointLineInput, l2: TwoPointLineInput, l3: CanonLineInput) -> Placement {
        validate_inputs(l1, l2, l3).unwrap();
        let a1 = LineABC::from_two_points(l1.p1, l1.p2);
        let a2 = LineABC::from_two_points(l2.p1, l2.p2);
        let a3 = LineABC::from_canonical(l3.x0, l3.y0, l3.l, l3.m);
        classify(a1, a2, a3)
    }

    // усі три співпадають
    #[test]
    fn ec_all_coincident_left() {
        let l1 = mk_two_point(MIN_PARAM, MIN_PARAM, MIN_PARAM + 1, MIN_PARAM + 1);
        let l2 = mk_two_point(MIN_PARAM, MIN_PARAM, MIN_PARAM + 1, MIN_PARAM + 1);
        let l3 = mk_can(MIN_PARAM, MIN_PARAM, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::AllCoincident));
    }

    #[test]
    fn ec_all_coincident_right() {
        let l1 = mk_two_point(MAX_PARAM, MAX_PARAM, MAX_PARAM - 1, MAX_PARAM - 1);
        let l2 = mk_two_point(MAX_PARAM, MAX_PARAM, MAX_PARAM - 1, MAX_PARAM - 1);
        let l3 = mk_can(MAX_PARAM, MAX_PARAM, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::AllCoincident));
    }

    #[test]
    fn ec_all_coincident_typical() {
        let l1 = mk_two_point(0, 0, 1, 1);
        let l2 = mk_two_point(2, 2, 3, 3);
        let l3 = mk_can(5, 5, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::AllCoincident));
    }

    #[test]
    fn ec_all_coincident_mix_near_left_mid_right() {
        let l1 = mk_two_point(MIN_PARAM + 1, MIN_PARAM + 1, 0, 0);
        let l2 = mk_two_point(10, 10, 20, 20);
        let l3 = mk_can(MAX_PARAM, MAX_PARAM, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::AllCoincident));
    }

    #[test]
    fn ec_all_coincident_two_near_left_next_and_mid() {
        let l1 = mk_two_point(MIN_PARAM, MIN_PARAM, MIN_PARAM + 2, MIN_PARAM + 2);
        let l2 = mk_two_point(MIN_PARAM + 1, MIN_PARAM + 1, MIN_PARAM + 3, MIN_PARAM + 3);
        let l3 = mk_can(0, 0, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::AllCoincident));
    }

    #[test]
    fn ec_all_coincident_two_near_right_prev_and_mid() {
        let l1 = mk_two_point(MAX_PARAM, MAX_PARAM, MAX_PARAM - 2, MAX_PARAM - 2);
        let l2 = mk_two_point(MAX_PARAM - 1, MAX_PARAM - 1, MAX_PARAM - 3, MAX_PARAM - 3);
        let l3 = mk_can(0, 0, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::AllCoincident));
    }

    // 0 точок перетину (усі три паралельні та різні)
    #[test]
    fn ec_no_intersections_left() {
        let l1 = mk_two_point(MIN_PARAM, MIN_PARAM, MIN_PARAM + 1, MIN_PARAM + 1); // y=x
        let l2 = mk_two_point(MIN_PARAM, MIN_PARAM + 1, MIN_PARAM + 1, MIN_PARAM + 2); // y=x+1
        let l3 = mk_can(MIN_PARAM, MIN_PARAM + 2, 1, 1); // y=x+2
        assert!(matches!(build(l1, l2, l3), Placement::NoIntersections));
    }

    #[test]
    fn ec_no_intersections_right() {
        let l1 = mk_two_point(MAX_PARAM, MAX_PARAM, MAX_PARAM - 1, MAX_PARAM - 1); // y=x
        let l2 = mk_two_point(MAX_PARAM, MAX_PARAM - 1, MAX_PARAM - 1, MAX_PARAM - 2); // y=x-1
        let l3 = mk_can(MAX_PARAM, MAX_PARAM - 2, 1, 1); // y=x-2
        assert!(matches!(build(l1, l2, l3), Placement::NoIntersections));
    }

    #[test]
    fn ec_no_intersections_typical() {
        let l1 = mk_two_point(0, 0, 1, 1); // y=x
        let l2 = mk_two_point(0, 1, 1, 2); // y=x+1
        let l3 = mk_can(0, 2, 1, 1); // y=x+2
        assert!(matches!(build(l1, l2, l3), Placement::NoIntersections));
    }

    #[test]
    fn ec_no_intersections_mix() {
        let l1 = mk_two_point(MIN_PARAM + 1, MIN_PARAM + 1, 0, 0); // y=x
        let l2 = mk_two_point(10, 11, 20, 21); // y=x+1
        let l3 = mk_can(MAX_PARAM, MAX_PARAM - 1, 1, 1); // y=x-1
        assert!(matches!(build(l1, l2, l3), Placement::NoIntersections));
    }

    #[test]
    fn ec_no_intersections_two_near_left_next_and_mid() {
        let l1 = mk_two_point(MIN_PARAM, MIN_PARAM, MIN_PARAM + 2, MIN_PARAM + 2); // y=x
        let l2 = mk_two_point(MIN_PARAM + 1, MIN_PARAM + 2, MIN_PARAM + 3, MIN_PARAM + 4); // y=x+1
        let l3 = mk_can(0, 2, 1, 1); // y=x+2
        assert!(matches!(build(l1, l2, l3), Placement::NoIntersections));
    }

    #[test]
    fn ec_no_intersections_two_near_right_prev_and_mid() {
        let l1 = mk_two_point(MAX_PARAM, MAX_PARAM, MAX_PARAM - 2, MAX_PARAM - 2); // y=x
        let l2 = mk_two_point(MAX_PARAM - 1, MAX_PARAM - 2, MAX_PARAM - 3, MAX_PARAM - 4); // y=x-1
        let l3 = mk_can(0, 1, 1, 1); // y=x+1
        assert!(matches!(build(l1, l2, l3), Placement::NoIntersections));
    }

    // 1 точка перетину
    #[test]
    fn ec_one_point_leftish() {
        let l1 = mk_two_point(-1, -1, 1, 1); // y=x
        let l2 = mk_two_point(-1, 1, 1, -1); // y=-x
        let l3 = mk_can(0, 0, 1, 2);
        match build(l1, l2, l3) {
            Placement::OnePoint(p) => assert_eq!(
                p,
                PointR {
                    x: Rat::new(0, 1),
                    y: Rat::new(0, 1)
                }
            ),
            _ => panic!("expected one point"),
        }
    }

    #[test]
    fn ec_one_point_rightish() {
        let _l1 = mk_two_point(MAX_PARAM - 1, MAX_PARAM - 1, MAX_PARAM, MAX_PARAM);
        let _l2 = mk_two_point(MAX_PARAM - 1, MAX_PARAM, MAX_PARAM, MAX_PARAM - 1);
        let l1 = mk_two_point(10, 10, 11, 11);
        let l2 = mk_two_point(10, 10, 11, 9);
        let l3 = mk_can(10, 10, 1, 2);
        assert!(matches!(build(l1, l2, l3), Placement::OnePoint(_)));
    }

    #[test]
    fn ec_one_point_typical() {
        let l1 = mk_two_point(0, 0, 2, 2);
        let l2 = mk_two_point(0, 0, 2, -2);
        let l3 = mk_can(0, 0, 2, 1);
        assert!(matches!(build(l1, l2, l3), Placement::OnePoint(_)));
    }

    #[test]
    fn ec_one_point_mix() {
        let l1 = mk_two_point(MIN_PARAM + 1, MIN_PARAM + 1, MIN_PARAM + 2, MIN_PARAM + 2);
        let l2 = mk_two_point(MIN_PARAM + 1, MIN_PARAM + 1, MIN_PARAM + 2, MIN_PARAM);
        let l3 = mk_can(MIN_PARAM + 1, MIN_PARAM + 1, 1, 2);
        assert!(matches!(build(l1, l2, l3), Placement::OnePoint(_)));
    }

    #[test]
    fn ec_one_point_two_near_left_next_and_mid() {
        let l1 = mk_two_point(-1, -1, 1, 1);
        let l2 = mk_two_point(-1, -1, 1, -3);
        let l3 = mk_can(-1, -1, 1, 3);
        assert!(matches!(build(l1, l2, l3), Placement::OnePoint(_)));
    }

    #[test]
    fn ec_one_point_two_near_right_prev_and_mid() {
        let l1 = mk_two_point(1, 1, 2, 2);
        let l2 = mk_two_point(1, 1, 2, 0);
        let l3 = mk_can(1, 1, 2, 3);
        assert!(matches!(build(l1, l2, l3), Placement::OnePoint(_)));
    }

    // 2 точки перетину
    #[test]
    fn ec_two_points_typical() {
        let l1 = mk_two_point(0, 0, 1, 1); // y=x
        let l2 = mk_two_point(0, 1, 1, 2); // y=x+1
        let l3 = mk_can(0, 0, 1, -1);
        match build(l1, l2, l3) {
            Placement::TwoPoints(p1, p2) => {
                let a = PointR {
                    x: Rat::new(-1, 2),
                    y: Rat::new(1, 2),
                };
                let b = PointR {
                    x: Rat::new(0, 1),
                    y: Rat::new(0, 1),
                };
                assert!((p1 == a && p2 == b) || (p1 == b && p2 == a));
            }
            _ => panic!("expected two points"),
        }
    }

    #[test]
    fn ec_two_points_leftish() {
        let l1 = mk_two_point(MIN_PARAM, MIN_PARAM, MIN_PARAM + 1, MIN_PARAM + 1); // y=x
        let l2 = mk_two_point(MIN_PARAM, MIN_PARAM + 1, MIN_PARAM + 1, MIN_PARAM + 2); // y=x+1
        let l3 = mk_can(0, 0, 1, -1); // y=-x
        assert!(matches!(build(l1, l2, l3), Placement::TwoPoints(_, _)));
    }

    #[test]
    fn ec_two_points_rightish() {
        let l1 = mk_two_point(MAX_PARAM - 1, MAX_PARAM - 1, MAX_PARAM, MAX_PARAM);
        let _l2 = mk_two_point(MAX_PARAM - 1, MAX_PARAM, MAX_PARAM, MAX_PARAM + 1);
        let l2 = mk_two_point(10, 11, 11, 12);
        let l3 = mk_can(0, 0, 1, -1);
        assert!(matches!(build(l1, l2, l3), Placement::TwoPoints(_, _)));
    }

    #[test]
    fn ec_two_points_mix() {
        let l1 = mk_two_point(10, 10, 11, 11); // y=x
        let l2 = mk_two_point(10, 11, 11, 12); // y=x+1
        let l3 = mk_can(5, -5, 1, -1);
        assert!(matches!(build(l1, l2, l3), Placement::TwoPoints(_, _)));
    }

    #[test]
    fn ec_two_points_two_near_left_next_and_mid() {
        let l1 = mk_two_point(-2, -2, -1, -1);
        let l2 = mk_two_point(-2, -1, -1, 0);
        let l3 = mk_can(0, 0, 1, -1);
        assert!(matches!(build(l1, l2, l3), Placement::TwoPoints(_, _)));
    }

    #[test]
    fn ec_two_points_two_near_right_prev_and_mid() {
        let l1 = mk_two_point(2, 2, 3, 3);
        let l2 = mk_two_point(2, 3, 3, 4);
        let l3 = mk_can(0, 0, 1, -1);
        assert!(matches!(build(l1, l2, l3), Placement::TwoPoints(_, _)));
    }

    // 3 точки перетину
    #[test]
    fn ec_three_points_typical() {
        let l1 = mk_two_point(0, -1, 0, 1); // x=0
        let l2 = mk_two_point(-1, 0, 1, 0); // y=0
        let l3 = mk_can(0, 1, 1, 1);
        match build(l1, l2, l3) {
            Placement::ThreePoints(p1, p2, p3) => {
                let s: BTreeSet<PointR> = [p1, p2, p3].into_iter().collect();
                let e: BTreeSet<PointR> = [
                    PointR {
                        x: Rat::new(0, 1),
                        y: Rat::new(0, 1),
                    },
                    PointR {
                        x: Rat::new(0, 1),
                        y: Rat::new(1, 1),
                    },
                    PointR {
                        x: Rat::new(-1, 1),
                        y: Rat::new(0, 1),
                    },
                ]
                .into_iter()
                .collect();
                assert_eq!(s, e);
            }
            _ => panic!("expected three points"),
        }
    }

    #[test]
    fn ec_three_points_leftish() {
        let l1 = mk_two_point(0, MIN_PARAM, 0, MIN_PARAM + 1);
        let l2 = mk_two_point(MIN_PARAM, 0, MIN_PARAM + 1, 0);
        let l3 = mk_can(MIN_PARAM, MIN_PARAM + 1, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::ThreePoints(_, _, _)));
    }

    #[test]
    fn ec_three_points_rightish() {
        let l1 = mk_two_point(0, MAX_PARAM - 1, 0, MAX_PARAM);
        let l2 = mk_two_point(MAX_PARAM - 1, 0, MAX_PARAM, 0);
        let l3 = mk_can(MAX_PARAM - 1, MAX_PARAM, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::ThreePoints(_, _, _)));
    }

    #[test]
    fn ec_three_points_mix() {
        let l1 = mk_two_point(0, -10, 0, 10);
        let l2 = mk_two_point(-10, 0, 10, 0);
        let l3 = mk_can(5, 6, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::ThreePoints(_, _, _)));
    }

    #[test]
    fn ec_three_points_two_near_left_next_and_mid() {
        let l1 = mk_two_point(0, -2, 0, -1);
        let l2 = mk_two_point(-2, 0, -1, 0);
        let l3 = mk_can(0, 1, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::ThreePoints(_, _, _)));
    }

    #[test]
    fn ec_three_points_two_near_right_prev_and_mid() {
        let l1 = mk_two_point(0, 2, 0, 3);
        let l2 = mk_two_point(2, 0, 3, 0);
        let l3 = mk_can(0, 1, 1, 1);
        assert!(matches!(build(l1, l2, l3), Placement::ThreePoints(_, _, _)));
    }

    // INVALID equivalence classes (out-of-range, співпадаючі точки, l=0 або m=0)
    #[test]
    fn ie_out_of_range_left() {
        let l1 = mk_two_point(MIN_PARAM - 1, 0, 0, 1);
        let l2 = mk_two_point(0, 0, 1, 1);
        let l3 = mk_can(0, 0, 1, 1);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::OutOfRange { .. })
        ));
    }

    #[test]
    fn ie_out_of_range_right() {
        let l1 = mk_two_point(MAX_PARAM + 1, 0, 0, 1);
        let l2 = mk_two_point(0, 0, 1, 1);
        let l3 = mk_can(0, 0, 1, 1);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::OutOfRange { .. })
        ));
    }

    #[test]
    fn ie_out_of_range_typical() {
        let l1 = mk_two_point(0, 0, 1, 1);
        let l2 = mk_two_point(0, 0, 1, 1);
        let l3 = mk_can(0, 0, 123, 1);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::OutOfRange { .. })
        ));
    }

    #[test]
    fn ie_coincident_points_left() {
        let l1 = mk_two_point(MIN_PARAM, MIN_PARAM, MIN_PARAM, MIN_PARAM);
        let l2 = mk_two_point(0, 0, 1, 1);
        let l3 = mk_can(0, 0, 1, 1);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::DegenerateTwoPointLine { .. })
        ));
    }

    #[test]
    fn ie_coincident_points_right() {
        let l1 = mk_two_point(MAX_PARAM, MAX_PARAM, MAX_PARAM, MAX_PARAM);
        let l2 = mk_two_point(0, 0, 1, 1);
        let l3 = mk_can(0, 0, 1, 1);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::DegenerateTwoPointLine { .. })
        ));
    }

    #[test]
    fn ie_coincident_points_typical() {
        let l1 = mk_two_point(0, 0, 0, 0);
        let l2 = mk_two_point(0, 0, 1, 1);
        let l3 = mk_can(0, 0, 1, 1);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::DegenerateTwoPointLine { .. })
        ));
    }

    #[test]
    fn ie_zero_l_left() {
        let l1 = mk_two_point(0, 0, 1, 1);
        let l2 = mk_two_point(0, 1, 1, 0);
        let l3 = mk_can(0, 0, 0, 1);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::ZeroDirectionComponent { component: "l", .. })
        ));
    }

    #[test]
    fn ie_zero_m_right() {
        let l1 = mk_two_point(0, 0, 1, 1);
        let l2 = mk_two_point(0, 1, 1, 0);
        let l3 = mk_can(0, 0, 1, 0);
        assert!(matches!(
            validate_inputs(l1, l2, l3),
            Err(InputError::ZeroDirectionComponent { component: "m", .. })
        ));
    }

    #[test]
    fn ie_zero_direction_typical() {
        let l1 = mk_two_point(0, 0, 1, 1);
        let l2 = mk_two_point(0, 1, 1, 0);
        let l3 = mk_can(0, 0, 0, 0);
        assert!(matches!(validate_inputs(l1, l2, l3), Err(_)));
    }
}
