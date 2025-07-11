use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/AgriculturalTowerPrototype`](https://lua-api.factorio.com/latest/prototypes/AgriculturalTowerPrototype.html)
pub type AgriculturalTowerPrototype =
    EntityWithOwnerPrototype<EnergyEntityData<AgriculturalTowerData>>;

/// [`Prototypes/AgriculturalTowerPrototype`](https://lua-api.factorio.com/latest/prototypes/AgriculturalTowerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AgriculturalTowerData {
    pub graphics_set: Option<CraftingMachineGraphicsSet>,
    pub crane: AgriculturalCraneProperties,
    pub input_inventory_size: ItemStackIndex,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub output_inventory_size: ItemStackIndex,
    pub energy_usage: Energy,
    pub crane_energy_usage: Energy,
    pub radius: f64,

    #[serde(default = "helper::u32_3", skip_serializing_if = "helper::is_3_u32")]
    pub growth_grid_tile_size: u32,
    #[serde(
        default = "helper::f64_095",
        skip_serializing_if = "helper::is_095_f64"
    )]
    pub growth_area_radius: f64,
    #[serde(
        default = "helper::f64_025",
        skip_serializing_if = "helper::is_025_f64"
    )]
    pub random_growth_offset: f64,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub randomize_planting_tile: bool,
    pub radius_visualisation_picture: Option<Sprite>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub planting_procedure_points: FactorioArray<Vector3D>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub harvesting_procedure_points: FactorioArray<Vector3D>,

    pub accepted_seeds: Option<FactorioArray<ItemID>>,
    // not implemented
    // pub central_orienting_sound: Option<InterruptibleSound>,
    // pub arm_extending_sound: Option<InterruptibleSound>,
    // pub grappler_orienting_sound: Option<InterruptibleSound>,
    // pub grappler_extending_sound: Option<InterruptibleSound>,
    // pub planting_sound: Option<Sound>,
    // pub harvesting_sound: Option<Sound>,
    // pub central_orienting_sound_source: Option<String>,
    // pub arm_extending_sound_source: Option<String>,
    // pub grappler_orienting_sound_source: Option<String>,
    // pub grappler_extending_sound_source: Option<String>,
}

impl super::Renderable for AgriculturalTowerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.graphics_set.as_ref().and_then(|gs| {
            gs.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

/// [`Types/AgriculturalCraneProperties`](https://lua-api.factorio.com/latest/types/AgriculturalCraneProperties.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct AgriculturalCraneProperties {
    pub speed: AgriculturalCraneSpeed,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub min_arm_extent: f64,
    #[serde(default = "helper::f64_02", skip_serializing_if = "helper::is_02_f64")]
    pub min_grappler_extent: f64,
    #[serde(default = "helper::f32_45", skip_serializing_if = "helper::is_45_f32")]
    pub operation_angle: f32,
    #[serde(default = "helper::f64_05", skip_serializing_if = "helper::is_05_f64")]
    pub telescope_default_extension: f64,

    pub origin: Vector3D,
    pub shadow_direction: Vector3D,
    pub parts: FactorioArray<CranePart>,
}

/// [`Types/AgriculturalCraneSpeed`](https://lua-api.factorio.com/latest/types/AgriculturalCraneSpeed.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct AgriculturalCraneSpeed {
    pub arm: AgriculturalCraneSpeedArm,
    pub grappler: AgriculturalCraneSpeedGrappler,
}

/// [`Types/AgriculturalCraneSpeedArm`](https://lua-api.factorio.com/latest/types/AgriculturalCraneSpeedArm.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct AgriculturalCraneSpeedArm {
    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub turn_rate: f64,
    #[serde(
        default = "helper::f64_005",
        skip_serializing_if = "helper::is_005_f64"
    )]
    pub extension_speed: f64,
}

/// [`Types/AgriculturalCraneSpeedGrappler`](https://lua-api.factorio.com/latest/types/AgriculturalCraneSpeedGrappler.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct AgriculturalCraneSpeedGrappler {
    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub vertical_turn_rate: f64,
    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub horizontal_turn_rate: f64,
    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub extension_speed: f64,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_transpolar_movement: bool,
}

/// [`Types/CranePart`](https://lua-api.factorio.com/latest/types/CranePart.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct CranePart {
    // TODO: implement crane parts & rendering
}
