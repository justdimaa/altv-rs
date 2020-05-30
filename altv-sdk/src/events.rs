use crate::mvalue::MValue;
use crate::natives::alt_CEvent_Type;
use crate::vector::Vector3;
use altv_core::ecs::Entity;

pub enum CEvent {
    None,
    PlayerConnect(CPlayerConnectEvent),
    PlayerDisconnect(CPlayerDisconnectEvent),
    // ResourceStart(CResourceStartEvent),
    // ResourceStop(CResourceStopEvent),
    // ResourceError(CResourceErrorEvent),
    ServerScript(CServerScriptEvent),
    ClientScript(CClientScriptEvent),
    // MetaChange(CMetaChangeEvent),
    SyncedMetaChange(CSyncedMetaChangeEvent),
    StreamSyncedMetaChange(CStreamSyncedMetaChangeEvent),
    GlobalMetaChange(CGlobalMetaChangeEvent),
    GlobalSyncedMetaChange(CGlobalSyncedMetaChangeEvent),
    PlayerDamage(CPlayerDamageEvent),
    PlayerDeath(CPlayerDeathEvent),
    // FireEvent(CFireEvent),
    ExplosionEvent(CExplosionEvent),
    WeaponDamageEvent(CWeaponDamageEvent),
    // VehicleDestroyEvent(CVehicleDestroyEvent),
    // CheckpointEvent(CCheckpointEvent),
    CollisionShapeEvent(CCollisionShapeEvent),
    PlayerEnterVehicle(CPlayerEnterVehicleEvent),
    PlayerLeaveVehicle(CPlayerLeaveVehicleEvent),
    PlayerChangeVehicleSeat(CPlayerChangeVehicleSeatEvent),
    RemoveEntity(CRemoveEntityEvent),
    DataNodeReceived(CDataNodeReceivedEvent),
    ConsoleCommand(CConsoleCommandEvent),
}

impl CEvent {
    pub fn get_type(&self) -> u32 {
        (match *self {
            CEvent::None => alt_CEvent_Type::ALT_CEVENT_TYPE_NONE,
            // CEvent::ResourceStart(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_RESOURCE_START,
            // CEvent::ResourceStop(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_RESOURCE_STOP,
            // CEvent::ResourceError(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_RESOURCE_ERROR,
            CEvent::PlayerConnect(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_CONNECT,
            CEvent::PlayerDisconnect(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_DISCONNECT,
            CEvent::ServerScript(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_SERVER_SCRIPT_EVENT,
            CEvent::ClientScript(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_CLIENT_SCRIPT_EVENT,
            // CEvent::MetaChange(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_META_CHANGE,
            CEvent::SyncedMetaChange(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_SYNCED_META_CHANGE,
            CEvent::StreamSyncedMetaChange(_) => {
                alt_CEvent_Type::ALT_CEVENT_TYPE_STREAM_SYNCED_META_CHANGE
            }
            CEvent::GlobalMetaChange(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_GLOBAL_META_CHANGE,
            CEvent::GlobalSyncedMetaChange(_) => {
                alt_CEvent_Type::ALT_CEVENT_TYPE_GLOBAL_SYNCED_META_CHANGE
            }
            CEvent::PlayerDamage(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_DAMAGE,
            CEvent::PlayerDeath(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_DEATH,
            // CEvent::FireEvent(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_FIRE_EVENT,
            CEvent::ExplosionEvent(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_EXPLOSION_EVENT,
            CEvent::WeaponDamageEvent(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_WEAPON_DAMAGE_EVENT,
            // CEvent::VehicleDestroyEvent(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_VEHICLE_DESTROY,
            // CEvent::CheckpointEvent(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_CHECKPOINT_EVENT,
            CEvent::CollisionShapeEvent(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_COLSHAPE_EVENT,
            CEvent::PlayerEnterVehicle(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_ENTER_VEHICLE,
            CEvent::PlayerLeaveVehicle(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_LEAVE_VEHICLE,
            CEvent::PlayerChangeVehicleSeat(_) => {
                alt_CEvent_Type::ALT_CEVENT_TYPE_PLAYER_CHANGE_VEHICLE_SEAT
            }
            CEvent::RemoveEntity(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_REMOVE_ENTITY_EVENT,
            CEvent::DataNodeReceived(_) => {
                alt_CEvent_Type::ALT_CEVENT_TYPE_DATA_NODE_RECEIVED_EVENT
            }
            CEvent::ConsoleCommand(_) => alt_CEvent_Type::ALT_CEVENT_TYPE_CONSOLE_COMMAND_EVENT,
        }) as u32
    }
}

pub struct CPlayerConnectEvent {
    target: Entity,
    reason: String,
}

impl CPlayerConnectEvent {
    pub fn new(target: Entity, reason: String) -> Self {
        CPlayerConnectEvent { target, reason }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_reason(&self) -> &str {
        &self.reason
    }
}

pub struct CPlayerDisconnectEvent {
    target: Entity,
    reason: String,
}

impl CPlayerDisconnectEvent {
    pub fn new(target: Entity, reason: String) -> Self {
        CPlayerDisconnectEvent { target, reason }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_reason(&self) -> &str {
        &self.reason
    }
}

pub struct CServerScriptEvent {
    name: String,
    args: Vec<MValue>,
}

impl CServerScriptEvent {
    pub fn new(name: String, args: Vec<MValue>) -> Self {
        CServerScriptEvent { name, args }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<MValue> {
        &self.args
    }
}

pub struct CClientScriptEvent {
    target: Entity,
    name: String,
    args: Vec<MValue>,
}

impl CClientScriptEvent {
    pub fn new(target: Entity, name: String, args: Vec<MValue>) -> Self {
        CClientScriptEvent { target, name, args }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<MValue> {
        &self.args
    }
}

pub struct CSyncedMetaChangeEvent {
    target: Entity,
    key: String,
    value: MValue,
    old_value: MValue,
}

impl CSyncedMetaChangeEvent {
    pub fn new(target: Entity, key: String, value: MValue, old_value: MValue) -> Self {
        CSyncedMetaChangeEvent {
            target,
            key,
            value,
            old_value,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &MValue {
        &self.value
    }

    pub fn get_old_value(&self) -> &MValue {
        &self.old_value
    }
}

pub struct CStreamSyncedMetaChangeEvent {
    target: Entity,
    key: String,
    value: MValue,
    old_value: MValue,
}

impl CStreamSyncedMetaChangeEvent {
    pub fn new(target: Entity, key: String, value: MValue, old_value: MValue) -> Self {
        CStreamSyncedMetaChangeEvent {
            target,
            key,
            value,
            old_value,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &MValue {
        &self.value
    }

    pub fn get_old_value(&self) -> &MValue {
        &self.old_value
    }
}

pub struct CGlobalMetaChangeEvent {
    key: String,
    value: MValue,
    old_value: MValue,
}

impl CGlobalMetaChangeEvent {
    pub fn new(key: String, value: MValue, old_value: MValue) -> Self {
        CGlobalMetaChangeEvent {
            key,
            value,
            old_value,
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &MValue {
        &self.value
    }

    pub fn get_old_value(&self) -> &MValue {
        &self.old_value
    }
}

pub struct CGlobalSyncedMetaChangeEvent {
    key: String,
    value: MValue,
    old_value: MValue,
}

impl CGlobalSyncedMetaChangeEvent {
    pub fn new(key: String, value: MValue, old_value: MValue) -> Self {
        CGlobalSyncedMetaChangeEvent {
            key,
            value,
            old_value,
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &MValue {
        &self.value
    }

    pub fn get_old_value(&self) -> &MValue {
        &self.old_value
    }
}

pub struct CPlayerDamageEvent {
    target: Entity,
    attacker: Option<Entity>,
    damage: u16,
    weapon: u32,
}

impl CPlayerDamageEvent {
    pub fn new(target: Entity, attacker: Option<Entity>, damage: u16, weapon: u32) -> Self {
        CPlayerDamageEvent {
            target,
            attacker,
            damage,
            weapon,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_attacker(&self) -> Option<Entity> {
        self.attacker
    }

    pub fn get_damage(&self) -> u16 {
        self.damage
    }

    pub fn get_weapon(&self) -> u32 {
        self.weapon
    }
}

pub struct CPlayerDeathEvent {
    target: Entity,
    killer: Option<Entity>,
    weapon: u32,
}

impl CPlayerDeathEvent {
    pub fn new(target: Entity, killer: Option<Entity>, weapon: u32) -> Self {
        CPlayerDeathEvent {
            target,
            killer,
            weapon,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_killer(&self) -> Option<Entity> {
        self.killer
    }

    pub fn get_weapon(&self) -> u32 {
        self.weapon
    }
}

pub struct CExplosionEvent {
    source: Entity,
    explosion_type: u8,
    position: Vector3,
    explosion_fx: u32,
}

impl CExplosionEvent {
    pub fn new(source: Entity, explosion_type: u8, position: Vector3, explosion_fx: u32) -> Self {
        CExplosionEvent {
            source,
            explosion_type,
            position,
            explosion_fx,
        }
    }

    pub fn get_source(&self) -> Entity {
        self.source
    }

    pub fn get_killer(&self) -> u8 {
        self.explosion_type
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    pub fn get_explosion_fx(&self) -> u32 {
        self.explosion_fx
    }
}

pub struct CWeaponDamageEvent {
    source: Entity,
    target: Option<Entity>,
    weapon: u32,
    damage: u16,
    shot_offset: Vector3,
    body_part: u8,
}

impl CWeaponDamageEvent {
    pub fn new(
        source: Entity,
        target: Option<Entity>,
        weapon: u32,
        damage: u16,
        shot_offset: Vector3,
        body_part: u8,
    ) -> Self {
        CWeaponDamageEvent {
            source,
            target,
            weapon,
            damage,
            shot_offset,
            body_part,
        }
    }

    pub fn get_source(&self) -> Entity {
        self.source
    }

    pub fn get_target(&self) -> Option<Entity> {
        self.target
    }

    pub fn get_weapon(&self) -> u32 {
        self.weapon
    }

    pub fn get_damage(&self) -> u16 {
        self.damage
    }

    pub fn get_shot_offset(&self) -> Vector3 {
        self.shot_offset
    }

    pub fn get_body_part(&self) -> u8 {
        self.body_part
    }
}

pub struct CCollisionShapeEvent {
    target: Entity,
    entity: Entity,
    state: bool,
}

impl CCollisionShapeEvent {
    pub fn new(target: Entity, entity: Entity, state: bool) -> Self {
        CCollisionShapeEvent {
            target,
            entity,
            state,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn get_state(&self) -> bool {
        self.state
    }
}

pub struct CPlayerEnterVehicleEvent {
    target: Entity,
    player: Entity,
    seat: u8,
}

impl CPlayerEnterVehicleEvent {
    pub fn new(target: Entity, player: Entity, seat: u8) -> Self {
        CPlayerEnterVehicleEvent {
            target,
            player,
            seat,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_player(&self) -> Entity {
        self.player
    }

    pub fn get_seat(&self) -> u8 {
        self.seat
    }
}

pub struct CPlayerLeaveVehicleEvent {
    target: Entity,
    player: Entity,
    seat: u8,
}

impl CPlayerLeaveVehicleEvent {
    pub fn new(target: Entity, player: Entity, seat: u8) -> Self {
        CPlayerLeaveVehicleEvent {
            target,
            player,
            seat,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_player(&self) -> Entity {
        self.player
    }

    pub fn get_seat(&self) -> u8 {
        self.seat
    }
}

pub struct CPlayerChangeVehicleSeatEvent {
    target: Entity,
    player: Entity,
    old_seat: u8,
    new_seat: u8,
}

impl CPlayerChangeVehicleSeatEvent {
    pub fn new(target: Entity, player: Entity, old_seat: u8, new_seat: u8) -> Self {
        CPlayerChangeVehicleSeatEvent {
            target,
            player,
            old_seat,
            new_seat,
        }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }

    pub fn get_player(&self) -> Entity {
        self.player
    }

    pub fn get_old_seat(&self) -> u8 {
        self.old_seat
    }

    pub fn get_new_seat(&self) -> u8 {
        self.new_seat
    }
}

pub struct CRemoveEntityEvent {
    target: Entity,
}

impl CRemoveEntityEvent {
    pub fn new(target: Entity) -> Self {
        CRemoveEntityEvent { target }
    }

    pub fn get_target(&self) -> Entity {
        self.target
    }
}

pub struct CDataNodeReceivedEvent {
    name: String,
    json: String,
}

impl CDataNodeReceivedEvent {
    pub fn new(name: String, json: String) -> Self {
        CDataNodeReceivedEvent { name, json }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_json(&self) -> &str {
        &self.json
    }
}

pub struct CConsoleCommandEvent {
    name: String,
    args: Vec<String>,
}

impl CConsoleCommandEvent {
    pub fn new(name: String, args: Vec<String>) -> Self {
        CConsoleCommandEvent { name, args }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}
