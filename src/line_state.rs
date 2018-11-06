use ggez::event::MouseButton;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use crate::intersection;

pub struct LineState {
    lines: Vec<(Point2, Point2)>,
    intersection: Vec<(Point2, Point2)>,
    point_of_line: Option<Point2>,
    color: graphics::Color,
    line_color: graphics::Color,
    intersection_color: graphics::Color,
    dirty_flag: bool,
}

impl LineState {
    pub fn new(_ctx: &mut Context) -> GameResult<LineState> {
        let color = graphics::Color::from_rgb(255, 255, 0);
        let line_color = graphics::Color::from_rgb(255, 255, 255);
        let intersection_color = graphics::Color::from_rgb(200, 50, 50);
        let s = LineState {
            lines: Vec::new(),
            intersection: Vec::new(),
            point_of_line: None,
            color,
            line_color,
            intersection_color,
            dirty_flag: false,
        };
        Ok(s)
    }
}

impl event::EventHandler for LineState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        if self.dirty_flag {
            self.dirty_flag = false;
            // TODO: Do line scan
            self.intersection = intersection::iso_scan_line(&self.lines[..]);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, self.color)?;
        if let Some(point) = self.point_of_line {
            graphics::circle(ctx, DrawMode::Fill, point, 2.5, 0.15)?;
        }

        graphics::set_color(ctx, self.line_color)?;
        for (p1, p2) in &self.lines {
            graphics::circle(ctx, DrawMode::Fill, p1.clone(), 2.5, 0.15)?;
            graphics::circle(ctx, DrawMode::Fill, p2.clone(), 2.5, 0.15)?;
            graphics::line(ctx, &vec![*p1, *p2][..], 1.0)?;
        }
        
        graphics::set_color(ctx, self.intersection_color)?;
        for (p1, p2) in &self.intersection {
            graphics::circle(ctx, DrawMode::Fill, p1.clone(), 2.5, 0.15)?;
            graphics::circle(ctx, DrawMode::Fill, p2.clone(), 2.5, 0.15)?;
            graphics::line(ctx, &vec![*p1, *p2][..], 1.0)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        if button != MouseButton::Left {
            if button == MouseButton::Right {
                self.point_of_line = None;
            }
            return;
        }

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
}
