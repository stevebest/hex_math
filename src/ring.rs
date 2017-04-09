use std::borrow::Borrow;
use std::collections::HashSet;

use enums::Direction;
use structs::Point;
use travel::travel;

/// Find points in a spherical ring of a provided radius
pub fn ring<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = ring_2d(point, range);

  for index in 1..range + 1 {
    let diff = range - index;
    let up: Point = travel(point, &Direction::Up, index);
    let down: Point = travel(point, &Direction::Down, index);
    let up_ring: HashSet<Point> = ring_2d(&up, diff);
    let down_ring: HashSet<Point> = ring_2d(&down, diff);

    set.extend(up_ring);
    set.extend(down_ring);
  }

  set.insert(travel(point, &Direction::Up, range));
  set.insert(travel(point, &Direction::Down, range));

  set
}

/// Find points at the same height in a ring of a provided radius
pub fn ring_2d<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = HashSet::new();
  let mut point: Point = travel(point, &Direction::Northwest, range);

  for direction in Direction::to_vec().iter().take(6) {
    for _ in 0..range {
      let next: Point = travel(&point, direction, 1);

      set.insert(point.into());
      point = next;
    }
  }

  set
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ring() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::ring(&point, 2);

    assert!(set.contains(&Point(1, 0, 5)));
    assert!(set.contains(&Point(2, 0, 5)));
    assert!(set.contains(&Point(3, 0, 5)));
    assert!(set.contains(&Point(3, 1, 5)));
    assert!(set.contains(&Point(3, 2, 5)));
    assert!(set.contains(&Point(2, 3, 5)));
    assert!(set.contains(&Point(1, 4, 5)));
    assert!(set.contains(&Point(0, 4, 5)));
    assert!(set.contains(&Point(-1, 4, 5)));
    assert!(set.contains(&Point(-1, 3, 5)));
    assert!(set.contains(&Point(-1, 2, 5)));
    assert!(set.contains(&Point(0, 1, 5)));

    assert!(set.contains(&Point(1, 1, 6)));
    assert!(set.contains(&Point(2, 1, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.contains(&Point(1, 3, 6)));
    assert!(set.contains(&Point(0, 3, 6)));
    assert!(set.contains(&Point(0, 2, 6)));

    assert!(set.contains(&Point(1, 1, 4)));
    assert!(set.contains(&Point(2, 1, 4)));
    assert!(set.contains(&Point(2, 2, 4)));
    assert!(set.contains(&Point(1, 3, 4)));
    assert!(set.contains(&Point(0, 3, 4)));
    assert!(set.contains(&Point(0, 2, 4)));

    assert!(set.contains(&Point(1, 2, 7)));
    assert!(set.contains(&Point(1, 2, 3)));
    assert!(set.len() == 26);
  }

  #[test]
  fn ring_2d() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::ring_2d(&point, 1);

    assert!(set.contains(&Point(1, 1, 5)));
    assert!(set.contains(&Point(2, 1, 5)));
    assert!(set.contains(&Point(2, 2, 5)));
    assert!(set.contains(&Point(1, 3, 5)));
    assert!(set.contains(&Point(0, 3, 5)));
    assert!(set.contains(&Point(0, 2, 5)));
    assert!(set.len() == 6);
  }
}
