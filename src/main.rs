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

use piston::input::keyboard;
use piston::input::{KeyPress};

use ecs::{World};

use square::{SquareRendererProcess,Square};
use snake::{Head, Tail, SnakeMoverProcess,SnakeDirection, Target};
use positioner::{CPosition,PositionerProcess};
use input::{InputProcess,PlayerInput};
use snake::{Left,Right,Up,Down};
use collision::CollisionProcess;

mod square;
mod snake;
mod positioner;
mod input;
mod collision;


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
    world.register_system(CollisionProcess::new_to_system());

    world.register_system(SquareRendererProcess::new_to_system());

    world.register_component::<Square>();
    world.register_component::<Head>();
    world.register_component::<Tail>();
    world.register_component::<CPosition>();
    world.register_component::<SnakeDirection>();
    world.register_component::<PlayerInput>();
    world.register_component::<Target>();
    world.finalise();

    let snake = world.create_entity();
    let apple = world.create_entity();

    world.add_component::<Head>(&snake, Head);
    world.add_component::<Tail>(&snake, Tail{next: None});
    world.add_component::<CPosition>(&snake, CPosition{x:0, y:0});
    world.add_component::<Square>(&snake, Square{
        x:0.0,
        y:0.0,
        width: 32.0,
        height: 32.0,
        rgba:(1.0,1.0,1.0,1.0),
        hidden: false
    });
    world.add_component::<SnakeDirection>(&snake, SnakeDirection{direction: Right});
    world.add_component::<PlayerInput>(&snake, PlayerInput{next: Right});
    world.add_component::<Target>(&snake, Target{t: Some(apple)});

    world.activate_entity(&snake);


    world.add_component::<CPosition>(&apple, CPosition{x:15, y:15});
    world.add_component::<Square>(&apple, Square{
        x:0.0,
        y:0.0,
        width: 32.0,
        height: 32.0,
        rgba:(0.0,1.0,1.0,1.0),
        hidden: false
    });

    world.activate_entity(&apple);

    let game_iter_settings = piston::GameIteratorSettings {
        updates_per_second: 15,
        max_frames_per_second: 15
    };
    let mut paused = false;
    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(_) =>{
                world.render()
            },
            Update(_) =>{
                if paused { continue }
                world.update();
            },
            Input(input) => {
                match input {
                    KeyPress{key,..} => {
                        if key == keyboard::P {
                            paused = !paused;
                        }
                        if paused { continue }
                        world.input(&Input(input));
                    },
                    _ => {
                        if paused { continue }
                        world.input(&Input(input))
                    }
                }
            }
        }
    }
}