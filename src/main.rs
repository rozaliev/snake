extern crate graphics;
extern crate piston;
extern crate glfw_game_window;
extern crate opengl_graphics;

use std::rand;

use glfw_game_window::GameWindowGLFW as Window;
use opengl_graphics::Gl;

use piston::{
    GameWindow,
    Render,
    RenderArgs,
    Update,
    UpdateArgs,
    Input
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

use piston::input;
use piston::input::keyboard;

#[deriving(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

pub struct Snake {
    state: Direction,
    next: Direction,
    doIncrease: bool,
    body: Vec<Block>

}

pub struct App {
    gl: Gl,
    board: Board,
    game_over: bool,
    pause: bool
}


fn rnd_in_range(range: uint) -> uint {
    return rand::random::<uint>() % range
}

impl Snake {
    fn new() -> Snake {
        return Snake{state:Right, next: Right, doIncrease: false, body: vec![Block::new(0,0)]}
    }

    fn update(&mut self) {
        let mut x = self.body[0].x;
        let mut y = self.body[0].y;

        match self.next {
            Left => {
                if self.state == Right {
                    self.next = Right;
                }
            },
            Right => {
                if self.state == Left {
                    self.next = Left
                }
            },
            Up => {
                if self.state == Down {
                    self.next = Down
                }
            },
            Down => {
                if self.state == Up {
                    self.next = Up
                }
            }
        }

        self.state = self.next;

        match self.state {
            Right => {
                x+=1
            },
            Left => {
                x-=1
            },
            Up => {
                y-=1
            },
            Down => {
                y+=1;
            }
        }
        self.body.insert(0,Block{x:x, y:y});

        if self.doIncrease {
            self.doIncrease = false;
        } else {
            self.body.pop();
        }
    }

    fn key_press(&mut self, key: keyboard::Key) {
        match key {
            keyboard::Left => {
                println!("pressed left");
                self.next = Left;
            },
            keyboard::Right => {
               println!("pressed right");
               self.next = Right ;
            },
            keyboard::Up => {
                println!("pressed up");
                self.next = Up;
            },
            keyboard::Down => {
                println!("pressed down");
                self.next = Down;
            },
            _ => {}
        }
    }

    fn increase(&mut self) {
        self.doIncrease = true;
    }

    fn check_collisions(&mut self) -> bool {
        let x = self.body[0].x;
        let y = self.body[0].y;

        for block in self.body.slice_from(1).iter() {
            if block.x == x && block.y == y {
                return true
            }
        }

        return false
    }
}


struct Block {
    x: uint,
    y: uint
}

struct Board {
    width: uint,
    height: uint,
    block: Block,
    snake: Snake
}


impl Block {
    fn new(x: uint,y: uint) -> Block {
        return Block{x:x, y:y}
    }

    fn new_rand(width: uint, height: uint) -> Block {
        let rx = rnd_in_range(width);
        let ry = rnd_in_range(height);
        return Block{x:rx,y:ry}
    }
}

impl Board {
    fn new(width: uint, height: uint) -> Board {
        return Board{width: width,height: width, block:Block::new_rand(width, height),snake: Snake::new()}
    }

    fn update(&mut self) -> bool {
        self.snake.update();
        if self.snake.body[0].x == self.block.x && self.snake.body[0].y == self.block.y {
            self.set_rand_block();
            self.snake.increase();
        }

        if self.check_collisions() || self.snake.check_collisions() {
            return false
        }
        return true
    }

    fn set_rand_block(&mut self) {
        loop {
            let rx = rnd_in_range(self.width);
            let ry = rnd_in_range(self.height);

            for block in self.snake.body.iter() {
                if block.x == rx && block.y == ry {
                    continue
                }
            }

            self.block.x = rx;
            self.block.y = ry;
            return
        }
    }

    fn check_collisions(&mut self) -> bool {
        self.snake.body[0].x > self.width-1 ||
           self.snake.body[0].y > self.height-1

    }

    fn render(&mut self, _: &RenderArgs, context: &Context, gl: &mut Gl) {
        for block in self.snake.body.iter() {
            context
                .trans((block.x * 32) as f64, (block.y*32) as f64)
                .rect(0.0, 0.0, 32.0, 32.0)
                .rgba(1.0, 1.0, 1.0,1.0)
                .draw(gl);
        }


        context
            .trans((self.block.x * 32) as f64, (self.block.y*32) as f64)
            .rect(0.0, 0.0, 32.0, 32.0)
            .rgba(1.0, 1.0, 1.0,1.0)
            .draw(gl);
    }
}



impl<W: GameWindow> App {
    fn render(&mut self, _: &mut W, args: &RenderArgs) {

        let context = &Context::abs(args.width as f64, args.height as f64);

        if self.game_over {
            context.rgba(1.0,0.0,0.0,0.0).draw(&mut self.gl);
        } else {
            context.rgba(0.0,0.0,0.0,0.0).draw(&mut self.gl);
        }

        self.board.render(args, context, &mut self.gl);
    }

    fn update(&mut self, _: &mut W, _: &UpdateArgs) {
        if self.pause || self.game_over {return}
        self.game_over = !self.board.update();
    }

    fn key_press(&mut self, w: &mut W, key: input::keyboard::Key) {
        match key {
            keyboard::R => self.start(w),
            keyboard::P => { self.pause = !self.pause },
            _ => {
                if self.pause || self.game_over {return}
                self.board.snake.key_press(key);
            }
        }
    }

    fn start(&mut self,_: &mut W) {
        self.pause = false;
        self.game_over = false;
        self.board = Board::new(25,25);
    }
}

fn main() {
    // Create a GLFW window.
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::GameWindowSettings {
            title: "Snake".to_string(),
            size: [800, 800],
            fullscreen: false,
            exit_on_esc: true
        }
    );


    let game_iter_settings = piston::GameIteratorSettings {
        updates_per_second: 15,
        max_frames_per_second: 60
    };


    let mut app = App { gl: Gl::new(),board: Board::new(25,25), game_over: false, pause: false};

    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(_args) =>{
                app.render(&mut window, &_args)
            },
            Update(_args) =>{
                app.update(&mut window, &_args)
            },
            Input(input::KeyPress{key,..}) => {
                app.key_press(&mut window, key);
            },
            _ => {},
        }
    }
}