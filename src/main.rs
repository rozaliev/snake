extern crate graphics;
extern crate piston;
extern crate glfw_game_window;
extern crate opengl_graphics;



use glfw_game_window::GameWindowGLFW as Window;

use piston::{
    Render,
    Update,
    Input
};

use piston::input;


mod app;
mod board;

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


    let mut app = app::App::new();

    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(_args) =>{
                app.render(&_args)
            },
            Update(_args) =>{
                app.update(&_args)
            },
            Input(input::KeyPress{key,..}) => {
                app.key_press(key);
            },
            _ => {},
        }
    }
}