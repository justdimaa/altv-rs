<img align="left" width="64px" src="https://altv.mp/img/v_logo.svg" />

# alt:V Rust Module

[![Rust](https://github.com/DimaaIO/altv-rs/workflows/Rust/badge.svg?branch=master)]()
[![Current Crates.io Version](https://img.shields.io/crates/v/altv.svg)](https://crates.io/crates/altv)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE.md)

## What is alt:V?

alt:V is a free third-party multiplayer modification for Grand Theft Auto: V.
It allows you to play with your friends on dedicated servers with custom gamemodes with an ultimate experience.

The module allows you to write your own gamemode in Rust.

## Setup

Before you start writing your own gamemode, setup an alt:V server following this tutorial:
- [Windows](https://wiki.altv.mp/Tutorial:Server_Setup_-_Windows)
- [Linux](https://wiki.altv.mp/Tutorial:Server_Setup_-_Linux)

To make the alt:V server compatible with rust, paste the [rust-module](https://github.com/DimaaIO/altv-rs/releases/tag/v0.2.0) into the `modules/` folder and add this line to your server.cfg file:

```
modules: [ rust-module ]
```

If you want to use multiple modules e.g. the csharp-module or js-module, seperate them with a `,`:

```
modules: [ rust-module, csharp-module, js-module ]
```

## Example

Create a rust lib project and add this line to your Cargo.toml file under [dependencies]:

```toml
altv = "0.2.0" 
```

Alternatively, pull it from GitHub to obtain the latest version from develop:
```toml
altv = { git = "https://github.com/DimaaIO/altv-rs" } 
```

Then replace src/lib.rs with the following:

```rust
use altv::app::{ApplicationBuilder, CoreApplication};
use altv::game_data::{GameData, GameDataBuilder, StateData};
use altv::sdk::events::*;
use altv::state::State;
use std::error::Error;

pub struct GameState;

impl State for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        altv::sdk::log::info("Hello from rust!");
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        altv::sdk::log::info("Time to sleep, bye!");
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: CEvent) {
        match &event {
            CEvent::PlayerConnect(event) => {
                altv::sdk::log::info("Listen closely! A new player connected to the server!")
            }
            _ => {}
        };
    }
}

#[no_mangle]
pub fn main(core: usize) -> Result<CoreApplication, Box<dyn Error>> {
    let game_data_builder = GameDataBuilder::new();
    let application = ApplicationBuilder::new(core, Box::new(GameState)).build(game_data_builder);
    Ok(application)
}
```

We have several simple example projects included.
You can see the full list in the `examples/` folder.

To compile any of the examples run:

```shell script
$ cargo build
```

Now create a new folder for the resource: `resource/example` and create a new file `resource.cfg` with the following text:

```
type: rust,
main: libexample.so
```

To enable the resource, add a reference in the server.cfg file:

```
resources: [ example ]
```

Then copy the compiled lib from `target/[debug/release]/libexample.so` to the example resource folder.
