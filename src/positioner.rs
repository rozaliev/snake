use ecs;
use ecs::{Component,Phantom, Entity, World, Components};
use ecs::system::{BulkEntityProcess,BulkEntitySystem};

use square::Square;

component!(
    ID_CellPosition: CPosition {
      x: u64,
      y: u64
    }
)


pub struct PositionerProcess;

impl PositionerProcess {
    pub fn new_to_system() -> Box<BulkEntitySystem> {
        let pos = box PositionerProcess;

        let conds = ecs::Aspect::for_all(vec![Component::cid(Phantom::<CPosition>),Component::cid(Phantom::<Square>)]);
        let bulk = box BulkEntitySystem::new(pos,conds);

        return bulk
    }
}

impl BulkEntityProcess for PositionerProcess {
    fn process(&mut self, es: Vec<&Entity>, _: &World, cs:&mut Components){
        for e in es.iter() {
          let mut x = 0;
          let mut y = 0;
          {
            let pos = cs.borrow::<CPosition>(*e).unwrap();
            x = pos.x;
            y = pos.y;
          }

          let mut sq = cs.borrow::<Square>(*e).unwrap();
          sq.x = (x * 32) as f64;
          sq.y = (y * 32) as f64;
        }
    }
}