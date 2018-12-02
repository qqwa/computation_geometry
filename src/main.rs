#![feature(box_syntax)]
#![feature(vec_remove_item)]

extern crate chrono;
extern crate fern;
extern crate ggez;
#[macro_use]
extern crate log;

use ggez::graphics::Point2;
use ggez::*;

// mod point_state;
mod convex_hull;
mod intersection;
mod kd_tree;
mod math;
mod states;

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

    // let mut menu = box states::menu_state::MenuState::new();
    let mut menu = box states::menu_state::MenuState::quick_start(3);

    let state = &mut states::MainState::new(ctx, menu);
    event::run(ctx, state).unwrap();
}
