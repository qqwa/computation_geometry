use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use super::*;

#[derive(Clone)]
pub struct TriangulateState {
    points: Vec<Point2>,
    point_color: graphics::Color,
    dirty_flag: bool,
    close: bool,
}

impl TriangulateState {
    pub fn new() -> Self {
        let point_color = graphics::Color::from_rgb(255, 255, 255);
        TriangulateState {
            points: Vec::new(),
            point_color,
            dirty_flag: false,
            close: false,
        }
    }
}

impl Scene<SharedState, Event> for TriangulateState {
    fn update(&mut self, _state: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        if self.dirty_flag {
            self.dirty_flag = false;
            // TODO: call algorithm
        }
        if self.close {
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }
    fn draw(&mut self, _state: &mut SharedState, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, self.point_color)?;
        for point in &self.points {
            graphics::circle(ctx, DrawMode::Fill, point.clone(), 2.5, 0.15)?;
        }

        graphics::present(ctx);
        Ok(())
    }
    fn input(&mut self, _state: &mut SharedState, event: Event, _started: bool) {
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
        "triangulate"
    }
    fn draw_previous(&self) -> bool {
        false
    }
}
