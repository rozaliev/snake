use std::default::Default;

use ecs;
use ecs::{Component,Phantom, Entity, World, Components};
use ecs::system::{BulkEntityProcess,BulkEntitySystem};
use positioner::{CPosition};


component!(
    ID_Tail: Tail {
      next: Option<Entity>
    }
)

component!(
    ID_Head: Head
)

component!(
    ID_Target: Target {
      t: Option<Entity>
    }
)

component!(
    ID_SnakeDirection: SnakeDirection {
      direction: Direction
    }
)

#[deriving(PartialEq, Show, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Default for Direction {
  fn default() -> Direction {
    return Right
  }
}


pub struct SnakeMoverProcess;


impl SnakeMoverProcess {
    pub fn new_to_system() -> Box<BulkEntitySystem> {
        let smp = box SnakeMoverProcess;

        let only_head = ecs::Aspect::for_all(vec![Component::cid(Phantom::<Head>)]);
        let bulk = box BulkEntitySystem::new(smp,only_head);

        return bulk
    }

    fn move(&mut self, e: Entity, cs:&mut Components, x: u64, y: u64){
      let mut next: Option<Entity> = None;
      {
        let tail = cs.borrow::<Tail>(&e);

        match tail {
          None => { return },
          Some(ntail) => {

            match ntail.next {
              None => { return },
              Some(n) => { next = Some(n) }
            }
          }
        }
      }
      let mut ox = 0;
      let mut oy = 0;
      {
        let cp = cs.borrow::<CPosition>(&e).unwrap();
        ox = cp.x;
        oy = cp.y;
        cp.x = x;
        cp.y = y;
      }

      self.move(next.unwrap(), cs, ox, oy)
    }
}

impl BulkEntityProcess for SnakeMoverProcess {
    fn process(&mut self, es: Vec<&Entity>, _: &World, cs:&mut Components){
        for e in es.iter() {
            let sd = *cs.borrow::<SnakeDirection>(&**e).unwrap();
            let mut ox = 0u64;
            let mut oy = 0u64;
            {
              let cp = cs.borrow::<CPosition>(&**e).unwrap();
              ox = cp.x;
              oy = cp.y;
              match sd.direction {
                Left => cp.x-=1,
                Right => cp.x+=1,
                Up => cp.y-=1,
                Down => cp.y+=1
              }
            }
          self.move(**e,cs, ox, oy)
        }
    }
}