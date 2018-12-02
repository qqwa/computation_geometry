use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use crate::kd_tree;

use super::*;

#[derive(Clone)]
pub struct SearchTreeState {
    points: Vec<Point2>,
    query_points: Vec<Point2>,
    query: (Option<Point2>, Option<Point2>),
    point_color: graphics::Color,
    query_color: graphics::Color,
    dirty_flag_tree: bool,
    dirty_flag_search: bool,
    point_mode: bool,
    query_started: bool,
    close: bool,
    tree: Option<kd_tree::KdTree>,
    name: String,
}

impl SearchTreeState {
    pub fn new(name: &str) -> Self {
        let point_color = graphics::Color::from_rgb(255, 255, 255);
        let query_color = graphics::Color::from_rgb(50, 50, 250);
        SearchTreeState {
            points: Vec::new(),
            query_points: Vec::new(),
            query: (None, None),
            point_color,
            query_color,
            dirty_flag_tree: false,
            dirty_flag_search: false,
            point_mode: true,
            query_started: false,
            close: false,
            tree: None,
            name: name.to_string(),
        }
    }
}

impl Scene<SharedState, Event> for SearchTreeState {
    fn update(&mut self, _state: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        // recalc tree
        if self.dirty_flag_tree {
            self.dirty_flag_tree = false;
            let points: Vec<(f32, f32)> = self.points.iter().map(|x| (x[0], x[1])).collect();
            self.tree = Some(kd_tree::KdTree::new(&points[..]));
            println!("{:#?}", self.tree.take().unwrap());
        }
        // recalc search result
        // TODO...

        if self.close {
            debug!("popped");
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }
    fn draw(&mut self, state: &mut SharedState, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx);

        let font = graphics::Font::default_font().unwrap();

        let color_text = graphics::Color::from_rgb(255, 255, 0);
        graphics::set_color(ctx, color_text)?;
        let text_str = if self.point_mode {
            "point mode"
        } else {
            "query mode"
        };

        let text = graphics::Text::new(ctx, "press m to change mode", &font)?;
        graphics::draw(ctx, &text, graphics::Point2::new(10.0, 10.0), 0.0)?;

        let text = graphics::Text::new(ctx, text_str, &font)?;
        graphics::draw(ctx, &text, graphics::Point2::new(10.0, 30.0), 0.0)?;

        graphics::set_color(ctx, self.point_color)?;
        for point in &self.points {
            graphics::circle(ctx, DrawMode::Fill, point.clone(), 2.5, 0.15)?;
        }

        graphics::set_color(ctx, self.query_color)?;
        for point in &self.query_points {
            graphics::circle(ctx, DrawMode::Fill, point.clone(), 2.5, 0.15)?;
        }

        graphics::set_color(ctx, self.query_color)?;
        if let (Some(p1), Some(p2)) = self.query {
            let rect = graphics::Rect::new(p1.x, p1.y, p2.x - p1.x, p2.y - p1.y);
            graphics::rectangle(ctx, DrawMode::Line(1.0), rect)?;
        }

        graphics::present(ctx);
        Ok(())
    }
    fn input(&mut self, state: &mut SharedState, event: Event, started: bool) {
        if self.point_mode {
            if let Event::LeftMouseButton { x, y } = event {
                let point = Point2::new(x as f32, y as f32);
                self.dirty_flag_tree = true;
                if !self.points.contains(&point) {
                    debug!("Created Point: {}", point);
                    self.points.push(point);
                } else {
                    debug!("Removed Point: {}", point);
                    self.points.remove_item(&point);
                }
            }
        } else {
            if let Event::LeftMouseButton { x, y } = event {
                let point = Point2::new(x as f32, y as f32);
                self.dirty_flag_search = true;
                if self.query_started {
                    self.query.1 = Some(point);
                    self.query_started = false;
                } else {
                    self.query = (Some(point), None);
                    self.query_started = true;
                }
            }
            if let Event::RightMouseButton { x, y } = event {
                let point = Point2::new(x as f32, y as f32);
                self.dirty_flag_search = true;
                self.query = (None, None);
                self.query_started = false;
            }
            if let Event::MouseMove { x, y } = event {
                let point = Point2::new(x as f32, y as f32);
                self.dirty_flag_search = true;
                if self.query_started {
                    self.query.1 = Some(point);
                    debug!("Mouse move: {} {}", x, y);
                }
            }
        }
        if let Event::Mode = event {
            self.point_mode = !self.point_mode;
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
