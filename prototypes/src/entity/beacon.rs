use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

use serde_helper as helper;

/// [`Prototypes/BeaconPrototype`](https://lua-api.factorio.com/latest/prototypes/BeaconPrototype.html)
pub type BeaconPrototype = EntityWithOwnerPrototype<BeaconData>;

/// [`Prototypes/BeaconPrototype`](https://lua-api.factorio.com/latest/prototypes/BeaconPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BeaconData {
    pub energy_usage: Energy,
    pub energy_source: AnyEnergySource, // must be electric or void

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub supply_area_distance: u32,
    pub distribution_effectivity: f64,
    pub distribution_effectivity_bonus_per_quality_level: Option<f64>,
    pub module_slots: ItemStackIndex,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub quality_affects_module_slots: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub quality_affects_supply_area_distance: bool,

    pub graphics_set: Option<BeaconGraphicsSet>,
    pub animation: Option<Animation>,
    pub base_picture: Option<Animation>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub perceived_performance: PerceivedPerformance,

    pub radius_visualisation_picture: Option<Sprite>,

    pub allowed_effects: Option<EffectTypeLimitation>,
    pub allowed_module_categories: Option<FactorioArray<ModuleCategoryID>>,
    pub profile: Option<FactorioArray<f64>>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub beacon_counter: BeaconCounter,
}

impl super::Renderable for BeaconData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = if let Some(set) = self.graphics_set.as_ref() {
            set.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        } else {
            merge_renders(
                &[
                    self.base_picture.as_ref().and_then(|g| {
                        g.render(
                            render_layers.scale(),
                            used_mods,
                            image_cache,
                            &options.into(),
                        )
                    }),
                    self.animation.as_ref().and_then(|a| {
                        a.render(
                            render_layers.scale(),
                            used_mods,
                            image_cache,
                            &options.into(),
                        )
                    }),
                ],
                render_layers.scale(),
            )
        }?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BeaconCounter {
    #[default]
    Total,
    SameType,
}
