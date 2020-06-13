use crate::systems::afk_kicker::AfkKicker;
use crate::systems::weather_sync::WeatherSync;
use altv::app::{ApplicationBuilder, CoreApplication};
use altv::core::AltResource;
use altv::ecs::{Join, Read, ReadStorage, WorldExt, WriteStorage};
use altv::game_data::{GameData, GameDataBuilder, StateData};
use altv::sdk::elements::*;
use altv::sdk::events::*;
use altv::sdk::mvalue::MValue;
use altv::sdk::vector::{Rotation3, Vector3};
use altv::state::State;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

mod components;
mod systems;

pub fn get_direction_from_rotation(rotation: Rotation3) -> Vector3 {
    let euler = rotation.euler_angles();

    let num = euler.0.cos().abs();
    Vector3::new(euler.2.sin() * num, euler.2.cos() * num, euler.0.sin())
}

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
                        altv::sdk::core::emit_client(
                            None,
                            "chatmessage",
                            &[
                                MValue::String("SERVER".to_owned()),
                                MValue::String(
                                    format!(
                                        "{{E0FFFF}}Player {} joined the server.",
                                        cplayer.get_name()
                                    )
                                    .to_owned(),
                                ),
                            ],
                        )
                    },
                );
            }
            CEvent::PlayerDisconnect(event) => {
                let target = event.get_target();

                data.world.exec(|cplayers: ReadStorage<CPlayer>| {
                    let cplayer = cplayers.get(target).unwrap();
                    altv::sdk::core::emit_client(
                        None,
                        "chatmessage",
                        &[MValue::String(
                            format!("{{E0FFFF}}Player {} left the server.", cplayer.get_name())
                                .to_owned(),
                        )],
                    )
                });
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
            CEvent::ClientScript(event) => match event.get_name() {
                "chatmessage" => {
                    let msg = event.get_args().get(0).unwrap();

                    if let MValue::String(msg) = msg {
                        if msg.starts_with("/") {
                            let mut command = msg.clone();
                            command.remove(0);
                            let command: Vec<_> = command.split_ascii_whitespace().collect();

                            match *command.get(0).unwrap() {
                                "pos" => {
                                    data.world.exec(
                                            |(mut cplayers, cworld_objs): (
                                                WriteStorage<CPlayer>,
                                                ReadStorage<CWorldObject>,
                                            )| {
                                                let cplayer =
                                                    cplayers.get_mut(event.get_target()).unwrap();
                                                let cworld_obj =
                                                    cworld_objs.get(event.get_target()).unwrap();

                                                let position = cworld_obj.get_position();

                                                cplayer.emit(
                                                    "chatmessage",
                                                    &[
                                                        MValue::String("SERVER".to_owned()),
                                                        MValue::String(
                                                            format!(
                                                                "Position: [X: {}, Y: {}, Z: {}] | Dimension: {}",
                                                                position.x, position.y, position.z,
                                                                cworld_obj.get_dimension()
                                                            )
                                                            .to_owned(),
                                                        ),
                                                    ],
                                                );
                                            },
                                        );
                                }
                                "rot" => {
                                    data.world.exec(
                                            |(mut cplayers, centities): (
                                                WriteStorage<CPlayer>,
                                                ReadStorage<CEntity>,
                                            )| {
                                                let cplayer =
                                                    cplayers.get_mut(event.get_target()).unwrap();
                                                let centity =
                                                    centities.get(event.get_target()).unwrap();

                                                let rotation = centity.get_rotation();
                                                let euler = rotation.euler_angles();

                                                cplayer.emit(
                                                    "chatmessage",
                                                    &[
                                                        MValue::String("SERVER".to_owned()),
                                                        MValue::String(
                                                            format!(
                                                                "Rotation: [Roll: {} Pitch: {} Yaw: {}]",
                                                                euler.0, euler.1, euler.2
                                                            )
                                                                .to_owned(),
                                                        ),
                                                    ],
                                                );
                                            },
                                        );
                                }
                                "veh" => {
                                    let model = command.get(1);

                                    if let Some(model) = model {
                                        let (dimension, position, rotation) = data.world.exec(
                                            |(cworld_objs, centities): (
                                                ReadStorage<CWorldObject>,
                                                ReadStorage<CEntity>,
                                            )| {
                                                let cworld_obj =
                                                    cworld_objs.get(event.get_target()).unwrap();
                                                let centity =
                                                    centities.get(event.get_target()).unwrap();
                                                (
                                                    cworld_obj.get_dimension(),
                                                    cworld_obj.get_position(),
                                                    centity.get_rotation(),
                                                )
                                            },
                                        );

                                        let euler = rotation.euler_angles();

                                        let vehicle = altv::sdk::elements::create_vehicle(
                                            data.world,
                                            altv::sdk::hash(model),
                                            position + get_direction_from_rotation(rotation) * 3.0,
                                            Rotation3::from_euler_angles(
                                                0.0,
                                                0.0,
                                                -euler.2 + std::f32::consts::PI / 2.0,
                                            ),
                                        );

                                        if let Some(vehicle) = vehicle {
                                            data.world.exec(
                                                |(mut cworld_objs, mut cvehicles): (
                                                    WriteStorage<CWorldObject>,
                                                    WriteStorage<CVehicle>,
                                                )| {
                                                    let cworld_obj =
                                                        cworld_objs.get_mut(vehicle).unwrap();
                                                    let cvehicle =
                                                        cvehicles.get_mut(vehicle).unwrap();

                                                    cworld_obj.set_dimension(dimension);

                                                    cvehicle.set_primary_color(5);
                                                    cvehicle.set_secondary_color(7);

                                                    cvehicle.set_license_plate_text("RUST");
                                                },
                                            );

                                            data.world.exec(
                                                |mut cplayers: WriteStorage<CPlayer>| {
                                                    let cplayer = cplayers
                                                        .get_mut(event.get_target())
                                                        .unwrap();
                                                    cplayer.emit(
                                                        "chatmessage",
                                                        &[
                                                            MValue::String("SERVER".to_owned()),
                                                            MValue::String(
                                                                format!(
                                                                    "{{00FF00}}Spawned vehicle {}.",
                                                                    &model
                                                                )
                                                                .to_owned(),
                                                            ),
                                                        ],
                                                    );
                                                },
                                            );
                                        } else {
                                            data.world.exec(
                                                |mut cplayers: WriteStorage<CPlayer>| {
                                                    let cplayer = cplayers
                                                        .get_mut(event.get_target())
                                                        .unwrap();
                                                    cplayer.emit(
                                                        "chatmessage",
                                                        &[
                                                            MValue::String("SERVER".to_owned()),
                                                            MValue::String(
                                                                format!(
                                                                    "{{FF0000}}Could not find vehicle with model {}.",
                                                                    &model
                                                                )
                                                                    .to_owned(),
                                                            ),
                                                        ],
                                                    );
                                                },
                                            );
                                        }
                                    }
                                }
                                "weapon" => {
                                    let model = command.get(1);
                                    let ammo = command.get(2);

                                    if let Some(model) = model {
                                        data.world.exec(|mut cplayers: WriteStorage<CPlayer>| {
                                            let cplayer = cplayers.get_mut(event.get_target()).unwrap();

                                            if let Some(ammo) = ammo {
                                                if let Ok(ammo) = i32::from_str(ammo) {
                                                    cplayer.give_weapon(altv::sdk::hash(*model), ammo, true);
                                                    cplayer.emit("chatmessage", &[
                                                        MValue::String("SERVER".to_owned()),
                                                        MValue::String(
                                                            format!(
                                                                "{{00FF00}}Spawned weapon {} with {} ammunition.",
                                                                *model, ammo
                                                            )
                                                                .to_owned(),
                                                        )])
                                                } else {
                                                    cplayer.emit("chatmessage", &[
                                                        MValue::String("SERVER".to_owned()),
                                                        MValue::String(
                                                            format!(
                                                                "{{00FF00}}Invalid ammunition {}.",
                                                                ammo
                                                            )
                                                                .to_owned(),
                                                        )])
                                                }
                                            } else {
                                                cplayer.give_weapon(altv::sdk::hash(*model), i32::MAX, true);
                                                cplayer.emit("chatmessage", &[
                                                    MValue::String("SERVER".to_owned()),
                                                    MValue::String(
                                                        format!(
                                                            "{{00FF00}}Spawned weapon {}.",
                                                            *model
                                                        )
                                                            .to_owned(),
                                                    )])
                                            }
                                        })
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            },
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
                                Rotation3::from_euler_angles(0.0, 0.0, 0.0),
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
                    altv::sdk::log::info(
                        format!(
                            "{} voice channels",
                            data.world.read_storage::<CVoiceChannel>().count()
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
    let game_data_builder = GameDataBuilder::new()
        .with_thread_local(AfkKicker::new())
        .with_thread_local(WeatherSync::new());
    let application = ApplicationBuilder::new(core, Box::new(GameState)).build(game_data_builder);
    Ok(application)
}
