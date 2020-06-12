use altv::ecs::{Join, ReadStorage, System, WriteStorage};
use altv::sdk::elements::*;
use altv::sdk::mvalue::MValue;
use std::time::{Duration, Instant};

pub struct AfkKicker {
    last_check: Instant,
}

impl AfkKicker {
    pub fn new() -> Self {
        AfkKicker {
            last_check: Instant::now(),
        }
    }
}

impl<'a> System<'a> for AfkKicker {
    type SystemData = (
        WriteStorage<'a, CBaseObject>,
        ReadStorage<'a, CWorldObject>,
        WriteStorage<'a, CPlayer>,
    );

    fn run(&mut self, (mut cbase_objs, cworld_objs, mut cplayers): Self::SystemData) {
        if self.last_check.elapsed() >= Duration::from_secs(300) {
            for (cbase_obj, cworld_obj, cplayer) in
                (&mut cbase_objs, &cworld_objs, &mut cplayers).join()
            {
                let last_position = cbase_obj.get_meta_data("lastPos");
                let current_position = cworld_obj.get_position();

                if let MValue::Vector3(last_position) = last_position {
                    if last_position.metric_distance(&current_position) < 0.5 {
                        cplayer.kick("");
                    }
                } else {
                    cbase_obj.set_meta_data("lastPos", MValue::Vector3(current_position.clone()))
                }
            }

            self.last_check = Instant::now();
        }
    }
}
