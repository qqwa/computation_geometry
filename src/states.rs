use ggez::event::{Keycode, Mod, MouseButton, MouseState};
use ggez::*;
use ggez_goodies::scene::*;

pub mod line_state;
pub mod menu_state;
pub mod point_state;
pub mod search_tree_state;

pub struct SharedState();

#[derive(PartialEq)]
pub enum Event {
    LeftMouseButton { x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
    RightMouseButton,
    ArrowDown,
    ArrowUp,
    Return,
    Esc,
    Mode,
    Ignore,
}

pub struct MainState {
    scenes: SceneStack<SharedState, Event>,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context, scene: Box<Scene<SharedState, Event>>) -> Self {
        let mut scenes = SceneStack::new(ctx, SharedState {});
        scenes.push(scene);
        MainState { scenes }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scenes.update();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scenes.draw(ctx);
        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        let event = match button {
            MouseButton::Left => Event::LeftMouseButton { x, y },
            MouseButton::Right => Event::RightMouseButton,
            _ => Event::Ignore,
        };
        if event != Event::Ignore {
            self.scenes.input(event, true);
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        let event = if !repeat {
            match keycode {
                Keycode::Up => Event::ArrowUp,
                Keycode::Down => Event::ArrowDown,
                Keycode::Return => Event::Return,
                Keycode::Escape => Event::Esc,
                Keycode::M => Event::Mode,
                _ => Event::Ignore,
            }
        } else {
            Event::Ignore
        };
        if event != Event::Ignore {
            self.scenes.input(event, true);
        }
    }
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        let event = Event::MouseMove { x, y };
        self.scenes.input(event, true);
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        if self.scenes.current().name() == "MenuState" {
            return false;
        } else {
            self.scenes.input(Event::Esc, true);
            return true;
        }
    }
}
