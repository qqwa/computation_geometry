use ggez::graphics::Point2;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

#[derive(Debug)]
enum Event {
    Start { x: f32, id: usize },
    End { x: f32, id: usize },
    Vertical { x: f32, id: usize },
}

impl Event {
    pub fn from_line(line: (Point2, Point2), id: usize) -> Vec<Event> {
        if line_is_horizontal((line.0, line.1)) {
            let e1 = Event::Start {
                x: line.0.x.min(line.1.x),
                id,
            };
            let e2 = Event::End {
                x: line.0.x.max(line.1.x),
                id,
            };
            return vec![e1, e2];
        }
        vec![Event::Vertical { x: line.0.x, id }]
    }
}

impl Eq for Event {}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        other.partial_cmp(self).unwrap()
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        let x_self = match self {
            Event::Start { x, .. } => x,
            Event::End { x, .. } => x,
            Event::Vertical { x, .. } => x,
        };
        let x_other = match other {
            Event::Start { x, .. } => x,
            Event::End { x, .. } => x,
            Event::Vertical { x, .. } => x,
        };

        x_self == x_other
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        let x_self = match self {
            Event::Start { x, .. } => x,
            Event::End { x, .. } => x,
            Event::Vertical { x, .. } => x,
        };
        let x_other = match other {
            Event::Start { x, .. } => x,
            Event::End { x, .. } => x,
            Event::Vertical { x, .. } => x,
        };

        x_other.partial_cmp(x_self)
    }
}

#[derive(Debug)]
struct Segment {
    p1: Point2,
    p2: Point2,
    id: usize,
}

#[derive(Debug)]
struct Key(f32);

impl Eq for Key {}

impl Ord for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

pub fn iso_scan_line(lines: &[(Point2, Point2)]) -> (Vec<(Point2, Point2)>, Vec<Point2>) {
    debug!("iso_scan_line");
    let mut events = BinaryHeap::new();
    let mut intersecting_lines = BTreeMap::new();
    let mut intersection_points = Vec::new();

    for (id, line) in lines.iter().enumerate() {
        let es = Event::from_line((line.0, line.1), id);
        for event in es {
            events.push(event);
        }
    }

    let mut scan_line = BTreeMap::new();

    while let Some(event) = events.pop() {
        match &event {
            Event::Start { id, .. } => {
                let seg = Segment {
                    p1: lines[*id].0,
                    p2: lines[*id].1,
                    id: *id,
                };
                scan_line.insert(Key(seg.p1.y), seg);
            }
            Event::End { id, .. } => {
                let p = lines[*id].0;
                scan_line.remove(&Key(p.y));
            }
            Event::Vertical { id, .. } => {
                let p1 = lines[*id].0;
                let p2 = lines[*id].1;
                scan_line.get(&Key(p1.y));
                for segment in scan_line.range(Key(p1.y.min(p2.y))..Key(p1.y.max(p2.y))) {
                    intersection_points.push(Point2::new(p1.x, segment.1.p1.y));
                    intersecting_lines.insert(*id, (p1, p2));
                    intersecting_lines.insert(segment.1.id, (segment.1.p1, segment.1.p2));
                }
            }
        }
    }

    (
        intersecting_lines.values().cloned().collect(),
        intersection_points,
    )
}

fn line_is_horizontal(points: (Point2, Point2)) -> bool {
    if points.0.y == points.1.y {
        true
    } else {
        false
    }
}
