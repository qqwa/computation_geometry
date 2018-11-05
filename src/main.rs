#![feature(box_syntax)]
#![feature(vec_remove_item)]

extern crate chrono;
extern crate fern;
extern crate ggez;
#[macro_use]
extern crate log;

use ggez::event::MouseButton;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;

mod math;

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
        let mut s = MainState {
            points: Vec::new(),
            polygon: Vec::new(),
            point_color,
            poly_color,
            dirty_flag: false,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // // test points with multiple points that have the same x value
        // if self.points.len() == 0 {
        //     self.points.extend(everything_is_convex());
        //     self.dirty_flag = true;
        // }

        if self.dirty_flag {
            self.dirty_flag = false;
            // self.polygon = math::grahams_scan(&self.points);
            self.polygon = math::jarvis_march(&self.points);
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

fn everything_is_convex() -> Vec<Point2> {
    vec![
        Point2::new(200.0, 100.0),
        Point2::new(250.0, 100.0),
        Point2::new(300.0, 150.0),
        Point2::new(250.0, 160.0),
        Point2::new(250.0, 170.0),
        Point2::new(250.0, 180.0),
        Point2::new(250.0, 190.0),
        Point2::new(250.0, 120.0),
    ]
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

