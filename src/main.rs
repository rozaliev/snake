#![feature(macro_rules,phase)]

extern crate debug;

extern crate graphics;
extern crate piston;
extern crate glfw_game_window;
extern crate opengl_graphics;


#[phase(plugin)] extern crate lazy_static;
#[phase(plugin, link)] extern crate ecs;


use glfw_game_window::GameWindowGLFW as Window;

use piston::{
    Render,
    Update,
    Input
};



use ecs::{World};

use square::{SquareRendererProcess,Square};
use snake::{Head, Tail, SnakeMoverProcess,SnakeDirection};
use positioner::{CPosition,PositionerProcess};
use input::{InputProcess,PlayerInput};
use snake::{Left,Right,Up,Down};

mod square;
mod snake;
mod positioner;
mod input;


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

    let mut world = World::new();

    world.register_system(InputProcess::new_to_system());
    world.register_system(SnakeMoverProcess::new_to_system());
    world.register_system(PositionerProcess::new_to_system());
    world.register_system(SquareRendererProcess::new_to_system());

    world.register_component::<Square>();
    world.register_component::<Head>();
    world.register_component::<Tail>();
    world.register_component::<CPosition>();
    world.register_component::<SnakeDirection>();
    world.register_component::<PlayerInput>();
    world.finalise();

    let snake = world.create_entity();
    world.add_component::<Head>(&snake, Head);
    world.add_component::<Tail>(&snake, Tail{next: None});
    world.add_component::<CPosition>(&snake, CPosition{x:0, y:0});
    world.add_component::<Square>(&snake, Square{x:0.0, y:0.0, width: 32.0, height: 32.0});
    world.add_component::<SnakeDirection>(&snake, SnakeDirection{direction: Right});
    world.add_component::<PlayerInput>(&snake, PlayerInput{next: Right});

    world.activate_entity(&snake);


    let game_iter_settings = piston::GameIteratorSettings {
        updates_per_second: 15,
        max_frames_per_second: 15
    };

    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(_) =>{
                world.render()
            },
            Update(_) =>{
                world.update()
            },
            Input(input) => {
               world.input(&Input(input))
            }
        }
    }
}