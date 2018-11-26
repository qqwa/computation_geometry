use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use super::*;
use crate::intersection;

pub struct LineState {
    lines: Vec<(Point2, Point2)>,
    intersection: Vec<(Point2, Point2)>,
    intersection_points: Vec<Point2>,
    point_of_line: Option<Point2>,
    color: graphics::Color,
    line_color: graphics::Color,
    intersection_color: graphics::Color,
    dirty_flag: bool,
    close: bool,
}

impl LineState {
    pub fn new() -> LineState {
        let color = graphics::Color::from_rgb(255, 255, 0);
        let line_color = graphics::Color::from_rgb(255, 255, 255);
        let intersection_color = graphics::Color::from_rgb(200, 50, 50);
        LineState {
            lines: Vec::new(),
            intersection: Vec::new(),
            intersection_points: Vec::new(),
            point_of_line: None,
            color,
            line_color,
            intersection_color,
            dirty_flag: false,
            close: false,
        }
    }
}

impl Scene<SharedState, Event> for LineState {
    fn update(&mut self, _state: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        if self.dirty_flag {
            self.dirty_flag = false;
            let (a, b) = intersection::iso_scan_line(&self.lines[..]);
            self.intersection = a;
            self.intersection_points = b;
        }

        if self.close {
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }
    fn draw(&mut self, state: &mut SharedState, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx);

        // draw point of unfinished line
        graphics::set_color(ctx, self.color)?;
        if let Some(point) = self.point_of_line {
            graphics::circle(ctx, DrawMode::Fill, point, 2.5, 0.15)?;
        }

        // draw lines
        graphics::set_color(ctx, self.line_color)?;
        for (p1, p2) in &self.lines {
            graphics::circle(ctx, DrawMode::Fill, p1.clone(), 2.5, 0.15)?;
            graphics::circle(ctx, DrawMode::Fill, p2.clone(), 2.5, 0.15)?;
            graphics::line(ctx, &vec![*p1, *p2][..], 1.0)?;
        }

        // redraw lines in different color that intersect
        graphics::set_color(ctx, self.intersection_color)?;
        for (p1, p2) in &self.intersection {
            graphics::circle(ctx, DrawMode::Fill, p1.clone(), 2.5, 0.15)?;
            graphics::circle(ctx, DrawMode::Fill, p2.clone(), 2.5, 0.15)?;
            graphics::line(ctx, &vec![*p1, *p2][..], 1.0)?;
        }

        // draw intersections points
        graphics::set_color(ctx, self.color)?;
        for p in &self.intersection_points {
            graphics::circle(ctx, DrawMode::Fill, p.clone(), 2.5, 0.15)?;
        }

        graphics::present(ctx);
        Ok(())
    }
    fn input(&mut self, state: &mut SharedState, event: Event, started: bool) {
        if let Event::LeftMouseButton { x, y } = event {
            let point = Point2::new(x as f32, y as f32);

            if self.point_of_line.is_none() {
                self.point_of_line = Some(point);
            } else {
                let cur_point = self.point_of_line.unwrap();
                let mut x_distance = cur_point.x - point.x;
                let mut y_distance = cur_point.y - point.y;
                if x_distance < 0.0 {
                    x_distance *= -1.0;
                }
                if y_distance < 0.0 {
                    y_distance *= -1.0;
                }
                let point = if x_distance < y_distance {
                    // take y value of new point
                    Point2::new(cur_point.x, point.y)
                } else {
                    // take x value of new point
                    Point2::new(point.x, cur_point.y)
                };
                if cur_point.x < point.x {
                    self.lines.push((cur_point, point));
                } else {
                    self.lines.push((point, cur_point));
                }
                self.point_of_line = None;
                self.dirty_flag = true;
            }
        }

        if let Event::RightMouseButton = event {
            self.point_of_line = None;
        }

        if let Event::Esc = event {
            self.close = true;
        }
    }
    fn name(&self) -> &str {
        "iso scan line"
    }
    fn draw_previous(&self) -> bool {
        false
    }
}
