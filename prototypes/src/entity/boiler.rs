use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, FluidBoxEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/BoilerPrototype`](https://lua-api.factorio.com/latest/prototypes/BoilerPrototype.html)
pub type BoilerPrototype =
    EntityWithOwnerPrototype<FluidBoxEntityData<EnergyEntityData<BoilerData>>>;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BoilerData {
    pub pictures: Option<BoilerPictureSet>,

    pub output_fluid_box: FluidBox,
    pub energy_consumption: Energy,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub burning_cooldown: u16,

    pub target_temperature: Option<f32>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fire_glow_flicker_enabled: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fire_flicker_enabled: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub mode: BoilerMode,
}

impl super::Renderable for BoilerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let Some(pictures) = self.pictures.as_ref() else {
            return Some(());
        };

        let res = pictures.structure().render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        self.output_fluid_box
            .connection_points(options.direction, options.mirrored)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BoilerMode {
    #[default]
    HeatFluidInside,
    OutputToSeparatePipe,
}

/// [`Prototypes/BoilerPrototype/BoilerPictureSet`](https://lua-api.factorio.com/latest/prototypes/BoilerPrototype.html#pictures)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoilerPictureSet {
    pub north: BoilerPictures,
    pub east: BoilerPictures,
    pub south: BoilerPictures,
    pub west: BoilerPictures,
}

impl BoilerPictureSet {
    #[must_use]
    pub fn structure(&self) -> Animation4Way {
        Animation4Way::Struct {
            north: Box::new(self.north.structure.clone()),
            east: Some(Box::new(self.east.structure.clone())),
            south: Some(Box::new(self.south.structure.clone())),
            west: Some(Box::new(self.west.structure.clone())),
            north_east: None,
            south_east: None,
            south_west: None,
            north_west: None,
        }
    }
}

/// [`Types/BoilerPictures`](https://lua-api.factorio.com/latest/types/BoilerPictures.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoilerPictures {
    pub structure: Animation,
    pub patch: Option<Sprite>,
    pub fire: Option<Animation>,
    pub fire_glow: Option<Animation>,
}
