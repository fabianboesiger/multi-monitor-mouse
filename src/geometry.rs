use std::ops::{Add, Sub, Mul, Div};


pub trait Shape {}

pub trait Intersection<S>
where
    S: Shape,
    Self: Shape,
{
    fn intersection(&self, other: &S) -> Option<Point>;
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn dist(&self, point: &Point) -> f64 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2)).sqrt()
    }

    pub fn zero() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}


/*
impl<S, O> Intersection<O> for S
where
    O: Shape,
    S: Shape,
    O: Intersection<S>
{
    fn intersection(&self, other: &O) -> Option<Point> {
        other.intersection(self)
    }
}
*/

#[derive(Debug, Clone, Copy)]
pub struct BoundedLine {
    pub p1: Point,
    pub p2: Point,
}

impl Shape for BoundedLine {}

impl From<UnboundedLine> for BoundedLine {
    fn from(UnboundedLine { p1, p2 }: UnboundedLine) -> Self {
        BoundedLine { p1, p2 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnboundedLine {
    pub p1: Point,
    pub p2: Point,
}

impl From<BoundedLine> for UnboundedLine {
    fn from(BoundedLine { p1, p2 }: BoundedLine) -> Self {
        UnboundedLine { p1, p2 }
    }
}

impl Shape for UnboundedLine {}

#[derive(Debug, Clone, Copy)]
pub struct VerticalLine {
    x: f64,
    y1: f64,
    y2: f64,
}

impl Shape for VerticalLine {}

impl Intersection<UnboundedLine> for VerticalLine {
    fn intersection(&self, other: &UnboundedLine) -> Option<Point> {
        let y_intersection = other.p1.y + (other.p2.y - other.p1.y) *
            ((self.x - other.p1.x) / (other.p2.x - other.p1.x)); 

        if y_intersection >= self.y1.min(self.y2) && y_intersection < self.y1.max(self.y2) {
            Some(Point {
                x: self.x,
                y: y_intersection,
            })
        } else {
            None
        }
    }
}

impl Intersection<BoundedLine> for VerticalLine {
    fn intersection(&self, other: &BoundedLine) -> Option<Point> {
        if self.x >= other.p1.x.min(other.p2.x) && self.x < other.p1.x.max(other.p2.x) {
            self.intersection(&UnboundedLine::from(*other))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalLine {
    y: f64,
    x1: f64,
    x2: f64,
}

impl Shape for HorizontalLine {}

impl Intersection<UnboundedLine> for HorizontalLine {
    fn intersection(&self, other: &UnboundedLine) -> Option<Point> {
        let x_intersection = other.p1.x + (other.p2.x - other.p1.x) *
            ((self.y - other.p1.y) / (other.p2.y - other.p1.y)); 

        if x_intersection >= self.x1.min(self.x2) && x_intersection < self.x1.max(self.x2) {
            Some(Point {
                x: x_intersection,
                y: self.y,
            })
        } else {
            None
        }
    }
}

impl Intersection<BoundedLine> for HorizontalLine {
    fn intersection(&self, other: &BoundedLine) -> Option<Point> {
        if self.y >= other.p1.y.min(other.p2.y) && self.y < other.p1.y.max(other.p2.y) {
            self.intersection(&UnboundedLine::from(*other))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    left: VerticalLine,
    right: VerticalLine,
    top: HorizontalLine,
    bottom: HorizontalLine,
}

impl Shape for Rect {}

impl Intersection<UnboundedLine> for Rect {
    fn intersection(&self, other: &UnboundedLine) -> Option<Point> {
        [
            self.left.intersection(other),
            self.right.intersection(other),
            self.top.intersection(other),
            self.bottom.intersection(other),
        ]
            .into_iter()
            .filter_map(|points| points)
            .map(|point| (point.dist(&other.p1), point))
            .min_by(|(dist1, _), (dist2, _)| dist1.partial_cmp(dist2).unwrap())
            .map(|(_, point)| point)
    }
}

impl Intersection<BoundedLine> for Rect {
    fn intersection(&self, other: &BoundedLine) -> Option<Point> {
        let bounding_rect = Rect::from_points(other.p1, other.p2);

        nearest([
            self.left.intersection(other),
            self.right.intersection(other),
            self.top.intersection(other),
            self.bottom.intersection(other),
        ]
            .into_iter()
            .filter_map(|points| points)
            .filter(|point| bounding_rect.includes(point)),
            other.p1)
    }
}

pub fn nearest(iter: impl Iterator<Item = Point>, target: Point) -> Option<Point> {
    iter
        .map(|point| (point, point.dist(&target)))
        .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
        .map(|(point, _)| point)
}

impl Rect {
    pub fn from_points(p1: Point, p2: Point) -> Self {
        let x_min = p1.x.min(p2.x);
        let x_max = p1.x.max(p2.x);
        let y_min = p1.y.min(p2.y);
        let y_max = p1.y.max(p2.y);

        Rect {
            left: VerticalLine { x: x_min, y1: y_min, y2: y_max },
            right: VerticalLine { x: x_max, y1: y_min, y2: y_max },
            top: HorizontalLine { y: y_min, x1: x_min, x2: x_max },
            bottom: HorizontalLine { y: y_max, x1: x_min, x2: x_max },
        }
    }

    pub fn includes(&self, p: &Point) -> bool {
        return p.x >= self.left.x && p.x < self.right.x && p.y >= self.top.y && p.y < self.bottom.y
    }

    pub fn exited_by(&self, movement: &BoundedLine) -> bool {
        self.includes(&movement.p1) && (
            self.left.intersection(movement).is_some() ||
            self.right.intersection(movement).is_some() ||
            self.top.intersection(movement).is_some() ||
            self.bottom.intersection(movement).is_some()
        )
    }
}
