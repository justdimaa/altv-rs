use crate::systems::player_spawner::PlayerSpawner;
use altv::app::{ApplicationBuilder, CoreApplication};
use altv::core::AltResource;
use altv::ecs::{Join, Read, ReadStorage, WorldExt, WriteStorage};
use altv::game_data::{GameData, GameDataBuilder, StateData};
use altv::sdk::elements::*;
use altv::sdk::events::*;
use altv::sdk::vector::Vector3;
use altv::state::State;
use std::error::Error;
use std::time::Duration;

mod components;
mod systems;

pub struct GameState;

impl State for GameState {
    fn on_start(&mut self, _data: StateData<GameData>) {}

    fn handle_event(&mut self, data: StateData<GameData>, event: CEvent) {
        match &event {
            CEvent::PlayerConnect(event) => {
                let target = event.get_target();

                data.world.exec(
                    |(mut cplayers, mut cworld_objs): (
                        WriteStorage<CPlayer>,
                        WriteStorage<CWorldObject>,
                    )| {
                        let cplayer = cplayers.get_mut(target).unwrap();
                        let cworld_obj = cworld_objs.get_mut(target).unwrap();

                        cworld_obj.set_position(Vector3::new(0.0, 0.0, 71.2));

                        cplayer.spawn(Vector3::new(0.0, 0.0, 71.2), Duration::from_secs(0));
                        cplayer.set_model(0x705E61F2);

                        cplayer.give_weapon(0x78A97CD0, 200, false);
                        cplayer.give_weapon(0x93E220BD, 200, false);
                    },
                );
            }
            CEvent::PlayerDeath(event) => {
                let target = event.get_target();

                data.world.exec(
                    |(mut cplayers, cworld_objs): (
                        WriteStorage<CPlayer>,
                        ReadStorage<CWorldObject>,
                    )| {
                        let cplayer = cplayers.get_mut(target).unwrap();
                        let cworld_obj = cworld_objs.get(target).unwrap();

                        cplayer.spawn(cworld_obj.get_position(), Duration::from_secs(0));
                        altv::sdk::log::info(
                            format!("Revived player {}.", cplayer.get_name()).as_str(),
                        );
                    },
                );
            }
            CEvent::CollisionShapeEvent(event) => {
                let entity = event.get_entity();

                let player_c = data.world.read_storage::<CPlayer>();

                if let Some(player_c) = player_c.get(entity) {
                    altv::sdk::log::info(
                        format!(
                            "Player {} triggered colshape with state {}.",
                            player_c.get_name(),
                            event.get_state()
                        )
                        .as_str(),
                    );
                }
            }
            CEvent::ConsoleCommand(event) => match event.get_name() {
                "pos" => {
                    data.world.exec(
                        |(cplayers, cworld_objs): (
                            ReadStorage<CPlayer>,
                            ReadStorage<CWorldObject>,
                        )| {
                            for (cplayer, cworld_obj) in (&cplayers, &cworld_objs).join() {
                                altv::sdk::log::info(
                                    format!(
                                        "{} | {}",
                                        cplayer.get_name(),
                                        cworld_obj.get_position()
                                    )
                                    .as_str(),
                                );
                            }
                        },
                    );
                }
                "veh" => {
                    let name = event.get_args().get(0);
                    let model = event.get_args().get(1);

                    if let (Some(name), Some(model)) = (name, model) {
                        let position = data.world.exec(
                            |(cplayers, cworld_objs): (
                                ReadStorage<CPlayer>,
                                ReadStorage<CWorldObject>,
                            )| {
                                for (cplayer, cworld_obj) in (&cplayers, &cworld_objs).join() {
                                    if cplayer.get_name().to_lowercase() == name.to_lowercase() {
                                        return Some(cworld_obj.get_position());
                                    }
                                }

                                None
                            },
                        );

                        if let Some(position) = position {
                            let vehicle = altv::sdk::elements::create_vehicle(
                                data.world,
                                altv::sdk::hash(model),
                                position,
                                Vector3::zero(),
                            );

                            if let Some(vehicle) = vehicle {
                                data.world.exec(|mut cvehicles: WriteStorage<CVehicle>| {
                                    let cvehicle = cvehicles.get_mut(vehicle).unwrap();

                                    cvehicle.set_primary_color(5);
                                    cvehicle.set_secondary_color(7);

                                    cvehicle.set_license_plate_text("RUST");
                                });

                                altv::sdk::log::info(
                                    format!("Spawned vehicle {} for player {}.", &model, &name)
                                        .as_str(),
                                );
                            }
                        } else {
                            altv::sdk::log::error(
                                format!("Could not find player {}.", &name).as_str(),
                            );
                        }
                    }
                }
                "info" => {
                    altv::sdk::log::info(
                        format!("{} players", data.world.read_storage::<CPlayer>().count())
                            .as_str(),
                    );
                    altv::sdk::log::info(
                        format!("{} vehicles", data.world.read_storage::<CVehicle>().count())
                            .as_str(),
                    );
                    altv::sdk::log::info(
                        format!("{} blips", data.world.read_storage::<CBlip>().count()).as_str(),
                    );
                    altv::sdk::log::info(
                        format!(
                            "{} collision shapes",
                            data.world.read_storage::<CCollisionShape>().count()
                        )
                        .as_str(),
                    );
                    altv::sdk::log::info(
                        format!(
                            "{} checkpoints",
                            data.world.read_storage::<CCheckpoint>().count()
                        )
                        .as_str(),
                    );
                }
                "vehinfo" => {
                    data.world.exec(
                        |(cvehicles, cplayers, centities, alt): (
                            ReadStorage<CVehicle>,
                            ReadStorage<CPlayer>,
                            ReadStorage<CEntity>,
                            Read<AltResource>,
                        )| {
                            for (cvehicle, centity) in (&cvehicles, &centities).join() {
                                let driver = cvehicle.get_driver(&alt);

                                match driver {
                                    Some(driver) => {
                                        let cplayer = cplayers.get(driver).unwrap();

                                        altv::sdk::log::info(
                                            format!(
                                                "Vehicle: {} | Driver: {}",
                                                centity.get_model(),
                                                cplayer.get_name()
                                            )
                                            .as_str(),
                                        )
                                    }
                                    None => altv::sdk::log::info(
                                        format!("Vehicle: {}", centity.get_model(),).as_str(),
                                    ),
                                }
                            }
                        },
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[no_mangle]
pub fn main(core: usize) -> Result<CoreApplication, Box<dyn Error>> {
    let game_data_builder = GameDataBuilder::new().with_thread_local(PlayerSpawner);
    let application = ApplicationBuilder::new(core, Box::new(GameState)).build(game_data_builder);
    Ok(application)
}
