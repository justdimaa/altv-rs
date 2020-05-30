use crate::game_data::{GameData, StateData};
use crate::sdk::events::CEvent;

pub trait State {
    fn on_start(&mut self, data: StateData<GameData>) {
        let _ = data;
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let _ = data;
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: CEvent) {
        let _ = data;
        let _ = event;
    }

    fn tick(&mut self, data: StateData<GameData>) {
        let _ = data;
    }
}
