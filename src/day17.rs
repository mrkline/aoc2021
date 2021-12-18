use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Target {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}

#[aoc_generator(day17)]
pub fn parse_target(input: &str) -> Target {
    // let input = "target area: x=20..30, y=-10..-5";

    assert!(input.starts_with("target area: x="));
    let mut dimension_split = input["target area: x=".len()..].split(", y=");
    let mut x_split = dimension_split.next().unwrap().split("..");
    let left = x_split.next().unwrap().parse().unwrap();
    let right = x_split.next().unwrap().parse().unwrap();
    assert!(left <= right);

    let mut y_split = dimension_split.next().unwrap().split("..");
    let bottom = y_split.next().unwrap().parse().unwrap();
    let top = y_split.next().unwrap().parse().unwrap();
    assert!(bottom <= top);

    Target {
        left,
        right,
        bottom,
        top,
    }
}

fn hits_with_apex(t: &Target, mut dy: i64) -> Option<i64> {
    let mut apex = i64::MIN;
    let mut y = 0;

    while dy > 0 || y >= t.bottom {
        y += dy;
        if y > apex {
            apex = y;
        }
        dy -= 1;

        if (t.bottom..=t.top).contains(&y) {
            return Some(apex);
        }
    }
    None
}

#[aoc(day17, part1)]
pub fn part1(t: &Target) -> i64 {
    (1..=100)
        .filter_map(|dy| hits_with_apex(t, dy))
        .max()
        .unwrap()
}

fn hits(t: &Target, mut dx: i64, mut dy: i64) -> bool {
    let mut x = 0;
    let mut y = 0;

    while x <= t.right && y >= t.bottom {
        x += dx;
        dx -= dx.signum();

        y += dy;
        dy -= 1;

        if (t.left..=t.right).contains(&x) && (t.bottom..=t.top).contains(&y) {
            return true;
        }
    }
    false
}

#[aoc(day17, part2)]
pub fn part2(t: &Target) -> i64 {
    // Be lazy and simplify math.
    assert!(t.left > 0);
    assert!(t.top < 0);

    let mut good_muzzle_velocities = 0;

    let far_x = if t.left > t.right { t.left } else { t.right };

    for dx in 0..=far_x {
        for dy in t.bottom..=100 {
            if hits(t, dx, dy) {
                good_muzzle_velocities += 1;
            }
        }
    }

    good_muzzle_velocities
}
