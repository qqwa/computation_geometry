use ggez::graphics::Point2;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn delaunay(points: &[Point2]) -> Vec<[Point2; 3]> {
    if points.len() < 3 {
        return Vec::new()
    }
    // shuffle points
    let mut shuffeld: Vec<&Point2> = points.iter().collect();
    let mut rng = thread_rng();
    shuffeld.shuffle(&mut rng);

    let mut triangles = Vec::new();
    triangles.push(initial_triangle(&points));


    triangles
}


fn initial_triangle(points: &[Point2]) -> [Point2; 3] {
    let mut x_min = points[0].x;
    let mut x_max = points[0].x;
    let mut y_min = points[0].y;
    let mut y_max = points[0].y;

    for point in &points[..] {
        if point.x < x_min {
            x_min = point.x;
        }
        if x_max < point.x {
            x_max = point.x;
        }
        if point.y < y_min {
            y_min = point.y;
        }
        if y_max < point.y {
            y_max = point.y;
        }
    }

    let x_len = (x_max - x_min) * 1.05;
    let y_len = (y_max - y_min) * 1.05;
    let bottem_left = Point2::new(x_min, y_min);
    let bottem_right = Point2::new(x_max+x_len, y_min);
    let top_left = Point2::new(x_min, y_max+y_len);

    [bottem_left, bottem_right, top_left]
}