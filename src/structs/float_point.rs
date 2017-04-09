use std::ops::{Add, Sub};

use structs::Point;

/// Point on a coordinate plane with floating point coordinate values
#[derive(Debug, PartialEq)]
pub struct FloatPoint(pub f32, pub f32, pub f32);

impl FloatPoint {
  /// Round a float point back to a standard point
  pub fn round(&self) -> Point {
    let &FloatPoint(q, r, t) = self;
    let s = self.s();

    let mut rq = q.round();
    let mut rr = r.round();

    let rs = s.round();
    let rt = t.round();

    let dq = (rq - q).abs();
    let dr = (rr - r).abs();
    let ds = (rs - s).abs();

    if (dq > ds) && (dq > dr) {
      rq = -rs - rr;
    } else if ds < dr {
      rr = -rq - rs;
    }

    let point: Point = Point(rq as i32, rr as i32, rt as i32);

    point
  }

  /// Calculate the S coordinate
  pub fn s(&self) -> f32 {
    let &FloatPoint(q, r, _) = self;
    let s = -q - r;

    s
  }
}

/// Add one point to another
impl<'a, 'b> Add<&'b FloatPoint> for &'a FloatPoint {
  type Output = FloatPoint;

  fn add(self, other: &'b FloatPoint) -> FloatPoint {
    let &FloatPoint(q0, r0, t0) = self;
    let &FloatPoint(q1, r1, t1) = other;

    FloatPoint(q0 + q1, r0 + r1, t0 + t1)
  }
}

/// Subtract one point from another
impl<'a, 'b> Sub<&'b FloatPoint> for &'a FloatPoint {
  type Output = FloatPoint;

  fn sub(self, other: &'b FloatPoint) -> FloatPoint {
    let &FloatPoint(q0, r0, t0) = self;
    let &FloatPoint(q1, r1, t1) = other;

    FloatPoint(q0 - q1, r0 - r1, t0 - t1)
  }
}

/// Convert from a Point to a FloatPoint
impl From<Point> for FloatPoint {
  fn from(point: Point) -> FloatPoint {
    let Point(q, r, t) = point;

    FloatPoint(q as f32, r as f32, t as f32)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn round() {
    let Point(q, r, t) = FloatPoint(1.6f32, 1.6f32, 2.5f32).round();

    assert!(2 == q);
    assert!(1 == r);
    assert!(3 == t);
  }

  #[test]
  fn s() {
    let point = FloatPoint(1f32, 2f32, 5f32);

    assert!(-3f32 == point.s());
  }

  #[test]
  fn add() {
    let point: FloatPoint = FloatPoint(1f32, 2f32, 5f32);
    let other: FloatPoint = FloatPoint(3f32, 4f32, 10f32);
    let FloatPoint(q, r, t) = &point + &other;

    assert!(4f32 == q);
    assert!(6f32 == r);
    assert!(15f32 == t);
  }

  #[test]
  fn sub() {
    let point: FloatPoint = FloatPoint(1f32, 2f32, 5f32);
    let other: FloatPoint = FloatPoint(3f32, 4f32, 10f32);
    let FloatPoint(q, r, t) = &point - &other;

    assert!(-2f32 == q);
    assert!(-2f32 == r);
    assert!(-5f32 == t);
  }

  #[test]
  fn from_point() {
    let point = Point(1, 2, 5);
    let FloatPoint(q, r, t) = point.into();

    assert!(1f32 == q);
    assert!(2f32 == r);
    assert!(5f32 == t);
  }
}
