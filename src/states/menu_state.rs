// use ggez_goodies::scene::*;

use ggez::graphics;

use super::*;
use crate::convex_hull;

pub struct MenuState {
    switch: bool,
    scenes: Vec<String>,
    selected: usize,
    font: graphics::Font,
    color: graphics::Color,
    color_selected: graphics::Color,
    quick_start: bool,
}

impl MenuState {
    pub fn new() -> Self {
        let color = graphics::Color::from_rgb(255, 255, 255);
        let color_selected = graphics::Color::from_rgb(255, 255, 0);
        let scenes = vec![
            "graham's scan".to_string(),
            "jarvi's march".to_string(),
            "iso scan line".to_string(),
            "2d-tree".to_string(),
            "triangulate".to_string(),
        ];
        MenuState {
            switch: false,
            scenes,
            selected: 0,
            font: graphics::Font::default_font().unwrap(),
            color,
            color_selected,
            quick_start: false,
        }
    }

    pub fn quick_start(scene: usize) -> Self {
        MenuState {
            selected: scene,
            quick_start: true,
            ..Self::new()
        }
    }
}

impl Scene<SharedState, Event> for MenuState {
    fn update(&mut self, _state: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        if self.switch || self.quick_start {
            self.switch = false;
            self.quick_start = false;
            debug!("Switch to {}", self.scenes[self.selected]);
            match self.scenes[self.selected].as_str() {
                "graham's scan" => SceneSwitch::Push(box super::point_state::PointState::new(
                    "graham's scan",
                    convex_hull::grahams_scan,
                )),
                "jarvi's march" => SceneSwitch::Push(box super::point_state::PointState::new(
                    "graham's scan",
                    convex_hull::jarvis_march,
                )),
                "iso scan line" => SceneSwitch::Push(box super::line_state::LineState::new()),
                "2d-tree" => SceneSwitch::Push(box super::search_tree_state::SearchTreeState::new(
                    "2d-tree",
                )),
                "triangulate" => SceneSwitch::Push(box super::triangulate_state::TriangulateState::new()),
                _ => SceneSwitch::None,
            }
        } else {
            SceneSwitch::None
        }
    }
    fn draw(&mut self, _state: &mut SharedState, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx);

        for (i, scene) in self.scenes.iter().enumerate() {
            if i == self.selected {
                graphics::set_color(ctx, self.color_selected)?;
            } else {
                graphics::set_color(ctx, self.color)?;
            }
            let text = graphics::Text::new(ctx, scene, &self.font)?;
            graphics::draw(
                ctx,
                &text,
                graphics::Point2::new(
                    ctx.conf.window_mode.width as f32 / 2.0 - text.width() as f32 / 2.0,
                    100.0 + i as f32 * 50.0,
                ),
                0.0,
            )?;
        }

        graphics::present(ctx);
        Ok(())
    }
    fn input(&mut self, _state: &mut SharedState, event: Event, _started: bool) {
        match event {
            Event::Return => self.switch = true,
            Event::ArrowUp => {
                if 0 == self.selected {
                    self.selected = self.scenes.len() - 1;
                } else {
                    self.selected -= 1;
                }
            }
            Event::ArrowDown => {
                if self.selected + 1 == self.scenes.len() {
                    self.selected = 0;
                } else {
                    self.selected += 1;
                }
            }
            _ => {}
        }
    }
    fn name(&self) -> &str {
        "MenuState"
    }
    fn draw_previous(&self) -> bool {
        false
    }
}

// impl Scene for other game states here
