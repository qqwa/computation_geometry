
#[derive(Clone, Debug)]
pub struct KdTree(pub Box<Node>);

impl KdTree {
    pub fn new(points: &[(f32, f32)]) -> Self {
        Self::with_orientation_even(&points, Orientation::Horizontal)
    }

    pub fn with_orientation_even(points: &[(f32, f32)], orientation_even: Orientation) -> Self {
        if points.len() == 0 {
            panic!("Tried to construct kd-tree with 0 points");
        }


        // sorted lists of references/pointers to points
        let mut pre_sorted_x: Vec<&(f32, f32)> = points.iter().map(|p| p).collect();
        pre_sorted_x.sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap()
        });
        let mut pre_sorted_y: Vec<&(f32, f32)> = points.iter().map(|p| p).collect();
        pre_sorted_y.sort_by(|a, b| {
            a.1.partial_cmp(&b.1).unwrap()
        });

        let node = Self::construct_balanced_2d_tree(&pre_sorted_x[..], &pre_sorted_y[..], orientation_even);

//        let mut depth = 1;
//        let mut knot = Node::Knot {
//            key: Key {
//                orientation: Orientation::Vertical,
//                value: 10.0,
//            },
//            left: None,
//            right: None,
//        };
//
//        if let Node::Knot{ref mut left, ref mut right, ..} = &mut knot {
//            left.replace(box Node::Leaf {
//                orientation: Orientation::Vertical,
//                value: (1.0, 1.0),
//            });
//            right.replace(box Node::Leaf {
//                orientation: Orientation::Vertical,
//                value: (1.0, 5.0),
//            });
//        }

        KdTree(Self::construct_balanced_2d_tree(&pre_sorted_x, &pre_sorted_y, orientation_even).unwrap())
    }

    fn construct_balanced_2d_tree(x: &[&(f32, f32)], y: &[&(f32, f32)], orientation: Orientation) -> Option<Box<Node>> {
        assert_eq!(x.len(), y.len());
        if x.len() == 0 {
            return None;
        }
        let median = x.len() / 2;
        match orientation {
            Orientation::Horizontal => {
                if x.len() == 1 {
                    // create Leaf
                    Some(Box::new(Node::Leaf {
//                        orientation: Orientation::Horizontal,
                        value: (x[0].0, x[0].1),
                    }))
                } else {
                    // create Node with key as y
                    let key = *y[median];

                    // partioning
                    let y_left = &y[..median];
                    let y_right = &y[median..];

                    let mut x_left = Vec::with_capacity(y_left.len());
                    let mut x_right = Vec::with_capacity(y_right.len());
                    for point in x.iter() {
                        if point.1 < key.1 {
                            x_left.push(*point);
                        } else if key.1 < point.1 {
                            x_right.push(*point);
                        } else if point.1 == key.1 {
                            if key.0 <= point.0 {
                                x_right.push(*point);
                            } else {
                                x_left.push(*point);
                            }
                        } else {
                            panic!("Something went terible wrong");
                        }
                    }

//                    println!("left len: should {} is {}", y_left.len(), x_left.len());
//                    println!("right len: should {} is {}", y_right.len(), x_right.len());

                    Some(Box::new(Node::Knot {
                        key: Key { value: key.1, orientation: Orientation::Horizontal },
                        left: Self::construct_balanced_2d_tree(&x_left[..], y_left, Orientation::Vertical),
                        right: Self::construct_balanced_2d_tree(&x_right[..], y_right, Orientation::Vertical),
                    }))

                }
            }
            Orientation::Vertical => {
                if x.len() == 1 {
                    // create Leaf
                    Some(Box::new(Node::Leaf {
//                        orientation: Orientation::Vertical,
                        value: (x[0].0, x[0].1),
                    }))

                } else {
                    // create Node with key as x

                    let key = *x[median];

                    // partioning
                    let x_left = &x[..median];
                    let x_right = &x[median..];

                    let mut y_left = Vec::with_capacity(x_left.len());
                    let mut y_right = Vec::with_capacity(x_right.len());
                    for point in y.iter() {
                        if point.0 < key.0 {
                            y_left.push(*point);
                        } else if key.0 < point.0 {
                            y_right.push(*point);
                        } else if point.0 == key.0 {
                            if key.1 <= point.1 {
                                y_right.push(*point);
                            } else {
                                y_left.push(*point);
                            }
                        } else {
                            panic!("Something went terible wrong");
                        }
                    }

//                    println!("left len: should {} is {}", y_left.len(), x_left.len());
//                    println!("right len: should {} is {}", y_right.len(), x_right.len());

                    Some(Box::new(Node::Knot {
                        key: Key { value: key.0, orientation: Orientation::Vertical },
                        left: Self::construct_balanced_2d_tree(x_left, &y_left[..], Orientation::Horizontal),
                        right: Self::construct_balanced_2d_tree(x_right, &y_right[..], Orientation::Horizontal),
                    }))
                }
            }
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Orientation {
    // y coord is key
    Horizontal,
    // x coord ist key
    Vertical,
}

#[derive(Clone, Debug)]
pub struct Key {
    pub orientation: Orientation,
    pub value: f32,
}

#[derive(Clone, Debug)]
pub enum Node {
    Knot {
        key: Key,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
    Leaf {
//        orientation: Orientation,
        value: (f32, f32),
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point_list() -> Vec<(f32, f32)> {
        vec![
            (20.0, 20.0),
            (10.0, 15.0),
            (15.0, 5.0),
            (-20.1, 24.0),
        ]
    }

    #[test]
    #[should_panic]
    fn construct_kd_tree_from_empty_list_panics() {
        let points: Vec<(f32, f32)> = Vec::new();
        KdTree::new(&points);
    }
}