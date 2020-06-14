use altv::app::{ApplicationBuilder, CoreApplication};
use altv::ecs::{Component, ReadStorage, VecStorage, WriteStorage};
use altv::game_data::{GameData, GameDataBuilder, StateData};
use altv::sdk::elements::CPlayer;
use altv::sdk::events::*;
use altv::state::State;
use std::error::Error;
use std::time::Instant;

pub struct CCreationInfo {
    joined_at: Instant,
}

impl Component for CCreationInfo {
    type Storage = VecStorage<Self>;
}

pub struct GameState;

impl State for GameState {
    fn handle_event(&mut self, data: StateData<GameData>, event: CEvent) {
        match &event {
            CEvent::PlayerConnect(event) => {
                // add the component for the player entity
                data.world
                    .exec(|mut ccreation_infos: WriteStorage<CCreationInfo>| {
                        ccreation_infos.insert(
                            event.get_target(),
                            CCreationInfo {
                                joined_at: Instant::now(),
                            },
                        ).unwrap();
                    });
            }
            CEvent::PlayerDisconnect(event) => {
                data.world.exec(
                    |(ccreation_infos, cplayers): (
                        ReadStorage<CCreationInfo>,
                        ReadStorage<CPlayer>,
                    )| {
                        let ccreation_info = ccreation_infos.get(event.get_target()).unwrap();
                        let cplayer = cplayers.get(event.get_target()).unwrap();

                        altv::sdk::log::info(
                            format!(
                                "Player {} stayed for {:?} on the server.",
                                cplayer.get_name(),
                                ccreation_info.joined_at.elapsed()
                            )
                            .as_str(),
                        );
                    },
                );
            }
            _ => {}
        };
    }
}

#[no_mangle]
pub fn main(core: usize) -> Result<CoreApplication, Box<dyn Error>> {
    let game_data_builder = GameDataBuilder::new();
    let application = ApplicationBuilder::new(core, Box::new(GameState))
        .register::<CCreationInfo>() // register the component
        .build(game_data_builder);
    Ok(application)
}
