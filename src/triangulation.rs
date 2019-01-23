use ggez::graphics::Point2;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::math;

pub fn delaunay(points: &[Point2]) -> Vec<[Point2; 3]> {
    // https://github.com/tynril/rtriangulate/blob/master/src/lib.rs
    if points.len() < 3 {
        return Vec::new()
    }
    // shuffle points
    let mut shuffeld: Vec<&Point2> = points.iter().collect();
    let mut rng = thread_rng();
    shuffeld.shuffle(&mut rng);

    // let mut points = points.to_vec();
    // points.sort_by(|a, b| {
    //     a[0].partial_cmp(&b[0])
    //         .unwrap()
    //         .then_with(|| a[1].partial_cmp(&b[1]).unwrap())
    // });

    let initial_triangle = initial_triangle(&points);
    let mut triangles = Vec::new();
    triangles.push(initial_triangle);

    // return triangles;

    for (i, p) in points.iter().enumerate() {
        // let triangle = triangles.iter().find(|&& x| math::point_in_triangle(*p, &x)).expect(&format!("Point({}) was not inside any triangle, this can't happen", i));

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
        let mut flipped_edge = true;
        // as long as we found one triangle pair where we needed to flip an edge we are trying again
        'outer: while flipped_edge {
            // get all triangles that contain p
            let p_triangles: Vec<[Point2; 3]> = triangles.clone().into_iter().filter(|x| {
                *p == x[0] || *p == x[1] || *p == x[2]
            }).collect();

            for p_triangle in p_triangles.iter() {
                // search triangle that shares edge with p_triangle
                let copy = p_triangles.clone();
                for triangle in copy.into_iter().filter(|x| *x != *p_triangle) {
                    // unique point of p_triangle
                    let l1 = p_triangle.iter().filter(|x| !triangle.contains(x)).next();
                    // unique point of triangle
                    let l2 = triangle.iter().filter(|x| !p_triangle.contains(x)).next();
                    // point1 of shared edge = p
                    let p1 = triangle.iter().filter(|x| *x==p).next();
                    // point2 of shared edge
                    let p2 = p_triangle.iter().filter(|x| triangle.contains(x) && *x != p).next();

                    match (l1, l2, p1, p2) {
                        (Some(l1), Some(l2), Some(p1), Some(p2)) => {
                            let t1 = [*l1, *l2, *p2];
                            let t2 = [*l1, *l2, *p1];
                            // check delauney condition, flip edge if violated
                            if math::point_in_triangle_circle(**&p1, &t1[..]) {
                                triangles.remove_item(&triangle).expect("tried to remove triangle but didnt find one");
                                triangles.remove_item(&p_triangle).expect("tried to remove p_triangle but didnt find one");
                                triangles.push(t1);
                                triangles.push(t2);
                                // repeat from the beginning
                                continue 'outer;
                            }
                        }
                        // triangle doesn't share edge with p_triangle, check next one
                        _ => {}
                    }
                }
            }
            flipped_edge = false;
        }
    }

    // remove triangles that contain point of initial triangle
    for p in &[initial_triangle[0], initial_triangle[1], initial_triangle[2]] {
        let removed_triangles: Vec<[Point2; 3]> = triangles.drain_filter(|x| {
            x[0] == *p || x[1] == *p || x[2] == *p
        }).collect();
    }

    // remove all triangles that don't contain point of initial
    // let it = initial_triangle;
    // let removed_triangles: Vec<[Point2; 3]> = triangles.drain_filter(|x| {
    //     !((x[0] == it[0] || x[1] == it[0] || x[2] == it[0]) 
    //     || (x[0] == it[1] || x[1] == it[1] || x[2] == it[1]) 
    //     || (x[0] == it[2] || x[1] == it[2] || x[2] == it[2])) 
    // }).collect();

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



    // let (min_point, max_point) = points.iter().fold(
    //     (
    //         Point2::new(std::f32::INFINITY, std::f32::INFINITY),
    //         Point2::new(std::f32::NEG_INFINITY, std::f32::NEG_INFINITY),
    //     ),
    //     |acc, p| {
    //         (
    //             Point2::new(acc.0[0].min(p[0]), acc.0[1].min(p[1])),
    //             Point2::new(acc.1[0].max(p[0]), acc.1[1].max(p[1])),
    //         )
    //     },
    // );

    // let half = 0.5;
    // let two = 2.0;

    // let delta_point =
    //     Point2::new(max_point[0] - min_point[0], max_point[1] - min_point[1]);
    // let delta_max = delta_point.x.max(delta_point.y);
    // let mid_point = Point2::new(
    //     (max_point[0] + min_point[0]) * half,
    //     (max_point[1] + min_point[1]) * half,
    // );

    // let p1 = Point2::new(mid_point[0] - two * delta_max, mid_point[1] - delta_max);
    // let p2 = Point2::new(mid_point[0], mid_point[1] + two * delta_max);
    // let p3 = Point2::new(mid_point[0] + two * delta_max, mid_point.y - delta_max);

    // [p1, p2, p3]
}