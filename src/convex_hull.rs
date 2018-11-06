use ggez::graphics::Point2;
use crate::math::*;

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
