extern crate graphics;
extern crate piston_app;

use piston_app::prelude::*;

struct GameController {
    x: f64,
}

impl Controller for GameController {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::clear;
        use graphics::color::{BLACK, WHITE};
        use graphics::ellipse::{circle, Ellipse};

        clear(BLACK, gl);
        gl.draw(args.viewport(), |c, gl| {
            Ellipse::new_border(WHITE, 1.0).draw(
                circle(self.x, 150.0, 50.0),
                &c.draw_state,
                c.transform,
                gl,
            );
        });
    }
    fn tick(&mut self, args: &UpdateArgs) {
        self.x = (self.x + args.dt * 20.0) % 300.0;
    }
}

fn main() {
    let contoller = GameController { x: 150.0 };

    let mut app = AppBuilder::new(contoller, [300, 300])
        .title("Circle test")
        .build()
        .expect("Error creating app");

    app.run();
}
