use altv::ecs::{ReadStorage, System, WriteStorage};
use altv::sdk::elements::*;

pub struct PlayerSpawner;

impl<'a> System<'a> for PlayerSpawner {
    type SystemData = (ReadStorage<'a, CEntity>, WriteStorage<'a, CPlayer>);

    fn run(&mut self, (_positions, mut _players): Self::SystemData) {
        // for (position, player) in (&positions, &mut players).join() {
        //     if player.is_dead() {
        //         player.spawn(position.get_position(), Duration::from_secs(0));
        //         altv_sdk::logi!("Revived player {}.", player.get_name());
        //     }
        // }
    }
}
