use crate::mvalue::MValue;
use crate::natives::*;
use crate::rgba::Rgba;
use crate::string_view::StringView;
use crate::vector::{Rotation3, Vector3};
use altv_core::ecs::{Component, Entity, ReadStorage, VecStorage, World, WorldExt};
use altv_core::AltResource;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::Duration;

pub fn create_vehicle(
    world: &World,
    model: u32,
    position: Vector3,
    rotation: Rotation3,
) -> Option<Entity> {
    let rotation: alt_RotationLayout = rotation.into();

    unsafe {
        let core = alt_ICore_Instance();
        let veh = alt_ICore_CreateVehicle_CAPI_Heap(
            core,
            model,
            Box::into_raw(Box::new(position.into())),
            Box::into_raw(Box::new(rotation)) as *mut alt_Vector_float_3_RotationLayout,
        );

        if (*veh).ptr.is_null() {
            return None;
        }

        let alt = world.read_resource::<AltResource>();
        Some(*alt.vehicles.get(&((*veh).ptr as usize)).unwrap())
    }
}

pub fn create_collision_shape_sphere(world: &World, position: Vector3, radius: f32) -> Entity {
    unsafe {
        let core = alt_ICore_Instance();
        let cs = alt_ICore_CreateColShapeSphere_CAPI_Heap(
            core,
            Box::into_raw(Box::new(position.into())),
            radius,
        );

        let alt = world.read_resource::<AltResource>();
        *alt.collision_shapes.get(&((*cs).ptr as usize)).unwrap()
    }
}

pub fn create_collision_shape_cube(
    world: &World,
    start_position: Vector3,
    end_position: Vector3,
) -> Entity {
    unsafe {
        let core = alt_ICore_Instance();
        let cs = alt_ICore_CreateColShapeCube_CAPI_Heap(
            core,
            Box::into_raw(Box::new(start_position.into())),
            Box::into_raw(Box::new(end_position.into())),
        );

        let alt = world.read_resource::<AltResource>();
        *alt.collision_shapes.get(&((*cs).ptr as usize)).unwrap()
    }
}

pub fn create_collision_shape_rectangle(
    world: &World,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    z: f32,
) -> Entity {
    unsafe {
        let core = alt_ICore_Instance();
        let cs = alt_ICore_CreateColShapeRectangle_CAPI_Heap(core, x1, y1, x2, y2, z);

        let alt = world.read_resource::<AltResource>();
        *alt.collision_shapes.get(&((*cs).ptr as usize)).unwrap()
    }
}

pub fn create_collision_shape_circle(world: &World, position: Vector3, radius: f32) -> Entity {
    unsafe {
        let core = alt_ICore_Instance();
        let cs = alt_ICore_CreateColShapeCircle_CAPI_Heap(
            core,
            Box::into_raw(Box::new(position.into())),
            radius,
        );

        let alt = world.read_resource::<AltResource>();
        *alt.collision_shapes.get(&((*cs).ptr as usize)).unwrap()
    }
}

pub fn create_collision_shape_cylinder(
    world: &World,
    position: Vector3,
    radius: f32,
    height: f32,
) -> Entity {
    unsafe {
        let core = alt_ICore_Instance();
        let cs = alt_ICore_CreateColShapeCylinder_CAPI_Heap(
            core,
            Box::into_raw(Box::new(position.into())),
            radius,
            height,
        );

        let alt = world.read_resource::<AltResource>();
        *alt.collision_shapes.get(&((*cs).ptr as usize)).unwrap()
    }
}

// pub fn create_checkpoint(
//     world: &World,
//     cp_type: u8,
//     position: Vector3,
//     radius: f32,
//     height: f32,
//     color: Rgba,
// ) -> Entity {
//     let position: alt_Vector_float_3_PointLayout = position.into();
//     let color: alt_RGBA = color.into();
//
//     unsafe {
//         let core = alt_ICore_Instance();
//         let cp = alt_ICore_CreateCheckpoint_CAPI_Heap(
//             core,
//             cp_type,
//             Box::into_raw(Box::new(position)),
//             radius,
//             height,
//             Box::into_raw(Box::new(color)),
//         );
//
//         let alt = world.read_resource::<AltResource>();
//         *alt.checkpoints
//             .get(&((*cp).ptr as usize))
//             .unwrap()
//     }
// }

// pub fn create_blip(world: &World, blip_type: u8, position: Vector3) -> Entity {
//     unsafe {
//         let core = alt_ICore_Instance();
//         let blip = alt_ICore_CreateBlip_CAPI_Heap(
//             core,
//             alt_RefBase_RefStore_IPlayer_Create_3_CAPI_Heap(),
//             alt_IBlip_BlipType::ALT_IBLIP_BLIPTYPE_OBJECT,
//             Box::into_raw(Box::new(position.into())),
//         );
//
//         let alt = world.read_resource::<AltResource>();
//         *alt.blips.get(&((*blip).ptr as usize)).unwrap()
//     }
// }

// pub fn create_voice_channel(world: &World, spacial: bool, max_distance: f32) -> Entity {
//     unsafe {
//         let core = alt_ICore_Instance();
//         let voice_channel = alt_ICore_CreateVoiceChannel_CAPI_Heap(core, spacial, max_distance);
//
//         let alt = world.read_resource::<AltResource>();
//         *alt.voice_channels.get(&((*voice_channel).ptr as usize)).unwrap()
//     }
// }

pub fn delete(world: &mut World, entity: Entity) {
    let ptr = world.exec(|cbase_objs: ReadStorage<CBaseObject>| {
        let cbase_obj = cbase_objs.get(entity).unwrap();
        cbase_obj.0.load(Ordering::Relaxed)
    });

    unsafe {
        let core = alt_ICore_Instance();
        alt_ICore_DestroyBaseObject(
            core,
            alt_RefBase_RefStore_IBaseObject_Create_2_CAPI_Heap(ptr),
        );
    }
}

pub struct CRefCountable(pub AtomicPtr<alt_CRefCountable>);

impl CRefCountable {
    pub fn get_ref_count(&self) -> u64 {
        unsafe { alt_CRefCountable_GetRefCount(self.0.load(Ordering::Relaxed)) }
    }

    pub fn add_ref(&mut self) {
        unsafe { alt_CRefCountable_AddRef(self.0.load(Ordering::Relaxed)) }
    }

    pub fn remove_ref(&mut self) {
        unsafe { alt_CRefCountable_RemoveRef(self.0.load(Ordering::Relaxed)) }
    }
}

impl Component for CRefCountable {
    type Storage = VecStorage<Self>;
}

pub struct CBaseObject(pub AtomicPtr<alt_IBaseObject>);

impl CBaseObject {
    pub fn has_meta_data(&self, key: &str) -> bool {
        unsafe {
            alt_IBaseObject_HasMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            )
        }
    }

    pub fn get_meta_data(&self, key: &str) -> MValue {
        unsafe {
            let val = alt_IBaseObject_GetMetaData_CAPI_Heap(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            );
            MValue::new((*val).ptr).into()
        }
    }

    pub fn set_meta_data(&mut self, key: &str, value: MValue) {
        unsafe {
            alt_IBaseObject_SetMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
                value.into(),
            )
        }
    }

    pub fn delete_meta_data(&mut self, key: &str) {
        unsafe {
            alt_IBaseObject_DeleteMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            )
        }
    }
}

impl Component for CBaseObject {
    type Storage = VecStorage<Self>;
}

pub struct CWorldObject(pub AtomicPtr<alt_IWorldObject>);

impl CWorldObject {
    pub fn get_dimension(&self) -> i32 {
        unsafe { alt_IWorldObject_GetDimension(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_dimension(&mut self, dimension: i32) {
        unsafe { alt_IWorldObject_SetDimension(self.0.load(Ordering::Relaxed), dimension) }
    }

    pub fn get_position(&self) -> Vector3 {
        unsafe {
            let pos = alt_IWorldObject_GetPosition_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Vector3::from(*pos)
        }
    }

    pub fn set_position(&mut self, pos: Vector3) {
        unsafe {
            alt_IWorldObject_SetPosition(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(pos.into())),
            )
        }
    }
}

impl Component for CWorldObject {
    type Storage = VecStorage<Self>;
}

pub struct CEntity(pub AtomicPtr<alt_IEntity>);

impl CEntity {
    pub fn get_id(&self) -> u16 {
        unsafe { alt_IEntity_GetID(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_network_owner(&self, alt: &AltResource) -> Option<Entity> {
        unsafe {
            let player = alt_IEntity_GetNetworkOwner_CAPI_Heap(self.0.load(Ordering::Relaxed));
            let player = (*player).ptr;

            if player.is_null() {
                return None;
            }

            Some(*alt.players.get(&(player as usize)).unwrap())
        }
    }

    pub fn get_rotation(&self) -> Rotation3 {
        unsafe {
            let rot = alt_IEntity_GetRotation_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Rotation3::from(*(rot as *mut alt_RotationLayout))
        }
    }

    pub fn set_rotation(&mut self, rot: Rotation3) {
        unsafe {
            let rot: alt_RotationLayout = rot.into();
            alt_IEntity_SetRotation(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(rot)) as *mut alt_Vector_float_3_RotationLayout,
            )
        }
    }

    pub fn has_synced_meta_data(&self, key: &str) -> bool {
        unsafe {
            alt_IEntity_HasSyncedMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            )
        }
    }

    pub fn get_synced_meta_data(&self, key: &str) -> MValue {
        unsafe {
            let val = alt_IEntity_GetSyncedMetaData_CAPI_Heap(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            );
            MValue::new((*val).ptr).into()
        }
    }

    pub fn has_stream_synced_meta_data(&self, key: &str) -> bool {
        unsafe {
            alt_IEntity_HasStreamSyncedMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            )
        }
    }

    pub fn get_stream_synced_meta_data(&self, key: &str) -> MValue {
        unsafe {
            let val = alt_IEntity_GetStreamSyncedMetaData_CAPI_Heap(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            );
            MValue::new((*val).ptr).into()
        }
    }

    pub fn set_synced_meta_data(&mut self, key: &str, value: MValue) {
        unsafe {
            alt_IEntity_SetSyncedMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
                value.into(),
            )
        }
    }

    pub fn delete_synced_meta_data(&mut self, key: &str) {
        unsafe {
            alt_IEntity_DeleteSyncedMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            )
        }
    }

    pub fn set_stream_synced_meta_data(&mut self, key: &str, value: MValue) {
        unsafe {
            alt_IEntity_SetStreamSyncedMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
                value.into(),
            )
        }
    }

    pub fn delete_stream_synced_meta_data(&mut self, key: &str) {
        unsafe {
            alt_IEntity_DeleteStreamSyncedMetaData(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(key).into())),
            )
        }
    }

    pub fn get_model(&self) -> u32 {
        unsafe { alt_IEntity_GetModel(self.0.load(Ordering::Relaxed)) }
    }
}

impl Component for CEntity {
    type Storage = VecStorage<Self>;
}

pub struct CPlayer(pub AtomicPtr<alt_IPlayer>);

impl CPlayer {
    pub fn is_connected(&self) -> bool {
        unsafe { alt_IPlayer_IsConnected(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_ping(&self) -> u32 {
        unsafe { alt_IPlayer_GetPing(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_ip(&self) -> String {
        unsafe {
            let ip = alt_IPlayer_GetIP_CAPI_Heap(self.0.load(Ordering::Relaxed));
            StringView::from(*ip).get_data()
        }
    }

    pub fn spawn(&mut self, pos: Vector3, delay: Duration) {
        unsafe {
            alt_IPlayer_Spawn(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(pos.into())),
                delay.as_millis() as u32,
            )
        }
    }

    pub fn despawn(&mut self) {
        unsafe { alt_IPlayer_Despawn(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let name = alt_IPlayer_GetName_CAPI_Heap(self.0.load(Ordering::Relaxed));
            StringView::from(*name).get_data()
        }
    }

    pub fn get_social_id(&self) -> u64 {
        unsafe { alt_IPlayer_GetSocialID(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_hwid_hash(&self) -> u64 {
        unsafe { alt_IPlayer_GetHwidHash(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_hwid_ex_hash(&self) -> u64 {
        unsafe { alt_IPlayer_GetHwidExHash(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_auth_token(&self) -> String {
        unsafe {
            let at = alt_IPlayer_GetAuthToken_CAPI_Heap(self.0.load(Ordering::Relaxed));
            StringView::from(*at).get_data()
        }
    }

    pub fn get_health(&self) -> u16 {
        unsafe { alt_IPlayer_GetHealth(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_health(&mut self, health: u16) {
        unsafe { alt_IPlayer_SetHealth(self.0.load(Ordering::Relaxed), health) }
    }

    pub fn get_max_health(&self) -> u16 {
        unsafe { alt_IPlayer_GetMaxHealth(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_max_health(&mut self, max_health: u16) {
        unsafe { alt_IPlayer_SetMaxHealth(self.0.load(Ordering::Relaxed), max_health) }
    }

    pub fn set_date_time(
        &mut self,
        day: i32,
        month: i32,
        year: i32,
        hour: i32,
        minute: i32,
        second: i32,
    ) {
        unsafe {
            alt_IPlayer_SetDateTime(
                self.0.load(Ordering::Relaxed),
                day,
                month,
                year,
                hour,
                minute,
                second,
            )
        }
    }

    pub fn set_weather(&mut self, weather: u32) {
        unsafe { alt_IPlayer_SetWeather(self.0.load(Ordering::Relaxed), weather) }
    }

    pub fn give_weapon(&mut self, weapon: u32, ammo: i32, select: bool) {
        unsafe { alt_IPlayer_GiveWeapon(self.0.load(Ordering::Relaxed), weapon, ammo, select) }
    }

    pub fn remove_weapon(&mut self, weapon: u32) {
        unsafe { alt_IPlayer_RemoveWeapon(self.0.load(Ordering::Relaxed), weapon) }
    }

    pub fn remove_all_weapons(&mut self) {
        unsafe { alt_IPlayer_RemoveAllWeapons(self.0.load(Ordering::Relaxed)) }
    }

    pub fn add_weapon_component(&mut self, weapon: u32, component: u32) {
        unsafe { alt_IPlayer_AddWeaponComponent(self.0.load(Ordering::Relaxed), weapon, component) }
    }

    pub fn remove_weapon_component(&mut self, weapon: u32, component: u32) {
        unsafe {
            alt_IPlayer_RemoveWeaponComponent(self.0.load(Ordering::Relaxed), weapon, component)
        }
    }

    // pub fn get_current_weapon_components(&self) -> Vec<u32> {
    //     unimplemented!()
    // }

    pub fn set_weapon_tint_index(&mut self, weapon: u32, tint_index: u8) {
        unsafe {
            alt_IPlayer_SetWeaponTintIndex(self.0.load(Ordering::Relaxed), weapon, tint_index)
        }
    }

    pub fn get_current_weapon_tint_index(&self) -> u8 {
        unsafe { alt_IPlayer_GetCurrentWeaponTintIndex(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_current_weapon(&self) -> u32 {
        unsafe { alt_IPlayer_GetCurrentWeapon(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_current_weapon(&mut self, weapon: u32) {
        unsafe { alt_IPlayer_SetCurrentWeapon(self.0.load(Ordering::Relaxed), weapon) }
    }

    pub fn is_dead(&self) -> bool {
        unsafe { alt_IPlayer_IsDead(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_jumping(&self) -> bool {
        unsafe { alt_IPlayer_IsJumping(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_in_ragdoll(&self) -> bool {
        unsafe { alt_IPlayer_IsInRagdoll(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_aiming(&self) -> bool {
        unsafe { alt_IPlayer_IsAiming(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_shooting(&self) -> bool {
        unsafe { alt_IPlayer_IsShooting(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_reloading(&self) -> bool {
        unsafe { alt_IPlayer_IsReloading(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_armor(&self) -> u16 {
        unsafe { alt_IPlayer_GetArmour(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_armor(&mut self, armor: u16) {
        unsafe { alt_IPlayer_SetArmour(self.0.load(Ordering::Relaxed), armor) }
    }

    pub fn get_max_armor(&self) -> u16 {
        unsafe { alt_IPlayer_GetMaxArmour(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_max_armor(&mut self, max_armor: u16) {
        unsafe { alt_IPlayer_SetMaxArmour(self.0.load(Ordering::Relaxed), max_armor) }
    }

    pub fn get_move_speed(&self) -> f32 {
        unsafe { alt_IPlayer_GetMoveSpeed(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_weapon(&self) -> u32 {
        unsafe { alt_IPlayer_GetWeapon(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_ammo(&self) -> u16 {
        unsafe { alt_IPlayer_GetAmmo(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_aim_position(&self) -> Vector3 {
        unsafe {
            let pos = alt_IPlayer_GetAimPos_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Vector3::from(*pos)
        }
    }

    pub fn get_head_rotation(&self) -> Rotation3 {
        unsafe {
            let rot = alt_IPlayer_GetHeadRotation_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Rotation3::from(*(rot as *mut alt_RotationLayout))
        }
    }

    pub fn is_in_vehicle(&self) -> bool {
        unsafe { alt_IPlayer_IsInVehicle(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_vehicle(&self, alt: &AltResource) -> Option<Entity> {
        unsafe {
            let vehicle = alt_IPlayer_GetVehicle_CAPI_Heap(self.0.load(Ordering::Relaxed));
            let vehicle = (*vehicle).ptr;

            if vehicle.is_null() {
                return None;
            }

            Some(*alt.vehicles.get(&(vehicle as usize)).unwrap())
        }
    }

    pub fn get_seat(&self) -> u8 {
        unsafe { alt_IPlayer_GetSeat(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_entity_aiming_at(&self, alt: &AltResource) -> Option<Entity> {
        unsafe {
            let entity = alt_IPlayer_GetEntityAimingAt_CAPI_Heap(self.0.load(Ordering::Relaxed));
            let entity = (*entity).ptr;

            if entity.is_null() {
                return None;
            }

            match alt_IEntity_GetType(entity) {
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_PLAYER => {
                    let player = alt_IEntity_to_alt_IPlayer(entity);
                    Some(*alt.players.get(&(player as usize)).unwrap())
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_VEHICLE => {
                    let vehicle = alt_IEntity_to_alt_IVehicle(entity);
                    Some(*alt.vehicles.get(&(vehicle as usize)).unwrap())
                }
                _ => panic!(),
            }
        }
    }

    pub fn get_entity_aim_offset(&self) -> Vector3 {
        unsafe {
            let offset = alt_IPlayer_GetEntityAimOffset_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Vector3::from(*offset)
        }
    }

    pub fn is_flashlight_active(&self) -> bool {
        unsafe { alt_IPlayer_IsFlashlightActive(self.0.load(Ordering::Relaxed)) }
    }

    pub fn kick(&mut self, reason: &str) {
        unsafe {
            alt_IPlayer_Kick(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(reason).into())),
            )
        }
    }

    pub fn set_model(&mut self, model: u32) {
        unsafe { alt_IPlayer_SetModel(self.0.load(Ordering::Relaxed), model) }
    }

    pub fn emit(&mut self, event_name: &str, args: &[MValue]) {
        unsafe {
            let core = alt_ICore_Instance();
            alt_ICore_TriggerClientEvent(
                core,
                alt_RefBase_RefStore_IPlayer_Create_4_CAPI_Heap(self.0.load(Ordering::Relaxed)),
                Box::into_raw(Box::new(StringView::new(event_name).into())),
                crate::array::convert_iter_to_array_mvalue(args.iter()),
            )
        }
    }
}

impl Component for CPlayer {
    type Storage = VecStorage<Self>;
}

pub struct CVehicle(pub AtomicPtr<alt_IVehicle>);

impl CVehicle {
    pub fn get_driver(&self, alt: &AltResource) -> Option<Entity> {
        unsafe {
            let player = alt_IVehicle_GetDriver_CAPI_Heap(self.0.load(Ordering::Relaxed));
            let player = (*player).ptr;

            if player.is_null() {
                return None;
            }

            Some(*alt.players.get(&(player as usize)).unwrap())
        }
    }

    pub fn get_mod(&self, category: u8) -> u8 {
        unsafe { alt_IVehicle_GetMod(self.0.load(Ordering::Relaxed), category) }
    }

    pub fn get_mods_count(&self, category: u8) -> u8 {
        unsafe { alt_IVehicle_GetModsCount(self.0.load(Ordering::Relaxed), category) }
    }

    pub fn set_mod(&self, category: u8, id: u8) -> bool {
        unsafe { alt_IVehicle_SetMod(self.0.load(Ordering::Relaxed), category, id) }
    }

    pub fn get_mod_kit(&self) -> u8 {
        unsafe { alt_IVehicle_GetModKit(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_mod_kits_count(&self) -> u8 {
        unsafe { alt_IVehicle_GetModKitsCount(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_mod_kit(&mut self, id: u8) -> bool {
        unsafe { alt_IVehicle_SetModKit(self.0.load(Ordering::Relaxed), id) }
    }

    pub fn is_primary_color_rgb(&self) -> bool {
        unsafe { alt_IVehicle_IsPrimaryColorRGB(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_primary_color(&self) -> u8 {
        unsafe { alt_IVehicle_GetPrimaryColor(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_primary_color_rgb(&self) -> Rgba {
        unsafe {
            let c = alt_IVehicle_GetPrimaryColorRGB_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Rgba::from(*c)
        }
    }

    pub fn set_primary_color(&mut self, primary_color: u8) {
        unsafe { alt_IVehicle_SetPrimaryColor(self.0.load(Ordering::Relaxed), primary_color) }
    }

    pub fn set_primary_color_rgb(&mut self, primary_color_rgb: Rgba) {
        unsafe {
            alt_IVehicle_SetPrimaryColorRGB(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(primary_color_rgb.into())),
            )
        }
    }

    pub fn is_secondary_color_rgb(&self) -> bool {
        unsafe { alt_IVehicle_IsSecondaryColorRGB(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_secondary_color(&self) -> u8 {
        unsafe { alt_IVehicle_GetSecondaryColor(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_secondary_color_rgb(&self) -> Rgba {
        unsafe {
            let c = alt_IVehicle_GetSecondaryColorRGB_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Rgba::from(*c)
        }
    }

    pub fn set_secondary_color(&mut self, secondary_color: u8) {
        unsafe { alt_IVehicle_SetSecondaryColor(self.0.load(Ordering::Relaxed), secondary_color) }
    }

    pub fn set_secondary_color_rgb(&mut self, secondary_color_rgb: Rgba) {
        unsafe {
            alt_IVehicle_SetSecondaryColorRGB(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(secondary_color_rgb.into())),
            )
        }
    }

    pub fn get_pearl_color(&self) -> u8 {
        unsafe { alt_IVehicle_GetPearlColor(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_pearl_color(&mut self, pearl_color: u8) {
        unsafe { alt_IVehicle_SetPearlColor(self.0.load(Ordering::Relaxed), pearl_color) }
    }

    pub fn get_wheel_color(&self) -> u8 {
        unsafe { alt_IVehicle_GetWheelColor(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_wheel_color(&mut self, wheel_color: u8) {
        unsafe { alt_IVehicle_SetWheelColor(self.0.load(Ordering::Relaxed), wheel_color) }
    }

    pub fn get_interior_color(&self) -> u8 {
        unsafe { alt_IVehicle_GetInteriorColor(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_interior_color(&mut self, interior_color: u8) {
        unsafe { alt_IVehicle_SetInteriorColor(self.0.load(Ordering::Relaxed), interior_color) }
    }

    pub fn get_dashboard_color(&self) -> u8 {
        unsafe { alt_IVehicle_GetDashboardColor(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_dashboard_color(&mut self, dashboard_color: u8) {
        unsafe { alt_IVehicle_SetDashboardColor(self.0.load(Ordering::Relaxed), dashboard_color) }
    }

    pub fn is_tire_smoke_color_custom(&self) -> bool {
        unsafe { alt_IVehicle_IsTireSmokeColorCustom(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_tire_smoke_color(&self) -> Rgba {
        unsafe {
            let c = alt_IVehicle_GetTireSmokeColor_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Rgba::from(*c)
        }
    }

    pub fn set_tire_smoke_color(&mut self, tire_smoke_color: Rgba) {
        unsafe {
            alt_IVehicle_SetTireSmokeColor(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(tire_smoke_color.into())),
            )
        }
    }

    pub fn get_wheel_type(&self) -> u8 {
        unsafe { alt_IVehicle_GetWheelType(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_wheel_variation(&self) -> u8 {
        unsafe { alt_IVehicle_GetWheelVariation(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_rear_wheel_variation(&self) -> u8 {
        unsafe { alt_IVehicle_GetRearWheelVariation(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_wheels(&mut self, t: u8, variation: u8) {
        unsafe { alt_IVehicle_SetWheels(self.0.load(Ordering::Relaxed), t, variation) }
    }

    pub fn set_rear_wheels(&mut self, variation: u8) {
        unsafe { alt_IVehicle_SetRearWheels(self.0.load(Ordering::Relaxed), variation) }
    }

    pub fn get_custom_tires(&self) -> bool {
        unsafe { alt_IVehicle_GetCustomTires(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_custom_tires(&mut self, state: bool) {
        unsafe { alt_IVehicle_SetCustomTires(self.0.load(Ordering::Relaxed), state) }
    }

    pub fn get_special_darkness(&self) -> u8 {
        unsafe { alt_IVehicle_GetSpecialDarkness(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_special_darkness(&mut self, special_darkness: u8) {
        unsafe { alt_IVehicle_SetSpecialDarkness(self.0.load(Ordering::Relaxed), special_darkness) }
    }

    pub fn get_license_plate_index(&self) -> u32 {
        unsafe { alt_IVehicle_GetNumberplateIndex(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_license_plate_index(&mut self, index: u32) {
        unsafe { alt_IVehicle_SetNumberplateIndex(self.0.load(Ordering::Relaxed), index) }
    }

    pub fn get_license_plate_text(&self) -> String {
        unsafe {
            let text = alt_IVehicle_GetNumberplateText_CAPI_Heap(self.0.load(Ordering::Relaxed));
            StringView::from(*text).get_data()
        }
    }

    pub fn set_license_plate_text(&mut self, text: &str) {
        unsafe {
            alt_IVehicle_SetNumberplateText(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(text).into())),
            )
        }
    }

    pub fn get_window_tint(&self) -> u8 {
        unsafe { alt_IVehicle_GetWindowTint(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_window_tint(&mut self, window_tint: u8) {
        unsafe { alt_IVehicle_SetWindowTint(self.0.load(Ordering::Relaxed), window_tint) }
    }

    pub fn get_dirt_level(&self) -> u8 {
        unsafe { alt_IVehicle_GetDirtLevel(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_dirt_level(&mut self, dirt_level: u8) {
        unsafe { alt_IVehicle_SetDirtLevel(self.0.load(Ordering::Relaxed), dirt_level) }
    }

    pub fn is_extra_on(&self, extra_id: u8) -> bool {
        unsafe { alt_IVehicle_IsExtraOn(self.0.load(Ordering::Relaxed), extra_id) }
    }

    pub fn set_extra_on(&mut self, extra_id: u8, state: bool) {
        unsafe { alt_IVehicle_ToggleExtra(self.0.load(Ordering::Relaxed), extra_id, state) }
    }

    pub fn is_neon_active(&self) -> bool {
        unsafe { alt_IVehicle_IsNeonActive(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_neon_active(&self) -> (bool, bool, bool, bool) {
        unsafe {
            let left = Box::into_raw(Box::new(false));
            let right = Box::into_raw(Box::new(false));
            let front = Box::into_raw(Box::new(false));
            let back = Box::into_raw(Box::new(false));
            alt_IVehicle_GetNeonActive(self.0.load(Ordering::Relaxed), left, right, front, back);
            (*left, *right, *front, *back)
        }
    }

    pub fn set_neon_active(&mut self, left: bool, right: bool, front: bool, back: bool) {
        unsafe {
            alt_IVehicle_SetNeonActive(self.0.load(Ordering::Relaxed), left, right, front, back)
        }
    }

    pub fn get_neon_color(&self) -> Rgba {
        unsafe {
            let c = alt_IVehicle_GetNeonColor_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Rgba::from(*c)
        }
    }

    pub fn set_neon_color(&mut self, neon_color: Rgba) {
        unsafe {
            alt_IVehicle_SetNeonColor(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(neon_color.into())),
            )
        }
    }

    pub fn get_livery(&self) -> u8 {
        unsafe { alt_IVehicle_GetLivery(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_livery(&mut self, livery: u8) {
        unsafe { alt_IVehicle_SetLivery(self.0.load(Ordering::Relaxed), livery) }
    }

    pub fn get_roof_livery(&self) -> u8 {
        unsafe { alt_IVehicle_GetRoofLivery(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_roof_livery(&mut self, roof_livery: u8) {
        unsafe { alt_IVehicle_SetRoofLivery(self.0.load(Ordering::Relaxed), roof_livery) }
    }

    pub fn get_appearance_data_base64(&self) -> String {
        unsafe {
            let b = alt_IVehicle_GetAppearanceDataBase64_CAPI_Heap(self.0.load(Ordering::Relaxed));
            crate::string::String::from(*b).get_data()
        }
    }

    pub fn load_appearance_data_from_base64(&mut self, base64: &str) {
        unsafe {
            alt_IVehicle_LoadAppearanceDataFromBase64(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(base64).into())),
            )
        }
    }

    pub fn is_engine_on(&self) -> bool {
        unsafe { alt_IVehicle_IsEngineOn(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_engine_on(&mut self, state: bool) {
        unsafe { alt_IVehicle_SetEngineOn(self.0.load(Ordering::Relaxed), state) }
    }

    pub fn is_handbrake_active(&self) -> bool {
        unsafe { alt_IVehicle_IsHandbrakeActive(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_headlight_color(&self) -> u8 {
        unsafe { alt_IVehicle_GetHeadlightColor(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_headlight_color(&mut self, headlight_color: u8) {
        unsafe { alt_IVehicle_SetHeadlightColor(self.0.load(Ordering::Relaxed), headlight_color) }
    }

    pub fn get_radio_station_index(&self) -> u32 {
        unsafe { alt_IVehicle_GetRadioStationIndex(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_radio_station_index(&mut self, index: u32) {
        unsafe { alt_IVehicle_SetRadioStationIndex(self.0.load(Ordering::Relaxed), index) }
    }

    pub fn is_siren_active(&self) -> bool {
        unsafe { alt_IVehicle_IsSirenActive(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_lock_state(&self) -> u8 {
        unsafe { alt_IVehicle_GetLockState(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_lock_state(&mut self, lock_state: u8) {
        unsafe { alt_IVehicle_SetLockState(self.0.load(Ordering::Relaxed), lock_state) }
    }

    pub fn is_window_opened(&self, window_id: u8) -> bool {
        unsafe { alt_IVehicle_IsWindowOpened(self.0.load(Ordering::Relaxed), window_id) }
    }

    pub fn set_window_opened(&mut self, window_id: u8, state: bool) {
        unsafe { alt_IVehicle_SetWindowOpened(self.0.load(Ordering::Relaxed), window_id, state) }
    }

    pub fn is_daylight_on(&self) -> bool {
        unsafe { alt_IVehicle_IsDaylightOn(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_nightlight_on(&self) -> bool {
        unsafe { alt_IVehicle_IsNightlightOn(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_roof_opened(&self) -> bool {
        unsafe { alt_IVehicle_IsRoofOpened(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_roof_opened(&mut self, state: bool) {
        unsafe { alt_IVehicle_SetRoofOpened(self.0.load(Ordering::Relaxed), state) }
    }

    pub fn is_flamethrower_active(&self) -> bool {
        unsafe { alt_IVehicle_IsFlamethrowerActive(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_lights_multiplier(&self) -> f32 {
        unsafe { alt_IVehicle_GetLightsMultiplier(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_lights_multiplier(&mut self, multiplier: f32) {
        unsafe { alt_IVehicle_SetLightsMultiplier(self.0.load(Ordering::Relaxed), multiplier) }
    }

    pub fn get_game_state_base64(&self) -> String {
        unsafe {
            let b = alt_IVehicle_GetScriptDataBase64_CAPI_Heap(self.0.load(Ordering::Relaxed));
            crate::string::String::from(*b).get_data()
        }
    }

    pub fn load_game_state_base64(&mut self, base64: &str) {
        unsafe {
            alt_IVehicle_LoadScriptDataFromBase64(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(base64).into())),
            )
        }
    }

    pub fn get_engine_health(&self) -> i32 {
        unsafe { alt_IVehicle_GetEngineHealth(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_engine_health(&mut self, health: i32) {
        unsafe { alt_IVehicle_SetEngineHealth(self.0.load(Ordering::Relaxed), health) }
    }

    pub fn get_fuel_tank_health(&self) -> i32 {
        unsafe { alt_IVehicle_GetPetrolTankHealth(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_fuel_tank_health(&mut self, health: i32) {
        unsafe { alt_IVehicle_SetPetrolTankHealth(self.0.load(Ordering::Relaxed), health) }
    }

    pub fn get_wheels_count(&self) -> u8 {
        unsafe { alt_IVehicle_GetWheelsCount(self.0.load(Ordering::Relaxed)) }
    }

    pub fn is_wheel_burst(&self, wheel_id: u8) -> bool {
        unsafe { alt_IVehicle_IsWheelBurst(self.0.load(Ordering::Relaxed), wheel_id) }
    }

    pub fn set_wheel_burst(&mut self, wheel_id: u8, state: bool) {
        unsafe { alt_IVehicle_SetWheelBurst(self.0.load(Ordering::Relaxed), wheel_id, state) }
    }

    pub fn does_wheel_have_tire(&self, wheel_id: u8) -> bool {
        unsafe { alt_IVehicle_DoesWheelHasTire(self.0.load(Ordering::Relaxed), wheel_id) }
    }

    pub fn set_wheel_has_tire(&mut self, wheel_id: u8, state: bool) {
        unsafe { alt_IVehicle_SetWheelHasTire(self.0.load(Ordering::Relaxed), wheel_id, state) }
    }

    pub fn is_wheel_detached(&self, wheel_id: u8) -> bool {
        unsafe { alt_IVehicle_IsWheelDetached(self.0.load(Ordering::Relaxed), wheel_id) }
    }

    pub fn set_wheel_detached(&mut self, wheel_id: u8, state: bool) {
        unsafe { alt_IVehicle_SetWheelDetached(self.0.load(Ordering::Relaxed), wheel_id, state) }
    }

    pub fn is_wheel_on_fire(&self, wheel_id: u8) -> bool {
        unsafe { alt_IVehicle_IsWheelOnFire(self.0.load(Ordering::Relaxed), wheel_id) }
    }

    pub fn set_wheel_on_fire(&mut self, wheel_id: u8, state: bool) {
        unsafe { alt_IVehicle_SetWheelOnFire(self.0.load(Ordering::Relaxed), wheel_id, state) }
    }

    pub fn get_wheel_health(&self, wheel_id: u8) -> f32 {
        unsafe { alt_IVehicle_GetWheelHealth(self.0.load(Ordering::Relaxed), wheel_id) }
    }

    pub fn set_wheel_health(&mut self, wheel_id: u8, health: f32) {
        unsafe { alt_IVehicle_SetWheelHealth(self.0.load(Ordering::Relaxed), wheel_id, health) }
    }

    pub fn get_repairs_count(&self) -> u8 {
        unsafe { alt_IVehicle_GetRepairsCount(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_body_health(&self) -> u32 {
        unsafe { alt_IVehicle_GetBodyHealth(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_body_health(&mut self, health: u32) {
        unsafe { alt_IVehicle_SetBodyHealth(self.0.load(Ordering::Relaxed), health) }
    }

    pub fn get_body_additional_health(&self) -> u32 {
        unsafe { alt_IVehicle_GetBodyAdditionalHealth(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set_body_additional_health(&mut self, health: u32) {
        unsafe { alt_IVehicle_SetBodyAdditionalHealth(self.0.load(Ordering::Relaxed), health) }
    }

    pub fn get_health_data_base64(&self) -> String {
        unsafe {
            let b = alt_IVehicle_GetHealthDataBase64_CAPI_Heap(self.0.load(Ordering::Relaxed));
            crate::string::String::from(*b).get_data()
        }
    }

    pub fn load_health_data_from_base64(&mut self, base64: &str) {
        unsafe {
            alt_IVehicle_LoadHealthDataFromBase64(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(base64).into())),
            )
        }
    }

    pub fn get_part_damage_level(&self, part_id: u8) -> u8 {
        unsafe { alt_IVehicle_GetPartDamageLevel(self.0.load(Ordering::Relaxed), part_id) }
    }

    pub fn set_part_damage_level(&mut self, part_id: u8, damage_level: u8) {
        unsafe {
            alt_IVehicle_SetPartDamageLevel(self.0.load(Ordering::Relaxed), part_id, damage_level)
        }
    }

    pub fn get_part_bullet_holes(&self, part_id: u8) -> u8 {
        unsafe { alt_IVehicle_GetPartBulletHoles(self.0.load(Ordering::Relaxed), part_id) }
    }

    pub fn set_part_bullet_holes(&mut self, part_id: u8, count: u8) {
        unsafe { alt_IVehicle_SetPartBulletHoles(self.0.load(Ordering::Relaxed), part_id, count) }
    }

    pub fn is_light_damaged(&self, light_id: u8) -> bool {
        unsafe { alt_IVehicle_IsLightDamaged(self.0.load(Ordering::Relaxed), light_id) }
    }

    pub fn set_light_damaged(&mut self, light_id: u8, state: bool) {
        unsafe { alt_IVehicle_SetLightDamaged(self.0.load(Ordering::Relaxed), light_id, state) }
    }

    pub fn is_window_damaged(&self, window_id: u8) -> bool {
        unsafe { alt_IVehicle_IsWindowDamaged(self.0.load(Ordering::Relaxed), window_id) }
    }

    pub fn set_window_damaged(&mut self, window_id: u8, state: bool) {
        unsafe { alt_IVehicle_SetWindowDamaged(self.0.load(Ordering::Relaxed), window_id, state) }
    }

    pub fn is_special_light_damaged(&self, special_light_id: u8) -> bool {
        unsafe {
            alt_IVehicle_IsSpecialLightDamaged(self.0.load(Ordering::Relaxed), special_light_id)
        }
    }

    pub fn set_special_light_damaged(&mut self, special_light_id: u8, state: bool) {
        unsafe {
            alt_IVehicle_SetSpecialLightDamaged(
                self.0.load(Ordering::Relaxed),
                special_light_id,
                state,
            )
        }
    }

    pub fn has_armored_windows(&self) -> bool {
        unsafe { alt_IVehicle_HasArmoredWindows(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_armored_window_health(&self, window_id: u8) -> f32 {
        unsafe { alt_IVehicle_GetArmoredWindowHealth(self.0.load(Ordering::Relaxed), window_id) }
    }

    pub fn set_armored_window_health(&self, window_id: u8, health: f32) {
        unsafe {
            alt_IVehicle_SetArmoredWindowHealth(self.0.load(Ordering::Relaxed), window_id, health)
        }
    }

    pub fn get_armored_window_shoot_count(&self, window_id: u8) -> u8 {
        unsafe {
            alt_IVehicle_GetArmoredWindowShootCount(self.0.load(Ordering::Relaxed), window_id)
        }
    }

    pub fn set_armored_window_shoot_count(&mut self, window_id: u8, shoot_count: u8) {
        unsafe {
            alt_IVehicle_SetArmoredWindowShootCount(
                self.0.load(Ordering::Relaxed),
                window_id,
                shoot_count,
            )
        }
    }

    pub fn get_bumper_damage_level(&self, bumper_id: u8) -> u8 {
        unsafe { alt_IVehicle_GetBumperDamageLevel(self.0.load(Ordering::Relaxed), bumper_id) }
    }

    pub fn set_bumper_damage_level(&mut self, bumper_id: u8, damage_level: u8) {
        unsafe {
            alt_IVehicle_SetBumperDamageLevel(
                self.0.load(Ordering::Relaxed),
                bumper_id,
                damage_level,
            )
        }
    }

    pub fn get_damage_data_base64(&self) -> String {
        unsafe {
            let b = alt_IVehicle_GetDamageDataBase64_CAPI_Heap(self.0.load(Ordering::Relaxed));
            crate::string::String::from(*b).get_data()
        }
    }

    pub fn load_damage_data_from_base64(&mut self, base64: &str) {
        unsafe {
            alt_IVehicle_LoadDamageDataFromBase64(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(base64).into())),
            )
        }
    }

    pub fn set_manual_engine_control(&mut self, state: bool) {
        unsafe { alt_IVehicle_SetManualEngineControl(self.0.load(Ordering::Relaxed), state) }
    }

    pub fn is_manual_engine_control(&self) -> bool {
        unsafe { alt_IVehicle_IsManualEngineControl(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_script_data_base64(&self) -> String {
        unsafe {
            let b = alt_IVehicle_GetScriptDataBase64_CAPI_Heap(self.0.load(Ordering::Relaxed));
            crate::string::String::from(*b).get_data()
        }
    }

    pub fn load_script_data_from_base64(&mut self, base64: &str) {
        unsafe {
            alt_IVehicle_LoadScriptDataFromBase64(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(StringView::new(base64).into())),
            )
        }
    }

    pub fn is_destroyed(&self) -> bool {
        unsafe { alt_IVehicle_IsDestroyed(self.0.load(Ordering::Relaxed)) }
    }
}

impl Component for CVehicle {
    type Storage = VecStorage<Self>;
}

pub struct CCollisionShape(pub AtomicPtr<alt_IColShape>);

impl CCollisionShape {
    pub fn get_type(&self) -> u8 {
        unsafe { alt_IColShape_GetColshapeType(self.0.load(Ordering::Relaxed)) as u8 }
    }

    pub fn is_entity_in(&self, entity: &CEntity) -> bool {
        unsafe {
            alt_IColShape_IsEntityIn(
                self.0.load(Ordering::Relaxed),
                alt_RefBase_RefStore_IEntity_Create_4_CAPI_Heap(entity.0.load(Ordering::Relaxed)),
            )
        }
    }

    pub fn is_point_in(&self, position: Vector3) -> bool {
        unsafe {
            alt_IColShape_IsPointIn(
                self.0.load(Ordering::Relaxed),
                Box::into_raw(Box::new(position.into())),
            )
        }
    }
}

impl Component for CCollisionShape {
    type Storage = VecStorage<Self>;
}

pub struct CCheckpoint(pub AtomicPtr<alt_ICheckpoint>);

impl CCheckpoint {
    pub fn get_type(&self) -> u8 {
        unsafe { alt_ICheckpoint_GetCheckpointType(self.0.load(Ordering::Relaxed)) as u8 }
    }

    pub fn get_height(&self) -> f32 {
        unsafe { alt_ICheckpoint_GetHeight(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_radius(&self) -> f32 {
        unsafe { alt_ICheckpoint_GetRadius(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_color(&self) -> Rgba {
        unsafe {
            let c = alt_ICheckpoint_GetColor_CAPI_Heap(self.0.load(Ordering::Relaxed));
            Rgba::from(*c)
        }
    }
}

impl Component for CCheckpoint {
    type Storage = VecStorage<Self>;
}

pub struct CBlip(pub AtomicPtr<alt_IBlip>);

impl CBlip {
    pub fn is_global(&self) -> bool {
        unsafe { alt_IBlip_IsGlobal(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_target(&self, alt: &AltResource) -> Option<Entity> {
        unsafe {
            let player = alt_IBlip_GetTarget_CAPI_Heap(self.0.load(Ordering::Relaxed));
            let player = (*player).ptr;

            if player.is_null() {
                return None;
            }

            Some(*alt.players.get(&(player as usize)).unwrap())
        }
    }

    pub fn attached_to(&self, alt: &AltResource) -> Option<Entity> {
        unsafe {
            let entity = alt_IBlip_AttachedTo_CAPI_Heap(self.0.load(Ordering::Relaxed));
            let entity = (*entity).ptr;

            if entity.is_null() {
                return None;
            }

            match alt_IEntity_GetType(entity) {
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_PLAYER => {
                    let player = alt_IEntity_to_alt_IPlayer(entity);
                    Some(*alt.players.get(&(player as usize)).unwrap())
                }
                alt_IBaseObject_Type::ALT_IBASEOBJECT_TYPE_VEHICLE => {
                    let vehicle = alt_IEntity_to_alt_IVehicle(entity);
                    Some(*alt.vehicles.get(&(vehicle as usize)).unwrap())
                }
                _ => panic!(),
            }
        }
    }

    pub fn get_blip_type(&self) -> u8 {
        unsafe { alt_IBlip_GetBlipType(self.0.load(Ordering::Relaxed)) as u8 }
    }

    pub fn set_sprite(&mut self, sprite: u16) {
        unsafe { alt_IBlip_SetSprite(self.0.load(Ordering::Relaxed), sprite) }
    }

    pub fn set_color(&mut self, color: u8) {
        unsafe { alt_IBlip_SetColor(self.0.load(Ordering::Relaxed), color) }
    }

    pub fn set_route(&mut self, state: bool) {
        unsafe { alt_IBlip_SetRoute(self.0.load(Ordering::Relaxed), state) }
    }

    pub fn set_route_color(&mut self, color: u8) {
        unsafe { alt_IBlip_SetRouteColor(self.0.load(Ordering::Relaxed), color) }
    }
}

impl Component for CBlip {
    type Storage = VecStorage<Self>;
}

pub struct CVoiceChannel(pub AtomicPtr<alt_IVoiceChannel>);

impl CVoiceChannel {
    pub fn is_spatial(&self) -> bool {
        unsafe { alt_IVoiceChannel_IsSpatial(self.0.load(Ordering::Relaxed)) }
    }

    pub fn get_max_distance(&self) -> f32 {
        unsafe { alt_IVoiceChannel_GetMaxDistance(self.0.load(Ordering::Relaxed)) }
    }

    pub fn has_player(&self, player: &CPlayer) -> bool {
        unsafe {
            alt_IVoiceChannel_HasPlayer(
                self.0.load(Ordering::Relaxed),
                alt_RefBase_RefStore_IPlayer_Create_4_CAPI_Heap(player.0.load(Ordering::Relaxed)),
            )
        }
    }

    pub fn add_player(&mut self, player: &CPlayer) {
        unsafe {
            alt_IVoiceChannel_AddPlayer(
                self.0.load(Ordering::Relaxed),
                alt_RefBase_RefStore_IPlayer_Create_4_CAPI_Heap(player.0.load(Ordering::Relaxed)),
            )
        }
    }

    pub fn remove_player(&mut self, player: &CPlayer) {
        unsafe {
            alt_IVoiceChannel_RemovePlayer(
                self.0.load(Ordering::Relaxed),
                alt_RefBase_RefStore_IPlayer_Create_4_CAPI_Heap(player.0.load(Ordering::Relaxed)),
            )
        }
    }

    pub fn is_player_muted(&self, player: &CPlayer) -> bool {
        unsafe {
            alt_IVoiceChannel_IsPlayerMuted(
                self.0.load(Ordering::Relaxed),
                alt_RefBase_RefStore_IPlayer_Create_4_CAPI_Heap(player.0.load(Ordering::Relaxed)),
            )
        }
    }

    pub fn mute_player(&mut self, player: &CPlayer) {
        unsafe {
            alt_IVoiceChannel_MutePlayer(
                self.0.load(Ordering::Relaxed),
                alt_RefBase_RefStore_IPlayer_Create_4_CAPI_Heap(player.0.load(Ordering::Relaxed)),
            )
        }
    }

    pub fn unmute_player(&mut self, player: &CPlayer) {
        unsafe {
            alt_IVoiceChannel_UnmutePlayer(
                self.0.load(Ordering::Relaxed),
                alt_RefBase_RefStore_IPlayer_Create_4_CAPI_Heap(player.0.load(Ordering::Relaxed)),
            )
        }
    }
}

impl Component for CVoiceChannel {
    type Storage = VecStorage<Self>;
}

pub enum CollisionShapeType {
    Sphere = 0,
    Cylinder = 1,
    Circle = 2,
    Cuboid = 3,
    Rectangle = 4,
    CheckpointCylinder = 5,
}
