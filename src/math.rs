use ggez::graphics::Point2;

pub fn left_turn(points: &[Point2]) -> bool {
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

pub fn smallest_angle_ccw(test_point: Point2, points: &[Point2]) -> Point2 {
    for potential_point in points.iter().filter(|&x| !equal_points(x, &test_point)) {
        let mut most_right = true;
        let mut test_vec = vec![test_point, *potential_point, Point2::new(0.0, 0.0)];
        for point in points
            .iter()
            .filter(|&x| !(equal_points(x, &test_point) && equal_points(x, &potential_point)))
        {
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

pub fn equal_points(a: &Point2, b: &Point2) -> bool {
    if a[0].partial_cmp(&b[0]).unwrap() != std::cmp::Ordering::Equal {
        return false;
    }
    if a[1].partial_cmp(&b[1]).unwrap() != std::cmp::Ordering::Equal {
        return false;
    }
    true
}

// point is in triangle if w1 and w2 are between 0 and 1
pub fn point_in_triangle(p: Point2, triangle: &[Point2]) -> (f32, f32) {
    if triangle.len() != 3 {
        panic!("Triangle must have 3 points and not {}", triangle.len());
    }
    let a = triangle[0];
    let b = triangle[1];
    let c = triangle[2];

    let s1 = c[1] - a[1];
    let s2 = c[0] - a[0];
    let s3 = b[1] - a[1];
    let s4 = p[1] - a[1];

    let w1 = (a[0] * s1 + s4 * s2 - p[0] * s1) / (s3 * s2 - (b[0] - a[0]) * s1);
    let w2 = (s4 - w1 * s3) / s1;

    (w1, w2)
}

#[cfg(test)]
mod tests {
    // use ggez::graphics::Point2;
    use super::*;
    #[test]
    fn left_turn() {
        use super::left_turn;
        let is_left = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 1.0),
        ];
        assert_eq!(left_turn(&is_left[..]), true);

        let straight = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(2.0, 0.0),
        ];
        assert_eq!(left_turn(&straight[..]), false);

        let is_right = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 0.0),
        ];
        assert_eq!(left_turn(&is_right[..]), false);
    }
}
