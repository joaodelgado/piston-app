extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{Size, WindowSettings};

pub mod prelude {
    pub use super::{App, AppBuilder, Controller};
    pub use opengl_graphics::GlGraphics;
    pub use piston::input::{RenderArgs, UpdateArgs};
}

pub trait Controller {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics);
    fn tick(&mut self, args: &UpdateArgs);
}

pub struct App<S: Controller> {
    state: S,
    window: Window,
    gl: GlGraphics,
    event_settings: EventSettings,
}

impl<S: Controller> App<S> {
    fn new(builder: AppBuilder<S>) -> Result<App<S>, String> {
        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new(
            builder.title.unwrap_or("Piston app".to_string()),
            builder.size,
        ).opengl(opengl)
            .srgb(builder.srgb)
            .build()?;

        let gl = GlGraphics::new(opengl);
        let mut event_settings = EventSettings::new();

        if let Some(ups) = builder.ups {
            event_settings.ups = ups;
        }
        if let Some(fps) = builder.fps {
            event_settings.max_fps = fps;
        }

        Ok(App {
            state: builder.state,
            window,
            gl,
            event_settings,
        })
    }

    pub fn run(&mut self) {
        let mut events = Events::new(self.event_settings);
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.state.render(&args, &mut self.gl);
            }

            if let Some(args) = e.update_args() {
                self.state.tick(&args);
            }
        }
    }
}

pub struct AppBuilder<S: Controller> {
    state: S,
    size: Size,
    title: Option<String>,
    ups: Option<u64>,
    fps: Option<u64>,
    srgb: bool,
}

impl<S: Controller> AppBuilder<S> {
    pub fn new<T: Into<Size>>(state: S, size: T) -> Self {
        AppBuilder {
            state,
            size: size.into(),
            title: None,
            ups: None,
            fps: None,
            srgb: true,
        }
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn ups(mut self, ups: u64) -> Self {
        self.ups = Some(ups);
        self
    }

    pub fn fps(mut self, fps: u64) -> Self {
        self.fps = Some(fps);
        self
    }

    pub fn srgb(mut self, srgb: bool) -> Self {
        self.srgb = srgb;
        self
    }

    pub fn build(self) -> Result<App<S>, String> {
        App::new(self)
    }
}
