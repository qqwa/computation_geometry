use ggez::event::MouseButton;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use crate::convex_hull::{grahams_scan, jarvis_march};

pub struct PointState {
    points: Vec<Point2>,
    polygon: Vec<Point2>,
    point_color: graphics::Color,
    poly_color: graphics::Color,
    dirty_flag: bool,
}

impl PointState {
    pub fn new(_ctx: &mut Context) -> GameResult<PointState> {
        let point_color = graphics::Color::from_rgb(255, 255, 255);
        let poly_color = graphics::Color::from_rgb(200, 50, 50);
        let mut s = PointState {
            points: Vec::new(),
            polygon: Vec::new(),
            point_color,
            poly_color,
            dirty_flag: false,
        };
        Ok(s)
    }
}

impl event::EventHandler for PointState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // // test points with multiple points that have the same x value
        // if self.points.len() == 0 {
        //     self.points.extend(everything_is_convex());
        //     self.dirty_flag = true;
        // }

        if self.dirty_flag {
            self.dirty_flag = false;
            // self.polygon = grahams_scan(&self.points);
            self.polygon = jarvis_march(&self.points);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, self.point_color)?;
        for point in &self.points {
            graphics::circle(ctx, DrawMode::Fill, point.clone(), 2.5, 0.15)?;
        }

        graphics::set_color(ctx, self.poly_color)?;
        graphics::polygon(ctx, DrawMode::Line(2.0), &self.polygon[..])?;

        graphics::present(ctx);
        Ok(())
    }

    //fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {}

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        if button != MouseButton::Left {
            return;
        }
        let point = Point2::new(x as f32, y as f32);
        if !self.points.contains(&point) {
            debug!("Created Point: {}", point);
            self.points.push(point);
            self.dirty_flag = true;
        } else {
            debug!("Removed Point: {}", point);
            self.points.remove_item(&point);
            self.dirty_flag = true;
        }
    }
}
