use ggez::graphics::Point2;

pub fn grahams_scan(points: &[Point2]) -> Vec<Point2> {
    debug!("Recomputed convex hull with graham's scan:");
    if points.len() < 3 {
        debug!("Less then 3 points can't do graham's scan");
        return Vec::new();
    }

    // sort points lexicographically by x then y
    let mut points = points.to_vec();
    points.sort_by(|a, b| {
        a[0].partial_cmp(&b[0])
            .unwrap()
            .then_with(|| a[1].partial_cmp(&b[1]).unwrap())
    });

    // compute upper half
    let mut upper = Vec::new();
    upper.extend_from_slice(&points[..2]);
    for point in &points[2..] {
        upper.push(point.clone());
        while 2 < upper.len() && left_turn(&upper[upper.len() - 3..]) {
            upper.remove(upper.len() - 2);
        }
    }

    // computer lower half
    let mut lower: Vec<Point2> = Vec::new();
    lower.extend_from_slice(&points[points.len() - 2..]);
    lower.reverse();
    for point in points[..points.len() - 2].iter().rev() {
        lower.push(point.clone());
        while 2 < lower.len() && left_turn(&lower[lower.len() - 3..]) {
            lower.remove(lower.len() - 2);
        }
    }

    // combine upper and lower half
    lower.remove(0);
    lower.pop();

    let mut polygon = Vec::with_capacity(upper.len() + lower.len());
    polygon.append(&mut upper);
    polygon.append(&mut lower);
    polygon
}

pub fn jarvis_march(points: &[Point2]) -> Vec<Point2> {
    debug!("Recomputed convex hull with jarvi's march:");

    // sort points lexicographically by y then x
    let mut points = points.to_vec();
    points.sort_by(|a, b| {
        a[1].partial_cmp(&b[1])
            .unwrap()
            .then_with(|| a[0].partial_cmp(&b[0]).unwrap())
    });

    let mut current_point = points[0];
    let mut polygon = vec![current_point];

    while {
        current_point = smallest_angle_ccw(current_point, &points[..]);
        polygon.push(current_point);
        current_point != points[0]
    } {}


    polygon
}

fn left_turn(points: &[Point2]) -> bool {
    if points.len() != 3 {
        panic!(
            "tried to calculate left turn for {} points instead of 3",
            points.len()
        );
    }
    let a = Point2::new(points[1][0] - points[0][0], points[1][1] - points[0][1]);
    let b = Point2::new(points[2][0] - points[0][0], points[2][1] - points[0][1]);

    (a[0] * b[1] - b[0] * a[1]) > 0.0
}

fn smallest_angle_ccw(test_point: Point2, points: &[Point2]) -> Point2 {
    for potential_point in points.iter().filter(|&x| !equal_points(x, &test_point)) {
        let mut most_right = true;
        let mut test_vec = vec![test_point, *potential_point, Point2::new(0.0, 0.0)];
        for point in points.iter().filter(|&x| !(equal_points(x, &test_point) && equal_points(x, &potential_point))) {
            test_vec[2] = *point;
            if left_turn(&test_vec[..]) {
                most_right = false;
                break;
            }
        }
        if most_right {
            return *potential_point;
        }
    }
    return test_point;
}

fn equal_points(a: &Point2, b: &Point2) -> bool {
    if a[0].partial_cmp(&b[0]).unwrap() != std::cmp::Ordering::Equal {
        return false;
    }
    if a[1].partial_cmp(&b[1]).unwrap() != std::cmp::Ordering::Equal {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    // use ggez::graphics::Point2;
    use super::*;
    #[test]
    fn left_turn() {
        use super::left_turn;
        let is_left = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(1.0, 1.0)];
        assert_eq!(left_turn(&is_left[..]), true);

        let straight = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(2.0, 0.0)];
        assert_eq!(left_turn(&straight[..]), false);

        let is_right = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 1.0), Point2::new(2.0, 0.0)];
        assert_eq!(left_turn(&is_right[..]), false);
    }
}
