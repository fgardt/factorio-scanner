use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/LabPrototype`](https://lua-api.factorio.com/latest/prototypes/LabPrototype.html)
pub type LabPrototype = EntityWithOwnerPrototype<EnergyEntityData<LabData>>;

/// [`Prototypes/LabPrototype`](https://lua-api.factorio.com/latest/prototypes/LabPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LabData {
    pub energy_usage: Energy,
    pub on_animation: Option<Animation>,
    pub off_animation: Option<Animation>,
    pub frozen_patch: Option<Sprite>,
    pub inputs: FactorioArray<ItemID>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub researching_speed: f64,

    pub effect_receiver: Option<EffectReceiver>,
    pub module_slots: Option<ItemStackIndex>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub uses_quality_drain_modifier: bool,

    #[serde(default = "helper::u8_100", skip_serializing_if = "helper::is_100_u8")]
    pub science_pack_drain_rate_percent: u8,

    pub allowed_effects: Option<EffectTypeLimitation>,
    pub allowed_module_categories: Option<FactorioArray<ModuleCategoryID>>,

    pub light: Option<LightDefinition>,
    pub trash_inventory_size: Option<ItemStackIndex>,
}

impl super::Renderable for LabData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.off_animation.as_ref().and_then(|oa| {
            oa.render(
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
