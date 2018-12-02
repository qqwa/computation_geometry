
#[derive(Clone, Debug)]
pub struct KdTree(Node);

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

        let mut depth = 1;
        let mut knot = Node::Knot {
            key: Key {
                orientation: Orientation::Vertical,
                value: 10.0,
            },
            left: None,
            right: None,
        };

        if let Node::Knot{ref mut left, ref mut right, ..} = &mut knot {
            left.replace(box Node::Leaf {
                orientation: Orientation::Vertical,
//                parent: box &knot,
                value: (1.0, 1.0),
            });
            right.replace(box Node::Leaf {
                orientation: Orientation::Vertical,
//                parent: box &knot,
                value: (1.0, 5.0),
            });
        }

        KdTree(knot)
    }

    fn construct_balanced_2d_tree(x: &[&(f32, f32)], y: &[&(f32, f32)], orientation: Orientation) -> () {
        assert_eq!(x.len(), y.len());
        match orientation {
            Orientation::Horizontal => {
                if x.len() == 1 {
                    // Leaf

                } else {
                    // Node
                }
            }
            Orientation::Vertical => {
            }
        };

    }
}

#[derive(Clone, Debug)]
pub enum Orientation {
    // y coord is key
    Horizontal,
    // x coord ist key
    Vertical,
}

#[derive(Clone, Debug)]
struct Key {
    orientation: Orientation,
    value: f32,
}

#[derive(Clone, Debug)]
enum Node {
    Knot {
        key: Key,
//        parent: Option<Box<Node>>,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
    Leaf {
        orientation: Orientation,
//        parent: Box<&Node>,
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