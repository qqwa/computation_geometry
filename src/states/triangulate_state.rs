use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use super::*;

#[derive(Clone)]
pub struct TriangulateState {
    points: Vec<Point2>,
    point_color: graphics::Color,
    triangles: Vec<[Point2; 3]>,
    triangle_color: graphics::Color,
    dirty_flag: bool,
    close: bool,
}

impl TriangulateState {
    pub fn new() -> Self {
        let point_color = graphics::Color::from_rgb(255, 255, 255);
        let triangle_color = graphics::Color::from_rgb(255, 255, 0);
        let mut points = Vec::new();
        // points.push(Point2::new(227.0, 250.0));
        // points.push(Point2::new(370.0, 163.0));
        // points.push(Point2::new(427.0, 362.0));
        // points.push(Point2::new(263.0, 403.0));
        // points.push(Point2::new(533.0, 254.0));
        // points.push(Point2::new(253.0, 162.0));
        // points.push(Point2::new(154.0, 356.0));
        // points.push(Point2::new(311.0, 463.0));
        // points.push(Point2::new(502.0, 450.0));
        // points.push(Point2::new(555.0, 454.0));
        // points.push(Point2::new(570.0, 476.0));
        // points.push(Point2::new(574.0, 500.0));
        // points.push(Point2::new(572.0, 527.0));
        // points.push(Point2::new(554.0, 552.0));
        // points.push(Point2::new(519.0, 565.0));
        // points.push(Point2::new(475.0, 576.0));
        // points.push(Point2::new(420.0, 573.0));
        // points.push(Point2::new(358.0, 565.0));
        // points.push(Point2::new(265.0, 558.0));
        // points.push(Point2::new(212.0, 544.0));
        // points.push(Point2::new(167.0, 522.0));
        // points.push(Point2::new(142.0, 489.0));
        // points.push(Point2::new(104.0, 458.0));
        // points.push(Point2::new( 70.0, 414.0));
        TriangulateState {
            points,
            point_color,
            triangles: Vec::new(),
            triangle_color,
            dirty_flag: true,
            close: false,
        }
    }
}

impl Scene<SharedState, Event> for TriangulateState {
    fn update(&mut self, _state: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        if self.dirty_flag {
            self.dirty_flag = false;
            self.triangles = crate::triangulation::delaunay(&self.points);
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

        graphics::set_color(ctx, self.triangle_color)?;
        for triangle in &self.triangles {
            graphics::polygon(ctx, DrawMode::Line(1.0), &triangle[..])?;
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
