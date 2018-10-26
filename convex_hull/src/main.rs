#![feature(box_syntax)]

extern crate ggez;

use ggez::*;
use ggez::graphics::{DrawMode, Point2};

struct MainState {
    points: Vec<Point2>,
    point_color: graphics::Color,
    poly_color: graphics::Color,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let point_color = graphics::Color::from_rgb(255, 255, 255);
        let poly_color = graphics::Color::from_rgb(200, 50, 50);
        let mut s = MainState {points: Vec::new(), point_color, poly_color };
        s.points.push(Point2::new(100.0, 100.0));
        s.points.push(Point2::new(200.0, 150.0));
        s.points.push(Point2::new(100.0, 200.0));
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, self.point_color)?;
        for point in &self.points {
            graphics::circle(ctx, DrawMode::Fill, point.clone(), 2.5, 0.15)?;
        }
        graphics::set_color(ctx, self.poly_color)?;
        graphics::polygon(ctx, DrawMode::Line(2.0), &self.points[..])?;

        graphics::present(ctx);
        Ok(())
    }
}


fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
