use ggez::graphics::Point2;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::math;

pub fn delaunay(points: &[Point2]) -> Vec<[Point2; 3]> {
    if points.len() < 3 {
        return Vec::new()
    }
    // shuffle points
    let mut shuffeld: Vec<&Point2> = points.iter().collect();
    let mut rng = thread_rng();
    shuffeld.shuffle(&mut rng);

    let initial_triangle = initial_triangle(&points);
    let mut triangles = Vec::new();
    triangles.push(initial_triangle);

    for p in points.iter() {
        let triangle = triangles.iter().find(|&& x| math::point_in_triangle(*p, &x)).expect("Point was not inside any triangle, this can't happen");

        // collect triangles to remove
        let violating_triangles: Vec<[Point2; 3]> = triangles.clone().into_iter().filter(|x| {
            math::point_in_triangle_circle(*p, &x[..])
        }).collect();


        // collect edges of violating triangles
        let mut edges: Vec<[Point2; 2]> = Vec::with_capacity(violating_triangles.len() * 3);
        for triangle in violating_triangles.iter() {
            for edge in [[triangle[0], triangle[1]], [triangle[1], triangle[2]], [triangle[2], triangle[0]]].into_iter() {
                if !edges.contains(edge) && !edges.contains(&[edge[1], edge[0]]) {
                    edges.push(*edge);
                }
            }
        }

        // remove edges that are part of more then two violating triangles
        let edges: Vec<[Point2; 2]> = edges.into_iter().filter(|edge| {
            let mut count = 0;
            for triangle in violating_triangles.iter() {
                if triangle_has_edge(*triangle, *edge) {
                    count += 1;
                }
            }
            count == 1
        }).collect();

        // remove violating triangles
        for triangle in violating_triangles.into_iter() {
            triangles.remove_item(&triangle);
        }


        // create new triangles by connecting point to edges
        for edge in edges {
            triangles.push([edge[0], edge[1], *p]);
        }

    }

    // for every point of bounding triangle
    for p in &[initial_triangle[0], initial_triangle[1], initial_triangle[2]] {
        // remove every triangle that contains p

        let removed_triangles: Vec<[Point2; 3]> = triangles.drain_filter(|x| {
            x[0] == *p || x[1] == *p || x[2] == *p
        }).collect();

        for t1 in removed_triangles.iter() {
            for t2 in removed_triangles.iter().filter(|x| *x != t1) {
                // check if triangle share edge -> two points are the same
                let mut same_points = 0;
                if t1[0] == t2[0] || t1[0] == t2[1] || t1[0] == t2[2] {
                    same_points += 1;
                }
                if t1[1] == t2[0] || t1[1] == t2[1] || t1[1] == t2[2] {
                    same_points += 1;
                }
                if t1[2] == t2[0] || t1[2] == t2[1] || t1[2] == t2[2] {
                    same_points += 1;
                }

                let intial_points = [initial_triangle[0], initial_triangle[1], initial_triangle[2]].iter().filter(|&p| {
                    *p == t1[0] || *p == t1[1] || *p == t1[2] || *p == t2[0] || *p == t2[1] || *p == t2[2]
                }).collect::<Vec<&Point2>>().len();

                if same_points == 2 && intial_points == 1 {
                    // check if we need to to flip edge
                    let mut potential_triangle = Vec::new();
                    if t1[0] != *p && !potential_triangle.contains(&t1[0]) {
                        potential_triangle.push(t1[0]);
                    }
                    if t1[1] != *p && !potential_triangle.contains(&t1[1]) {
                        potential_triangle.push(t1[1]);
                    }
                    if t1[2] != *p && !potential_triangle.contains(&t1[2]) {
                        potential_triangle.push(t1[2]);
                    }

                    if t2[0] != *p && !potential_triangle.contains(&t2[0]) {
                        potential_triangle.push(t2[0]);
                    }
                    if t2[1] != *p && !potential_triangle.contains(&t2[1]) {
                        potential_triangle.push(t2[1]);
                    }
                    if t2[2] != *p && !potential_triangle.contains(&t2[2]) {
                        potential_triangle.push(t2[2]);
                    }

                    if math::point_in_triangle_circle(*p, &potential_triangle[..]) {
                        log::info!("Flip edge!");
                        triangles.push([potential_triangle[0], potential_triangle[1], potential_triangle[2]])
                    }
                }
            }
        }
    }

    triangles
}

fn triangle_has_edge(triangle: [Point2; 3], edge: [Point2; 2]) -> bool {
    let edges: Vec<[Point2; 2]> = vec![[triangle[0], triangle[1]], [triangle[1], triangle[2]], [triangle[2], triangle[0]]];

    if edges.contains(&edge) || edges.contains(&[edge[1], edge[0]]) {
        true
    } else {
        false
    }
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

    x_min *= 0.99;
    y_min *= 0.99;

    let x_len = (x_max - x_min) * 1.05;
    let y_len = (y_max - y_min) * 1.05;
    let bottem_left = Point2::new(x_min, y_min);
    let bottem_right = Point2::new(x_max+x_len, y_min);
    let top_left = Point2::new(x_min, y_max+y_len);

    [bottem_left, bottem_right, top_left]
}