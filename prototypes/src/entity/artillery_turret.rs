use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ArtilleryTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ArtilleryTurretPrototype.html)
pub type ArtilleryTurretPrototype = EntityWithOwnerPrototype<WireEntityData<ArtilleryTurretData>>;

/// [`Prototypes/ArtilleryTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ArtilleryTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ArtilleryTurretData {
    pub gun: ItemID,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub ammo_stack_limit: ItemCountType,

    #[serde(deserialize_with = "helper::truncating_opt_deserializer")]
    pub automated_ammo_count: Option<ItemCountType>,
    pub turret_rotation_speed: f64,
    pub manual_range_modifier: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub alert_when_attacking: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub disable_automatic_firing: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub base_picture_secondary_draw_order: u8,

    pub base_picture_render_layer: Option<RenderLayer>,
    pub base_picture: Option<Animation4Way>,
    pub cannon_base_shift: Vector3D,
    pub cannon_base_pictures: Option<RotatedSprite>,
    pub cannon_barrel_pictures: Option<RotatedSprite>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub turn_after_shooting_cooldown: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub cannon_parking_frame_count: u16,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub cannon_parking_speed: f64,

    #[serde(flatten)]
    pub cannon_barrel_recoil_shiftings: Option<ArtilleryTurretCannonBarrelShiftings>,
    // not implemented
    // pub rotating_sound: Option<InterruptibleSound>,
    // pub rotating_stopped_sound: Option<Sound>,
}

impl super::Renderable for ArtilleryTurretData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let cannon_opts = &super::RenderOpts {
            orientation: Some(
                options
                    .orientation
                    .map_or(options.direction.to_orientation(), |o| o),
            ),
            ..options.clone()
        };

        let res = merge_renders(
            &[
                self.base_picture.as_ref().and_then(|a| {
                    a.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &cannon_opts.into(),
                    )
                }),
                self.cannon_barrel_pictures.as_ref().and_then(|s| {
                    s.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &cannon_opts.into(),
                    )
                }),
                self.cannon_base_pictures.as_ref().and_then(|s| {
                    s.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &cannon_opts.into(),
                    )
                }),
            ],
            render_layers.scale(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ArtilleryTurretCannonBarrelShiftings {
    #[serde(
        default,
        rename = "cannon_barrel_recoil_shiftings",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shiftings: FactorioArray<Vector3D>,

    #[serde(
        default,
        rename = "cannon_barrel_recoil_shiftings_load_correction_matrix",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_correction_matrix: FactorioArray<Vector3D>,

    #[serde(rename = "cannon_barrel_light_direction")]
    pub light_direction: Option<Vector3D>,
}
