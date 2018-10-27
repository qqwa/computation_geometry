#![feature(box_syntax)]
#![feature(vec_remove_item)]

extern crate ggez;
extern crate fern;
extern crate chrono;
#[macro_use]
extern crate log;

use ggez::*;
use ggez::graphics::{DrawMode, Point2};
use ggez::event::MouseButton;

struct MainState {
    points: Vec<Point2>,
    polygon: Vec<Point2>,
    point_color: graphics::Color,
    poly_color: graphics::Color,
    dirty_flag: bool,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let point_color = graphics::Color::from_rgb(255, 255, 255);
        let poly_color = graphics::Color::from_rgb(200, 50, 50);
        let mut s = MainState {points: Vec::new(), polygon: Vec::new(), point_color, poly_color, dirty_flag: false };
        Ok(s)
    }
}

fn left_turn(points: &[Point2]) -> bool {
    if points.len() != 3 {
        panic!("tried to calculate left turn for {} points instead of 3", points.len());
    }
    let a = Point2::new(points[1][0]-points[0][0], points[1][1]-points[0][1]);
    let b = Point2::new(points[2][0]-points[0][0], points[2][1]-points[0][1]);

    let left = (a[0]*b[1] - b[0]*a[1]) < 0.0;
    left
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.dirty_flag {
            self.dirty_flag = false;
            debug!("Recomputed convex hull:");
            self.points.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
            debug!("Left point: {}", self.points[0]);
            debug!("Right pont: {}", self.points[self.points.len()-1]);

            if self.points.len() < 3 {
                return Ok(())
            }

            let mut upper = Vec::new();
            upper.extend_from_slice(&self.points[..2]);

            for point in &self.points[2..] {
                upper.push(point.clone());
                while 2 < upper.len() && left_turn(&upper[upper.len()-3..]) {
                    upper.remove(upper.len()-2);
                }
            }

            let mut lower: Vec<Point2> = Vec::new();
            lower.extend_from_slice(&self.points[self.points.len()-2..]);
            lower.reverse();
            for point in self.points[..self.points.len()-2].iter().rev() {
                lower.push(point.clone());
                while 2 < lower.len() && left_turn(&lower[lower.len()-3..]) {
                    lower.remove(lower.len()-2);
                }
            }
            lower.remove(0);
            lower.pop();
            // debug!("{:#?}", lower);
            // debug!("lower for first value: {:?}, len: {}", self.points[1..self.points.len()-1].iter().rev().nth(0), self.points[1..self.points.len()-2].len());

            self.polygon.truncate(0);
            self.polygon.append(&mut upper);
            self.polygon.append(&mut lower);

        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, self.point_color)?;
        for point in &self.points {
            graphics::circle(ctx, DrawMode::Fill, point.clone(), 2.5, 0.15)?;
        }

        let mut polygon: Vec<Point2> = Vec::new();
        for point in &self.points {
            if point[0] < 100.0 {
                polygon.push(point.clone())
            }
        }

        graphics::set_color(ctx, self.poly_color)?;
        graphics::polygon(ctx, DrawMode::Line(2.0), &self.polygon[..])?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    }

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

        // self.polygon.truncate(0);
        // for point in self.points.iter().step_by(2) {
        //     self.polygon.push(point.clone());
        // }

    }
}


fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{:<5}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level().to_string(),
                record.target(),
                message
            ))
        })
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .level_for("ggez", log::LevelFilter::Warn)
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
