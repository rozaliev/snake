use ecs;
use ecs::{Component,Phantom, Entity, World, Components};
use ecs::system::{BulkEntityProcess,BulkEntitySystem};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

use opengl_graphics::Gl;

pub struct SquareRendererProcess{
    gl: Gl
}

impl SquareRendererProcess {
    pub fn new_to_system() -> Box<BulkEntitySystem> {
        let sqr = box SquareRendererProcess{gl: Gl::new()};

        let only_square = ecs::Aspect::for_all(vec![Component::cid(Phantom::<Square>)]);
        let bulk = box BulkEntitySystem::new(sqr,only_square);

        return bulk
    }

    fn render(&mut self, sq: &Square, context: &Context) {
        let (r,g,b,a) = sq.rgba;
        context
            .trans(sq.x,sq.y)
            .rect(0.0, 0.0, sq.width, sq.height)
            .rgba(r,g,b,a)
            .draw(&mut self.gl);
    }
}


impl BulkEntityProcess for SquareRendererProcess {
    fn process(&mut self, es: Vec<&Entity>, _: &World, cs:&mut Components){}

    fn render(&mut self, es: Vec<&Entity>, _: &World, cs:&mut Components){
        let context = &Context::abs(800.0, 800.0);
        context.rgba(0.0,0.0,0.0,0.0).draw(&mut self.gl);

        for e in es.iter() {
            match cs.borrow::<Square>(*e) {
                None => {},
                Some(c) => {
                    if c.hidden { continue }
                    self.render(c,context)
                }
            }
        }
    }
}

component!(
    ID_Squre: Square {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rgba: (f32,f32,f32,f32),
        hidden: bool
    }
)