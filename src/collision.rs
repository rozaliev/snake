use std::rand;
use ecs;
use ecs::{Component,Phantom, Entity, World, Components};
use ecs::system::{BulkEntityProcess,BulkEntitySystem};

use snake::{Head, Target};
use positioner::{CPosition};

pub struct CollisionProcess;

impl CollisionProcess {
    pub fn new_to_system() -> Box<BulkEntitySystem> {
        let pos = box CollisionProcess;

        let conds = ecs::Aspect::for_all(vec![Component::cid(Phantom::<Head>)]);
        let bulk = box BulkEntitySystem::new(pos,conds);

        return bulk
    }

    fn check_for_target(&self, e: Entity,cs:&mut Components ) {
      let target = cs.borrow::<Target>(&e).unwrap().t.unwrap();
      let spos = *cs.borrow::<CPosition>(&e).unwrap();
      let mut tpos = cs.borrow::<CPosition>(&target).unwrap();
      if spos.x == tpos.x && spos.y == tpos.y {
        tpos.x = rand::random::<u64>() % 25;
        tpos.y = rand::random::<u64>() % 25;
      }
    }
}

impl BulkEntityProcess for CollisionProcess {
    fn process(&mut self, es: Vec<&Entity>, _: &World, cs:&mut Components){
        for e in es.iter() {
          self.check_for_target(**e,cs);
        }
    }
}