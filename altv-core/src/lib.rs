pub use specs as ecs;

use ecs::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct AltResource {
    pub players: HashMap<usize, Entity>,
    pub vehicles: HashMap<usize, Entity>,
    pub blips: HashMap<usize, Entity>,
    pub voice_channels: HashMap<usize, Entity>,
    pub collision_shapes: HashMap<usize, Entity>,
    pub checkpoints: HashMap<usize, Entity>,
}
