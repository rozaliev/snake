use opengl_graphics::Gl;

use piston::{
    RenderArgs,
    UpdateArgs
};

use piston::input;
use piston::input::keyboard;

use board::Board;

use graphics::{
    Context,
    AddColor,
    Draw
};


pub struct App {
    gl: Gl,
    board: Board,
    game_over: bool,
    pause: bool
}


impl App {
    pub fn new() -> App {
         return App{ gl: Gl::new(),board: Board::new(25,25), game_over: false, pause: false}
    }

    pub fn render(&mut self,args: &RenderArgs) {

        let context = &Context::abs(args.width as f64, args.height as f64);

        if self.game_over {
            context.rgba(1.0,0.0,0.0,0.0).draw(&mut self.gl);
        } else {
            context.rgba(0.0,0.0,0.0,0.0).draw(&mut self.gl);
        }

        self.board.render(context, &mut self.gl);
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        if self.pause || self.game_over {return}
        self.game_over = !self.board.update();
    }

    pub fn key_press(&mut self, key: input::keyboard::Key) {
        match key {
            keyboard::R => self.start(),
            keyboard::P => { self.pause = !self.pause },
            _ => {
                if self.pause || self.game_over {return}
                self.board.key_press(key);
            }
        }
    }

    fn start(&mut self) {
        self.pause = false;
        self.game_over = false;
        self.board = Board::new(25,25);
    }
}