use ggez::event::MouseButton;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use super::*;

use crate::convex_hull::{grahams_scan, jarvis_march};

#[derive(Clone)]
pub struct PointState {
    points: Vec<Point2>,
    polygon: Vec<Point2>,
    point_color: graphics::Color,
    poly_color: graphics::Color,
    dirty_flag: bool,
    close: bool,
    get_convex_hull: fn(&[Point2]) -> Vec<Point2>,
    name: String,
}

impl PointState {
    pub fn new(name: &str, get_convex_hull: fn(&[Point2]) -> Vec<Point2>) -> Self {
        let point_color = graphics::Color::from_rgb(255, 255, 255);
        let poly_color = graphics::Color::from_rgb(200, 50, 50);
        PointState {
            points: Vec::new(),
            polygon: Vec::new(),
            point_color,
            poly_color,
            dirty_flag: false,
            close: false,
            get_convex_hull,
            name: name.to_string(),
        }
    }
}

impl Scene<SharedState, Event> for PointState {
    fn update(&mut self, _state: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        if self.dirty_flag {
            self.dirty_flag = false;
            self.polygon = (self.get_convex_hull)(&self.points);
        }
        if self.close {
            debug!("popped");
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }
    fn draw(&mut self, state: &mut SharedState, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
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
    fn input(&mut self, state: &mut SharedState, event: Event, started: bool) {
        if let Event::LeftMouseButton { x, y } = event {
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
        if let Event::Esc = event {
            self.close = true;
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn draw_previous(&self) -> bool {
        false
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
            // self.polygon = jarvis_march(&self.points);
            self.polygon = (self.get_convex_hull)(&self.points);
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
