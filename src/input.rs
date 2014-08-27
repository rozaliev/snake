use std::any::{Any, AnyRefExt};

use ecs;
use ecs::{Component,Phantom, Entity, World, Components};
use ecs::system::{BulkEntityProcess,BulkEntitySystem};

use piston::{Input,GameEvent};
use piston::input;
use piston::input::keyboard;

use positioner::{CPosition};

use snake::{Left,Right,Up,Down};
use snake::{Direction,SnakeDirection};
use snake;


pub struct InputProcess;

component!(
    ID_Input: PlayerInput {
      next: Direction
    }
)

impl InputProcess {
    pub fn new_to_system() -> Box<BulkEntitySystem> {
        let pos = box InputProcess;

        let conds = ecs::Aspect::for_all(vec![Component::cid(Phantom::<SnakeDirection>),Component::cid(Phantom::<PlayerInput>)]);
        let bulk = box BulkEntitySystem::new(pos,conds);

        return bulk
    }
}

impl InputProcess {
  fn key_press(&self,e: Entity,cs: &mut Components, key: keyboard::Key){
    let pi = cs.borrow::<PlayerInput>(&e).unwrap();
    match key {
      keyboard::Left => pi.next = Left,
      keyboard::Right => pi.next = Right,
      keyboard::Up => pi.next = Up,
      keyboard::Down => pi.next = Down,
      _ => {}
    }

  }

  fn set_input(&self,e: Entity,cs: &mut Components) {
    let mut next = cs.borrow::<PlayerInput>(&e).unwrap().next;
    let sd = cs.borrow::<SnakeDirection>(&e).unwrap();
    match next {
      Left => {
          if sd.direction == Right {
              next = Right;
          }
      },
      Right => {
          if sd.direction == Left {
              next = Left
          }
      },
      Up => {
          if sd.direction == Down {
              next = Down
          }
      },
      Down => {
          if sd.direction == Up {
              next = Up
          }
      }
    }
    sd.direction = next;
  }
}

impl BulkEntityProcess for InputProcess {
    fn process(&mut self, es: Vec<&Entity>, _: &World, cs:&mut Components){
      for e in es.iter() {
        self.set_input(**e,cs)
      }
    }

    fn input(&mut self, es: Vec<&Entity>,  _: &World, cs: &mut Components, input: &Any){
      for e in es.iter() {
        match input.downcast_ref::<GameEvent>(){
          Some(ge) => {
            match *ge {
              Input(input::KeyPress{key,..}) => {
                self.key_press(**e,cs,key)
              },
              _ => {}
            }
          },
          None => {}
        }
      }

    }
}