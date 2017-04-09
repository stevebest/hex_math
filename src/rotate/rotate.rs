use std::borrow::Borrow;

use structs::{CubePoint, Point};

/// Rotate the point around a provided center, keeping the same height
///
/// Positive rotations are clockwise. Six rotations bring a point back to the
/// starting position.
pub fn rotate<T: Borrow<Point>>(
  point: &T,
  center: &T,
  mut times: i32,
) -> Point {
  let point = point.borrow();
  let center = center.borrow();

  if point == center {
    return *point;
  }

  let relative_point: Point = point - center;
  let CubePoint(q, r, s, t) = relative_point.into();

  times %= 6;

  if times < 0 {
    times += 6;
  }

  let rotated_point: Point = match times {
    0 => relative_point,
    1 => Point(-r, -s, t),
    2 => Point( s,  q, t),
    3 => Point(-q, -r, t),
    4 => Point( r,  s, t),
    5 => Point(-s, -q, t),
    _ => panic!(),
  };

  let result: Point = &rotated_point + &center;

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rotate() {
    let point: Point = Point(1, 2, 5);
    let center: Point = Point(1, 1, 5);

    assert!(Point(0, 1, 5) == super::rotate(&point, &center, 2));
  }
}