use crate::core::ecs::{Builder, Component, Entity, World, WorldExt};
use crate::core::AltResource;
use crate::game_data::{DataInit, GameData, StateData};
use crate::sdk::elements::{
    CBaseObject, CBlip, CCheckpoint, CCollisionShape, CEntity, CPlayer, CRefCountable, CVehicle,
    CVoiceChannel, CWorldObject,
};
use crate::sdk::events::{
    CClientScriptEvent, CCollisionShapeEvent, CConsoleCommandEvent, CDataNodeReceivedEvent, CEvent,
    CExplosionEvent, CGlobalMetaChangeEvent, CGlobalSyncedMetaChangeEvent,
    CPlayerChangeVehicleSeatEvent, CPlayerConnectEvent, CPlayerDamageEvent, CPlayerDeathEvent,
    CPlayerDisconnectEvent, CPlayerEnterVehicleEvent, CPlayerLeaveVehicleEvent, CRemoveEntityEvent,
    CServerScriptEvent, CStreamSyncedMetaChangeEvent, CSyncedMetaChangeEvent, CWeaponDamageEvent,
};
use crate::sdk::mvalue::MValue;
use crate::sdk::natives::*;
use crate::sdk::string_view::StringView;
use crate::sdk::vector::Vector3;
use crate::state::State;
use std::error::Error;
use std::sync::atomic::AtomicPtr;

pub type ResourceMainFn = fn(core: usize) -> Result<CoreApplication, Box<dyn Error>>;

pub struct ApplicationBuilder {
    core: usize,
    world: World,
    state: Box<dyn State>,
}

impl ApplicationBuilder {
    pub fn new(core: usize, state: Box<dyn State>) -> Self {
        let mut world = World::new();
        world.insert(AltResource::default());
        // world.insert(EventChannel::<CEvent>::with_capacity(40));
        world.register::<CRefCountable>();
        world.register::<CBaseObject>();
        world.register::<CWorldObject>();
        world.register::<CEntity>();
        world.register::<CPlayer>();
        world.register::<CVehicle>();
        world.register::<CBlip>();
        world.register::<CVoiceChannel>();
        world.register::<CCollisionShape>();
        world.register::<CCheckpoint>();

        ApplicationBuilder { core, world, state }
    }

    pub fn register<C>(mut self) -> Self
    where
        C: Component,
        C::Storage: Default,
    {
        self.world.register::<C>();
        self
    }

    pub fn build<I>(mut self, init: I) -> CoreApplication
    where
        I: DataInit<GameData<'static, 'static>>,
    {
        unsafe {
            alt_ICore_SetInstance(self.core as *mut alt_ICore);
        }

        let data = init.build(&mut self.world);

        CoreApplication {
            world: self.world,
            state: self.state,
            data,
        }
    }
}

pub struct CoreApplication {
    world: World,
    state: Box<dyn State>,
    data: GameData<'static, 'static>,
}

impl CoreApplication {
    pub fn start(&mut self) {
        self.state
            .on_start(StateData::new(&mut self.world, &mut self.data))
    }

    pub fn stop(&mut self) {
        self.state
            .on_stop(StateData::new(&mut self.world, &mut self.data))
    }

    pub fn tick(&mut self) {
        self.state
            .tick(StateData::new(&mut self.world, &mut self.data));
        self.world.maintain();
        self.data.update(&mut self.world);
    }

    pub fn handle_event(&mut self, event: *mut alt_CEvent) {
        unsafe {
            let ce = match alt_CEvent_GetType(event) {
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_CONNECT => {
                    let event = event as *mut alt_CPlayerConnectEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.players.get(&((*event).target.ptr as usize)).unwrap();

                    Some(CEvent::PlayerConnect(CPlayerConnectEvent::new(
                        *target,
                        String::new(),
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_DISCONNECT => {
                    let event = event as *mut alt_CPlayerDisconnectEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.players.get(&((*event).target.ptr as usize)).unwrap();

                    Some(CEvent::PlayerDisconnect(CPlayerDisconnectEvent::new(
                        *target,
                        String::new(),
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_CLIENT_SCRIPT_EVENT => {
                    let event = event as *mut alt_CClientScriptEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.players.get(&((*event).target.ptr as usize)).unwrap();

                    let name = alt_CClientScriptEvent_GetName_CAPI_Heap(event);
                    let name = StringView::from(*name).get_data();
                    dbg!(&name);

                    let args = alt_CClientScriptEvent_GetArgs(event);
                    let args = (*args).into();

                    Some(CEvent::ClientScript(CClientScriptEvent::new(
                        *target, name, args,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_SERVER_SCRIPT_EVENT => {
                    let event = event as *mut alt_CServerScriptEvent;

                    let name = alt_CServerScriptEvent_GetName_CAPI_Heap(event);
                    let name = StringView::from(*name).get_data();

                    let args = alt_CServerScriptEvent_GetArgs(event);
                    let args = (*args).into();

                    Some(CEvent::ServerScript(CServerScriptEvent::new(name, args)))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_SYNCED_META_CHANGE => {
                    let event = event as *mut alt_CSyncedMetaDataChangeEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = CoreApplication::get_entity(&alt, (*event).target.ptr);

                    let key = alt_CSyncedMetaDataChangeEvent_GetKey_CAPI_Heap(event);
                    let key = StringView::from(*key).get_data();

                    let val = alt_CSyncedMetaDataChangeEvent_GetVal_CAPI_Heap(event);
                    let val = MValue::new((*val).ptr);

                    let old_val = alt_CSyncedMetaDataChangeEvent_GetOldVal_CAPI_Heap(event);
                    let old_val = MValue::new((*old_val).ptr);

                    Some(CEvent::SyncedMetaChange(CSyncedMetaChangeEvent::new(
                        target, key, val, old_val,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_STREAM_SYNCED_META_CHANGE => {
                    let event = event as *mut alt_CStreamSyncedMetaDataChangeEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = CoreApplication::get_entity(&alt, (*event).target.ptr);

                    let key = alt_CStreamSyncedMetaDataChangeEvent_GetKey_CAPI_Heap(event);
                    let key = StringView::from(*key).get_data();

                    let val = alt_CStreamSyncedMetaDataChangeEvent_GetVal_CAPI_Heap(event);
                    let val = MValue::new((*val).ptr);

                    let old_val = alt_CStreamSyncedMetaDataChangeEvent_GetOldVal_CAPI_Heap(event);
                    let old_val = MValue::new((*old_val).ptr);

                    Some(CEvent::StreamSyncedMetaChange(
                        CStreamSyncedMetaChangeEvent::new(target, key, val, old_val),
                    ))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_GLOBAL_META_CHANGE => {
                    let event = event as *mut alt_CGlobalMetaDataChangeEvent;

                    let key = alt_CGlobalMetaDataChangeEvent_GetKey_CAPI_Heap(event);
                    let key = StringView::from(*key).get_data();

                    let val = alt_CGlobalMetaDataChangeEvent_GetVal_CAPI_Heap(event);
                    let val = MValue::new((*val).ptr);

                    let old_val = alt_CGlobalMetaDataChangeEvent_GetOldVal_CAPI_Heap(event);
                    let old_val = MValue::new((*old_val).ptr);

                    Some(CEvent::GlobalMetaChange(CGlobalMetaChangeEvent::new(
                        key, val, old_val,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_GLOBAL_SYNCED_META_CHANGE => {
                    let event = event as *mut alt_CGlobalSyncedMetaDataChangeEvent;

                    let key = alt_CGlobalSyncedMetaDataChangeEvent_GetKey_CAPI_Heap(event);
                    let key = StringView::from(*key).get_data();

                    let val = alt_CGlobalSyncedMetaDataChangeEvent_GetVal_CAPI_Heap(event);
                    let val = MValue::new((*val).ptr);

                    let old_val = alt_CGlobalSyncedMetaDataChangeEvent_GetOldVal_CAPI_Heap(event);
                    let old_val = MValue::new((*old_val).ptr);

                    Some(CEvent::GlobalSyncedMetaChange(
                        CGlobalSyncedMetaChangeEvent::new(key, val, old_val),
                    ))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_DAMAGE => {
                    let event = event as *mut alt_CPlayerDamageEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.players.get(&((*event).target.ptr as usize)).unwrap();

                    let attacker = match (*event).attacker.ptr.is_null() {
                        true => None,
                        false => Some(CoreApplication::get_entity(&alt, (*event).attacker.ptr)),
                    };

                    let damage = alt_CPlayerDamageEvent_GetDamage(event);
                    let weapon = alt_CPlayerDamageEvent_GetWeapon(event);

                    Some(CEvent::PlayerDamage(CPlayerDamageEvent::new(
                        *target, attacker, damage, weapon,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_DEATH => {
                    let event = event as *mut alt_CPlayerDeathEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.players.get(&((*event).target.ptr as usize)).unwrap();

                    let killer = match (*event).killer.ptr.is_null() {
                        true => None,
                        false => Some(CoreApplication::get_entity(&alt, (*event).killer.ptr)),
                    };

                    let weapon = alt_CPlayerDeathEvent_GetWeapon(event);

                    Some(CEvent::PlayerDeath(CPlayerDeathEvent::new(
                        *target, killer, weapon,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_EXPLOSION_EVENT => {
                    let event = event as *mut alt_CExplosionEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let source = alt.players.get(&((*event).source.ptr as usize)).unwrap();

                    let explosion_type = alt_CExplosionEvent_GetExplosionType(event) as u8;

                    let pos = alt_CExplosionEvent_GetPosition_CAPI_Heap(event);
                    let pos = Vector3::from(*pos);

                    let explosion_fx = alt_CExplosionEvent_GetExplosionFX(event);

                    Some(CEvent::ExplosionEvent(CExplosionEvent::new(
                        *source,
                        explosion_type,
                        pos,
                        explosion_fx,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_WEAPON_DAMAGE_EVENT => {
                    let event = event as *mut alt_CWeaponDamageEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let source = alt.players.get(&((*event).source.ptr as usize)).unwrap();

                    let target = match (*event).target.ptr.is_null() {
                        true => None,
                        false => Some(CoreApplication::get_entity(&alt, (*event).target.ptr)),
                    };

                    let weapon = alt_CWeaponDamageEvent_GetWeaponHash(event);
                    let damage = alt_CWeaponDamageEvent_GetDamageValue(event);

                    let shot_offset = alt_CWeaponDamageEvent_GetShotOffset_CAPI_Heap(event);
                    let shot_offset = Vector3::from(*shot_offset);

                    let body_part = alt_CWeaponDamageEvent_GetBodyPart(event) as u8;

                    Some(CEvent::WeaponDamageEvent(CWeaponDamageEvent::new(
                        *source,
                        target,
                        weapon,
                        damage,
                        shot_offset,
                        body_part,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_COLSHAPE_EVENT => {
                    let event = event as *mut alt_CColShapeEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt
                        .collision_shapes
                        .get(&((*event).target.ptr as usize))
                        .unwrap();
                    let entity = CoreApplication::get_entity(&alt, (*event).entity.ptr);
                    let state = alt_CColShapeEvent_GetState(event);

                    Some(CEvent::CollisionShapeEvent(CCollisionShapeEvent::new(
                        *target, entity, state,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_ENTER_VEHICLE => {
                    let event = event as *mut alt_CPlayerEnterVehicleEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.vehicles.get(&((*event).target.ptr as usize)).unwrap();
                    let player = alt.players.get(&((*event).player.ptr as usize)).unwrap();
                    let seat = alt_CPlayerEnterVehicleEvent_GetSeat(event);

                    Some(CEvent::PlayerEnterVehicle(CPlayerEnterVehicleEvent::new(
                        *target, *player, seat,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_LEAVE_VEHICLE => {
                    let event = event as *mut alt_CPlayerLeaveVehicleEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.vehicles.get(&((*event).target.ptr as usize)).unwrap();
                    let player = alt.players.get(&((*event).player.ptr as usize)).unwrap();
                    let seat = alt_CPlayerLeaveVehicleEvent_GetSeat(event);

                    Some(CEvent::PlayerLeaveVehicle(CPlayerLeaveVehicleEvent::new(
                        *target, *player, seat,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_CHANGE_VEHICLE_SEAT => {
                    let event = event as *mut alt_CPlayerChangeVehicleSeatEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = alt.vehicles.get(&((*event).target.ptr as usize)).unwrap();
                    let player = alt.players.get(&((*event).player.ptr as usize)).unwrap();
                    let old_seat = alt_CPlayerChangeVehicleSeatEvent_GetOldSeat(event);
                    let new_seat = alt_CPlayerChangeVehicleSeatEvent_GetNewSeat(event);

                    Some(CEvent::PlayerChangeVehicleSeat(
                        CPlayerChangeVehicleSeatEvent::new(*target, *player, old_seat, new_seat),
                    ))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_REMOVE_ENTITY_EVENT => {
                    let event = event as *mut alt_CRemoveEntityEvent;

                    let alt = self.world.read_resource::<AltResource>();
                    let target = CoreApplication::get_entity(&alt, (*event).target.ptr);

                    Some(CEvent::RemoveEntity(CRemoveEntityEvent::new(target)))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_DATA_NODE_RECEIVED_EVENT => {
                    let event = event as *mut alt_CDataNodeReceivedEvent;

                    let name = alt_CDataNodeReceivedEvent_GetName_CAPI_Heap(event);
                    let name = StringView::from(*name).get_data();

                    let json = alt_CDataNodeReceivedEvent_GetName_CAPI_Heap(event);
                    let json = StringView::from(*json).get_data();

                    Some(CEvent::DataNodeReceived(CDataNodeReceivedEvent::new(
                        name, json,
                    )))
                }
                alt_CEvent_Type::ALT_CEVENT_TYPE_CONSOLE_COMMAND_EVENT => {
                    let event = event as *mut alt_CConsoleCommandEvent;

                    let name = alt_CConsoleCommandEvent_GetName_CAPI_Heap(event);
                    let name = StringView::from(*name).get_data();

                    let args = alt_CConsoleCommandEvent_GetArgs(event);
                    let args = Vec::from(*args).iter().map(|a| a.get_data()).collect();

                    Some(CEvent::ConsoleCommand(CConsoleCommandEvent::new(
                        name, args,
                    )))
                }
                _ => None,
            };

            match ce {
                Some(ce) => {
                    dbg!("Pushing event to resource.");
                    self.state
                        .handle_event(StateData::new(&mut self.world, &mut self.data), ce);
                }
                None => {
                    altv_sdk::loge!(
                        "[Rust] Unknown event type {}.",
                        alt_CEvent_GetType(event) as i32
                    );
                }
            }
        }
    }

    pub fn create_game_object(&mut self, base_obj: *mut alt_IBaseObject) {
        unsafe {
            match alt_IBaseObject_GetType(base_obj) {
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_PLAYER => {
                    let player = alt_IBaseObject_to_alt_IPlayer(base_obj);
                    let entity = create_player(&mut self.world, player);

                    let mut alt = self.world.write_resource::<AltResource>();
                    alt.players.insert(player as usize, entity);
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_VEHICLE => {
                    let vehicle = alt_IBaseObject_to_alt_IVehicle(base_obj);
                    dbg!();
                    let entity = create_vehicle(&mut self.world, vehicle);
                    dbg!();

                    let mut alt = self.world.write_resource::<AltResource>();
                    dbg!();
                    alt.vehicles.insert(vehicle as usize, entity);
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_BLIP => {
                    let blip = alt_IBaseObject_to_alt_IBlip(base_obj);
                    dbg!(blip);
                    let entity = create_blip(&mut self.world, blip);

                    let mut alt = self.world.write_resource::<AltResource>();
                    alt.blips.insert(blip as usize, entity);
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_VOICE_CHANNEL => {
                    let voice_channel = alt_IBaseObject_to_alt_IVoiceChannel(base_obj);
                    let entity = create_voice_channel(&mut self.world, voice_channel);

                    let mut alt = self.world.write_resource::<AltResource>();
                    alt.voice_channels.insert(voice_channel as usize, entity);
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_COLSHAPE => {
                    let collision_shape = alt_IBaseObject_to_alt_IColShape(base_obj);
                    let entity = create_collision_shape(&mut self.world, collision_shape);

                    let mut alt = self.world.write_resource::<AltResource>();
                    alt.collision_shapes
                        .insert(collision_shape as usize, entity);
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_CHECKPOINT => {
                    let checkpoint = alt_IBaseObject_to_alt_ICheckpoint(base_obj);
                    let entity = create_checkpoint(&mut self.world, checkpoint);

                    let mut alt = self.world.write_resource::<AltResource>();
                    alt.checkpoints.insert(checkpoint as usize, entity);
                }
                _ => {}
            }
        }
    }

    pub fn remove_game_object(&mut self, base_obj: *mut alt_IBaseObject) {
        unsafe {
            match alt_IBaseObject_GetType(base_obj) {
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_PLAYER => {
                    let player = alt_IBaseObject_to_alt_IPlayer(base_obj);

                    let entity = {
                        let mut alt = self.world.write_resource::<AltResource>();
                        alt.players.remove(&(player as usize)).unwrap()
                    };

                    self.world.delete_entity(entity).unwrap();
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_VEHICLE => {
                    let vehicle = alt_IBaseObject_to_alt_IVehicle(base_obj);

                    let entity = {
                        let mut alt = self.world.write_resource::<AltResource>();
                        alt.vehicles.remove(&(vehicle as usize)).unwrap()
                    };

                    self.world.delete_entity(entity).unwrap();
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_BLIP => {
                    let blip = alt_IBaseObject_to_alt_IVehicle(base_obj);

                    let entity = {
                        let mut alt = self.world.write_resource::<AltResource>();
                        alt.blips.remove(&(blip as usize)).unwrap()
                    };

                    self.world.delete_entity(entity).unwrap();
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_VOICE_CHANNEL => {
                    let voice_channel = alt_IBaseObject_to_alt_IVehicle(base_obj);

                    let entity = {
                        let mut alt = self.world.write_resource::<AltResource>();
                        alt.voice_channels
                            .remove(&(voice_channel as usize))
                            .unwrap()
                    };

                    self.world.delete_entity(entity).unwrap();
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_COLSHAPE => {
                    let collision_shape = alt_IBaseObject_to_alt_IVehicle(base_obj);

                    let entity = {
                        let mut alt = self.world.write_resource::<AltResource>();
                        alt.collision_shapes
                            .remove(&(collision_shape as usize))
                            .unwrap()
                    };

                    self.world.delete_entity(entity).unwrap();
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_CHECKPOINT => {
                    let checkpoint = alt_IBaseObject_to_alt_ICheckpoint(base_obj);

                    let entity = {
                        let mut alt = self.world.write_resource::<AltResource>();
                        alt.checkpoints.remove(&(checkpoint as usize)).unwrap()
                    };

                    self.world.delete_entity(entity).unwrap();
                }
                _ => {}
            }
        }
    }

    fn get_entity(alt: &AltResource, entity: *mut alt_IEntity) -> Entity {
        unsafe {
            match alt_IEntity_GetType(entity) {
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_PLAYER => {
                    let player = alt_IEntity_to_alt_IPlayer(entity);
                    *alt.players.get(&(player as usize)).unwrap()
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_VEHICLE => {
                    let vehicle = alt_IEntity_to_alt_IVehicle(entity);
                    *alt.vehicles.get(&(vehicle as usize)).unwrap()
                }
                _ => panic!(),
            }
        }
    }
}

fn create_blip(world: &mut World, ptr: *mut alt_IBlip) -> Entity {
    unsafe {
        world
            .create_entity()
            .with(CRefCountable(AtomicPtr::new(
                alt_IBlip_to_alt_CRefCountable(ptr),
            )))
            .with(CBaseObject(AtomicPtr::new(alt_IBlip_to_alt_IBaseObject(
                ptr,
            ))))
            .with(CWorldObject(AtomicPtr::new(alt_IBlip_to_alt_IWorldObject(
                ptr,
            ))))
            .with(CBlip(AtomicPtr::new(ptr)))
            .build()
    }
}

fn create_collision_shape(world: &mut World, ptr: *mut alt_IColShape) -> Entity {
    unsafe {
        world
            .create_entity()
            .with(CRefCountable(AtomicPtr::new(
                alt_IColShape_to_alt_CRefCountable(ptr),
            )))
            .with(CBaseObject(AtomicPtr::new(
                alt_IColShape_to_alt_IBaseObject(ptr),
            )))
            .with(CWorldObject(AtomicPtr::new(
                alt_IColShape_to_alt_IWorldObject(ptr),
            )))
            .with(CCollisionShape(AtomicPtr::new(ptr)))
            .build()
    }
}

fn create_checkpoint(world: &mut World, ptr: *mut alt_ICheckpoint) -> Entity {
    unsafe {
        world
            .create_entity()
            .with(CRefCountable(AtomicPtr::new(
                alt_ICheckpoint_to_alt_CRefCountable(ptr),
            )))
            .with(CBaseObject(AtomicPtr::new(
                alt_ICheckpoint_to_alt_IBaseObject(ptr),
            )))
            .with(CWorldObject(AtomicPtr::new(
                alt_ICheckpoint_to_alt_IWorldObject(ptr),
            )))
            .with(CCollisionShape(AtomicPtr::new(
                alt_ICheckpoint_to_alt_IColShape(ptr),
            )))
            .with(CCheckpoint(AtomicPtr::new(ptr)))
            .build()
    }
}

pub fn create_player(world: &mut World, ptr: *mut alt_IPlayer) -> Entity {
    unsafe {
        world
            .create_entity()
            .with(CRefCountable(AtomicPtr::new(
                alt_IPlayer_to_alt_CRefCountable(ptr),
            )))
            .with(CBaseObject(AtomicPtr::new(alt_IPlayer_to_alt_IBaseObject(
                ptr,
            ))))
            .with(CWorldObject(AtomicPtr::new(
                alt_IPlayer_to_alt_IWorldObject(ptr),
            )))
            .with(CEntity(AtomicPtr::new(alt_IPlayer_to_alt_IEntity(ptr))))
            .with(CPlayer(AtomicPtr::new(ptr)))
            .build()
    }
}

fn create_vehicle(world: &mut World, ptr: *mut alt_IVehicle) -> Entity {
    unsafe {
        world
            .create_entity()
            .with(CRefCountable(AtomicPtr::new(
                alt_IVehicle_to_alt_CRefCountable(ptr),
            )))
            .with(CBaseObject(AtomicPtr::new(
                alt_IVehicle_to_alt_IBaseObject(ptr),
            )))
            .with(CWorldObject(AtomicPtr::new(
                alt_IVehicle_to_alt_IWorldObject(ptr),
            )))
            .with(CEntity(AtomicPtr::new(alt_IVehicle_to_alt_IEntity(ptr))))
            .with(CVehicle(AtomicPtr::new(ptr)))
            .build()
    }
}

fn create_voice_channel(world: &mut World, ptr: *mut alt_IVoiceChannel) -> Entity {
    unsafe {
        world
            .create_entity()
            .with(CRefCountable(AtomicPtr::new(
                alt_IVoiceChannel_to_alt_CRefCountable(ptr),
            )))
            .with(CBaseObject(AtomicPtr::new(
                alt_IVoiceChannel_to_alt_IBaseObject(ptr),
            )))
            .build()
    }
}
