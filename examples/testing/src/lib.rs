use altv::app::{ApplicationBuilder, CoreApplication};
use altv::core::AltResource;
use altv::ecs::{Read, WriteStorage};
use altv::game_data::{GameData, GameDataBuilder, StateData};
use altv::sdk::elements::*;
use altv::sdk::events::*;
use altv::sdk::mvalue::MValue;
use altv::sdk::rgba::Rgba;
use altv::sdk::vector::{Rotation3, Vector3};
use altv::state::State;
use std::error::Error;

pub struct GameState;

impl State for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        assert!(altv::sdk::elements::create_vehicle(
            data.world,
            0,
            Vector3::new(0.0, 0.0, 0.0),
            Rotation3::from_euler_angles(0.0, 0.0, 0.0)
        )
        .is_none());

        let vehicle = altv::sdk::elements::create_vehicle(
            data.world,
            altv::sdk::hash("neon"),
            Vector3::new(0.0, 0.0, 0.0),
            Rotation3::from_euler_angles(0.0, 0.0, 0.0),
        );
        assert!(vehicle.is_some());

        let vehicle = vehicle.unwrap();

        data.world.exec(
            |(mut crefs, mut cbase_objs, mut cworld_objs, mut centities, mut cvehicles, alt): (
                WriteStorage<CRefCountable>,
                WriteStorage<CBaseObject>,
                WriteStorage<CWorldObject>,
                WriteStorage<CEntity>,
                WriteStorage<CVehicle>,
                Read<AltResource>,
            )| {
                let cref = crefs.get_mut(vehicle);
                let cbase_obj = cbase_objs.get_mut(vehicle);
                let cworld_obj = cworld_objs.get_mut(vehicle);
                let centity = centities.get_mut(vehicle);
                let cvehicle = cvehicles.get_mut(vehicle);

                assert!(cref.is_some());
                assert!(cbase_obj.is_some());
                assert!(cworld_obj.is_some());
                assert!(centity.is_some());
                assert!(cvehicle.is_some());

                let cref = cref.unwrap();
                let cbase_obj = cbase_obj.unwrap();
                let cworld_obj = cworld_obj.unwrap();
                let centity = centity.unwrap();
                let cvehicle = cvehicle.unwrap();

                assert_cref(cref);
                assert_cbase_obj(cbase_obj);
                assert_cworld_obj(cworld_obj);
                assert_centity(centity);
                assert_cvehicle(cvehicle, &alt);
            },
        );

        let colshape = altv::sdk::elements::create_collision_shape_sphere(
            data.world,
            Vector3::new(0.0, 0.0, 71.2),
            5.0,
        );

        data.world.exec(
            |(mut crefs, mut cbase_objs, mut cworld_objs, mut ccolshapes): (
                WriteStorage<CRefCountable>,
                WriteStorage<CBaseObject>,
                WriteStorage<CWorldObject>,
                WriteStorage<CCollisionShape>,
            )| {
                let cref = crefs.get_mut(colshape);
                let cbase_obj = cbase_objs.get_mut(colshape);
                let cworld_obj = cworld_objs.get_mut(colshape);
                let ccolshape = ccolshapes.get_mut(colshape);

                assert!(cref.is_some());
                assert!(cbase_obj.is_some());
                assert!(cworld_obj.is_some());
                assert!(ccolshape.is_some());

                let cref = cref.unwrap();
                let cbase_obj = cbase_obj.unwrap();
                let cworld_obj = cworld_obj.unwrap();
                let ccolshape = ccolshape.unwrap();

                assert_cref(cref);
                assert_cbase_obj(cbase_obj);
                assert_cworld_obj(cworld_obj);
                assert_ccolshape(ccolshape, Vector3::new(0.0, 0.0, 71.2), 5.0);
            },
        );
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: CEvent) {
        match &event {
            CEvent::PlayerConnect(event) => {
                let target = event.get_target();

                data.world.exec(
                    |(
                        mut crefs,
                        mut cbase_objs,
                        mut cworld_objs,
                        mut centities,
                        mut cplayers,
                        alt,
                    ): (
                        WriteStorage<CRefCountable>,
                        WriteStorage<CBaseObject>,
                        WriteStorage<CWorldObject>,
                        WriteStorage<CEntity>,
                        WriteStorage<CPlayer>,
                        Read<AltResource>,
                    )| {
                        let cref = crefs.get_mut(target);
                        let cbase_obj = cbase_objs.get_mut(target);
                        let cworld_obj = cworld_objs.get_mut(target);
                        let centity = centities.get_mut(target);
                        let cplayer = cplayers.get_mut(target);

                        assert!(cref.is_some());
                        assert!(cbase_obj.is_some());
                        assert!(cworld_obj.is_some());
                        assert!(centity.is_some());
                        assert!(cplayer.is_some());

                        let cref = cref.unwrap();
                        let cbase_obj = cbase_obj.unwrap();
                        let cworld_obj = cworld_obj.unwrap();
                        let centity = centity.unwrap();
                        let cplayer = cplayer.unwrap();

                        assert_cref(cref);
                        assert_cbase_obj(cbase_obj);
                        assert_cworld_obj(cworld_obj);
                        assert_centity(centity);
                        assert_cplayer(cplayer, &alt);
                    },
                );
            }
            _ => {}
        }
    }
}

#[no_mangle]
pub fn main(core: usize) -> Result<CoreApplication, Box<dyn Error>> {
    let game_data_builder = GameDataBuilder::new();
    let application = ApplicationBuilder::new(core, Box::new(GameState)).build(game_data_builder);
    Ok(application)
}

fn assert_cref(cref: &mut CRefCountable) {
    let ref_cnt = cref.get_ref_count();
    cref.add_ref();
    assert_eq!(ref_cnt + 1, cref.get_ref_count());
    cref.remove_ref();
    assert_eq!(ref_cnt, cref.get_ref_count());
}

fn assert_cbase_obj(cbase_obj: &mut CBaseObject) {
    assert!(!cbase_obj.has_meta_data("test"));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::None);
    cbase_obj.set_meta_data("test", MValue::Nil);
    assert!(cbase_obj.has_meta_data("test"));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Nil);
    cbase_obj.delete_meta_data("test");
    assert!(!cbase_obj.has_meta_data("test"));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::None);

    cbase_obj.set_meta_data("test", MValue::Bool(false));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Bool(false));

    cbase_obj.set_meta_data("test", MValue::Bool(true));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Bool(true));

    cbase_obj.set_meta_data("test", MValue::Int(0));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Int(0));
    cbase_obj.set_meta_data("test", MValue::Int(std::i64::MIN));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Int(std::i64::MIN));
    cbase_obj.set_meta_data("test", MValue::Int(std::i64::MAX));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Int(std::i64::MAX));

    cbase_obj.set_meta_data("test", MValue::Uint(std::u64::MIN));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Uint(std::u64::MIN));
    cbase_obj.set_meta_data("test", MValue::Uint(std::u64::MAX));
    assert_eq!(cbase_obj.get_meta_data("test"), MValue::Uint(std::u64::MAX));

    cbase_obj.set_meta_data("test", MValue::Double(std::f64::MIN));
    assert_eq!(
        cbase_obj.get_meta_data("test"),
        MValue::Double(std::f64::MIN)
    );
    cbase_obj.set_meta_data("test", MValue::Double(std::f64::MAX));
    assert_eq!(
        cbase_obj.get_meta_data("test"),
        MValue::Double(std::f64::MAX)
    );

    cbase_obj.set_meta_data("test", MValue::String("test".to_owned()));
    assert_eq!(
        cbase_obj.get_meta_data("test"),
        MValue::String("test".to_owned())
    );

    // cbase_obj.set_meta_data("test", MValue::List(vec![MValue::List(vec![MValue::Bool(false), MValue::Bool(true), MValue::List(vec![MValue::Int(0)])])]));
    // assert_eq!(cbase_obj.get_meta_data("test"), MValue::List(vec![MValue::List(vec![MValue::Bool(false), MValue::Bool(true), MValue::List(vec![MValue::Int(0)])])]));

    // cbase_obj.set_meta_data("test", MValue::Vector3(Vector3::new(0.0, std::f32::MIN, std::f32::MAX)));
    // assert_eq!(cbase_obj.get_meta_data("test"), MValue::Vector3(Vector3::new(0.0, std::f32::MIN, std::f32::MAX))); // fails

    cbase_obj.set_meta_data("test", MValue::Rgba(Rgba::new(0, 64, 128, 255)));
    assert_eq!(
        cbase_obj.get_meta_data("test"),
        MValue::Rgba(Rgba::new(0, 64, 128, 255))
    );

    cbase_obj.set_meta_data("test", MValue::ByteArray(vec![0, 64, 128, 255]));
    assert_eq!(
        cbase_obj.get_meta_data("test"),
        MValue::ByteArray(vec![0, 64, 128, 255])
    );
}

fn assert_cworld_obj(cworld_obj: &mut CWorldObject) {
    for n in -1..1 {
        cworld_obj.set_dimension(n);
        assert_eq!(cworld_obj.get_dimension(), n);
    }

    cworld_obj.set_position(Vector3::new(1.0, 1.0, 71.2));
    assert_eq!(cworld_obj.get_position(), Vector3::new(1.0, 1.0, 71.2));
}

fn assert_centity(centity: &mut CEntity) {
    // centity.set_rotation(Rotation3::from_euler_angles(0.0, 1.0, 2.0));
    // assert_eq!(centity.get_rotation(), Rotation3::from_euler_angles(0.0, 1.0, 2.0));

    assert!(!centity.has_synced_meta_data("test"));
    assert_eq!(centity.get_synced_meta_data("test"), MValue::None);
    centity.set_synced_meta_data("test", MValue::Nil);
    assert!(centity.has_synced_meta_data("test"));
    assert_eq!(centity.get_synced_meta_data("test"), MValue::Nil);
    centity.delete_synced_meta_data("test");
    assert!(!centity.has_synced_meta_data("test"));
    assert_eq!(centity.get_synced_meta_data("test"), MValue::None);

    assert!(!centity.has_stream_synced_meta_data("test"));
    assert_eq!(centity.get_stream_synced_meta_data("test"), MValue::None);
    centity.set_stream_synced_meta_data("test", MValue::Nil);
    assert!(centity.has_stream_synced_meta_data("test"));
    assert_eq!(centity.get_stream_synced_meta_data("test"), MValue::Nil);
    centity.delete_stream_synced_meta_data("test");
    assert!(!centity.has_stream_synced_meta_data("test"));
    assert_eq!(centity.get_stream_synced_meta_data("test"), MValue::None);
}

fn assert_cplayer(cplayer: &mut CPlayer, alt: &AltResource) {
    // cplayer.set_model(0x705E61F2);
    // assert_eq!(centity.get_model(), 0x705E61F2);

    cplayer.set_health(175);
    assert_eq!(cplayer.get_health(), 175);

    cplayer.set_armor(150);
    assert_eq!(cplayer.get_armor(), 150);

    cplayer.set_max_health(300);
    assert_eq!(cplayer.get_max_health(), 300);

    cplayer.set_max_armor(325);
    assert_eq!(cplayer.get_max_armor(), 325);

    assert!(!cplayer.is_in_vehicle());
    assert!(cplayer.get_vehicle(&alt).is_none());

    // assert!(!cplayer.is_dead()); // fails

    // assert!(!cplayer.is_aiming()); // fails
    assert!(cplayer.is_connected());
    assert!(!cplayer.is_flashlight_active());
    // assert!(!cplayer.is_in_ragdoll()); // fails
    // assert!(!cplayer.is_reloading()); // fails
    // assert!(!cplayer.is_shooting()); // fails

    cplayer.set_health(100);
    assert_eq!(cplayer.get_health(), 100);

    assert!(cplayer.is_dead());

    cplayer.set_health(175);
    assert_eq!(cplayer.get_health(), 175);

    // assert!(!cplayer.is_dead()); // fails
}

fn assert_cvehicle(cvehicle: &mut CVehicle, alt: &AltResource) {
    assert!(cvehicle.get_driver(alt).is_none());

    for n in 0..std::u8::MAX {
        cvehicle.set_primary_color(n);
        assert!(!cvehicle.is_primary_color_rgb());
        assert_eq!(cvehicle.get_primary_color(), n);
    }

    cvehicle.set_primary_color_rgb(Rgba::new(0, 64, 128, 0));
    assert!(cvehicle.is_primary_color_rgb());
    assert_eq!(cvehicle.get_primary_color_rgb(), Rgba::new(0, 64, 128, 255));

    for n in 0..std::u8::MAX {
        cvehicle.set_secondary_color(n);
        assert!(!cvehicle.is_secondary_color_rgb());
        assert_eq!(cvehicle.get_secondary_color(), n);
    }

    cvehicle.set_secondary_color_rgb(Rgba::new(0, 64, 128, 0));
    assert!(cvehicle.is_secondary_color_rgb());
    assert_eq!(
        cvehicle.get_secondary_color_rgb(),
        Rgba::new(0, 64, 128, 255)
    );

    cvehicle.set_tire_smoke_color(Rgba::new(255, 128, 0, 0));
    assert_eq!(cvehicle.get_tire_smoke_color(), Rgba::new(255, 128, 0, 255));

    cvehicle.set_neon_active(false, false, true, true);
    assert_eq!(cvehicle.get_neon_active(), (false, false, true, true));
    assert!(cvehicle.is_neon_active());

    cvehicle.set_neon_color(Rgba::new(1, 2, 3, 0));
    assert_eq!(cvehicle.get_neon_color(), Rgba::new(1, 2, 3, 255));

    cvehicle.set_neon_active(false, false, false, false);
    assert!(!cvehicle.is_neon_active());

    cvehicle.set_neon_color(Rgba::new(1, 2, 3, 255));
    assert_eq!(cvehicle.get_neon_color(), Rgba::new(0, 0, 0, 0));

    for n in 0..std::u8::MAX {
        cvehicle.set_window_tint(n);
        assert_eq!(cvehicle.get_window_tint(), n);
    }

    for n in 0..5 {
        cvehicle.set_license_plate_index(n);
        assert_eq!(cvehicle.get_license_plate_index(), n);
    }

    cvehicle.set_license_plate_text("RUST");
    assert_eq!(cvehicle.get_license_plate_text(), "RUST".to_owned());

    cvehicle.set_engine_on(true);
    assert!(cvehicle.is_engine_on());

    cvehicle.set_engine_on(false);
    assert!(!cvehicle.is_engine_on());

    // cvehicle.set_body_health(750);
    // assert_eq!(cvehicle.get_body_health(), 750); // fails

    // cvehicle.set_body_additional_health(800);
    // assert_eq!(cvehicle.get_body_additional_health(), 800); // fails

    for n in 0..1000 {
        cvehicle.set_engine_health(n);
        assert_eq!(cvehicle.get_engine_health(), n);
    }

    for n in 0..1000 {
        cvehicle.set_fuel_tank_health(n);
        assert_eq!(cvehicle.get_fuel_tank_health(), n);
    }

    for n in 0..std::u8::MAX {
        cvehicle.set_dirt_level(n);
        assert_eq!(cvehicle.get_dirt_level(), n);
    }

    for n in 0..std::u8::MAX {
        cvehicle.set_dashboard_color(n);
        assert_eq!(cvehicle.get_dashboard_color(), n);
    }

    for n in 0..std::u8::MAX {
        cvehicle.set_headlight_color(n);
        assert_eq!(cvehicle.get_headlight_color(), n);
    }

    for n in 0..std::u8::MAX {
        cvehicle.set_interior_color(n);
        assert_eq!(cvehicle.get_interior_color(), n);
    }

    for n in 0..std::u8::MAX {
        cvehicle.set_pearl_color(n);
        assert_eq!(cvehicle.get_pearl_color(), n);
    }
}

fn assert_ccolshape(ccolshape: &mut CCollisionShape, position: Vector3, radius: f32) {
    assert!(ccolshape.is_point_in(position.clone()));
    assert!(ccolshape.is_point_in(position.clone() + Vector3::new(radius, 0.0, 0.0)));
    assert!(ccolshape.is_point_in(position.clone() + Vector3::new(0.0, radius, 0.0)));
    // assert!(ccolshape.is_point_in(position.clone() + Vector3::new(0.0, 0.0, radius))); // fails
    assert!(!ccolshape.is_point_in(position.clone() + Vector3::new(radius + 1.0, 0.0, 0.0)));
    assert!(!ccolshape.is_point_in(position.clone() + Vector3::new(0.0, radius + 1.0, 0.0)));
    assert!(!ccolshape.is_point_in(position.clone() + Vector3::new(0.0, 0.0, radius + 1.0)));
}
