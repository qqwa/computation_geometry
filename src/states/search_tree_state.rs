use ggez::graphics::{DrawMode, Point2};
use ggez::*;

use crate::kd_tree;

use super::*;

#[derive(Clone)]
pub struct SearchTreeState {
    points: Vec<Point2>,
    query_points: Vec<(f32, f32)>,
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
        let points = if self.dirty_flag_search || self.dirty_flag_tree {
            self.points.iter().map(|x| (x[0], x[1])).collect()
        } else {
            Vec::new()
        };

        // recalc tree
        if self.dirty_flag_tree {
            self.dirty_flag_tree = false;

            self.tree = Some(kd_tree::KdTree::new(&points[..]));
        }
        // recalc search result
        if self.dirty_flag_search {
            self.dirty_flag_search = false;
            if let (Some(t1), Some(t2)) = self.query {
                let p1 = { (t1.x.min(t2.x), t1.y.min(t2.y)) };
                let p2 = { (t1.x.max(t2.x), t1.y.max(t2.y)) };
                if let Some(tree) = &self.tree {
                    self.query_points = tree.range_query(p1, p2);
                }
            } else {
                // clear queried points
                self.query_points = Vec::new();
            }
        }

        if self.close {
            debug!("popped");
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }
    fn draw(&mut self, _state: &mut SharedState, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx);

        let font = graphics::Font::default_font().unwrap();

        let color_text = graphics::Color::from_rgb(255, 255, 0);
        graphics::set_color(ctx, color_text)?;
        let text_str = if self.point_mode {
            "point mode"
        } else {
            "query mode"
        };

        // draw tree partioning
        if let Some(tree) = &self.tree {
            draw_node(
                ctx,
                &*tree.0,
                0,
                ctx.conf.window_mode.width,
                0,
                ctx.conf.window_mode.height,
            )?;
        }

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
            let point = Point2::new(point.0, point.1);
            graphics::circle(ctx, DrawMode::Fill, point, 4.5, 0.15)?;
        }

        graphics::set_color(ctx, self.query_color)?;
        if let (Some(p1), Some(p2)) = self.query {
            let rect = graphics::Rect::new(p1.x, p1.y, p2.x - p1.x, p2.y - p1.y);
            graphics::rectangle(ctx, DrawMode::Line(2.0), rect)?;
        }

        graphics::present(ctx);
        Ok(())
    }
    fn input(&mut self, _state: &mut SharedState, event: Event, _started: bool) {
        if self.point_mode {
            if let Event::LeftMouseButton { x, y } = event {
                let point = Point2::new(x as f32, y as f32);
                self.dirty_flag_tree = true;
                self.dirty_flag_search = true;
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
            if let Event::RightMouseButton { .. } = event {
                self.dirty_flag_search = true;
                self.query = (None, None);
                self.query_started = false;
            }
            if let Event::MouseMove { x, y } = event {
                let point = Point2::new(x as f32, y as f32);
                if self.query_started {
                    self.dirty_flag_search = true;
                    self.query.1 = Some(point);
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

fn draw_node(
    ctx: &mut ggez::Context,
    node: &kd_tree::Node,
    x_off: u32,
    x_width: u32,
    y_off: u32,
    y_width: u32,
) -> ggez::GameResult<()> {
    match node {
        kd_tree::Node::Knot { key, left, right } => {
            if key.orientation == kd_tree::Orientation::Horizontal {
                let p1 = Point2::new(x_off as f32, key.value);
                let p2 = Point2::new((x_off + x_width) as f32, key.value);
                graphics::line(ctx, &vec![p1, p2][..], 1.0)?;

                let key_with_offset = key.value as u32 - y_off;

                if let Some(left) = left {
                    draw_node(
                        ctx,
                        left,
                        x_off,
                        x_width,
                        y_off,
                        y_width - (y_width - key_with_offset),
                    )?;
                }
                if let Some(right) = right {
                    draw_node(
                        ctx,
                        right,
                        x_off,
                        x_width,
                        key.value as u32,
                        y_width - key_with_offset,
                    )?;
                }
            } else {
                let p1 = Point2::new(key.value, y_off as f32);
                let p2 = Point2::new(key.value, (y_off + y_width) as f32);
                graphics::line(ctx, &vec![p1, p2][..], 1.0)?;

                let key_with_offset = key.value as u32 - x_off;

                if let Some(left) = left {
                    draw_node(
                        ctx,
                        left,
                        x_off,
                        x_width - (x_width - key_with_offset),
                        y_off,
                        y_width,
                    )?;
                }
                if let Some(right) = right {
                    draw_node(
                        ctx,
                        right,
                        key.value as u32,
                        x_width - key_with_offset,
                        y_off,
                        y_width,
                    )?;
                }
            }
        }
        _ => {}
    }

    Ok(())
}
