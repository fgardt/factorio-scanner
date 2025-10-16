use serde::{Deserialize, Serialize};
use types::{AnyEnergySource, Direction, MapPosition};

use super::Renderable;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyEntityData<T: Renderable> {
    pub energy_source: AnyEnergySource,

    #[serde(flatten)]
    child: T,
}

impl<T: Renderable> std::ops::Deref for EnergyEntityData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: Renderable> Renderable for EnergyEntityData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut types::ImageCache,
    ) -> crate::entity::RenderOutput {
        // TODO: render pipe covers on fluid energy source / heat pipe connections on heat energy source

        match &self.energy_source {
            AnyEnergySource::Fluid { data } => {
                data.fluid_box
                    .render(options, used_mods, render_layers, image_cache);
            }
            AnyEnergySource::Heat { data } => {
                data.render_debug(options, used_mods, render_layers);
            }
            _ => {}
        }

        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<(MapPosition, Direction)> {
        let mut res = self.child.fluid_box_connections(options);

        if let AnyEnergySource::Fluid { data } = &self.energy_source {
            res.append(&mut data.fluid_box.fluid_box_connections(options));
        }

        res
    }

    fn heat_buffer_connections(
        &self,
        options: &super::RenderOpts,
    ) -> Vec<(MapPosition, Direction)> {
        let mut res = self.child.heat_buffer_connections(options);

        if let AnyEnergySource::Heat { data } = &self.energy_source {
            res.append(&mut data.heat_buffer_connections(options));
        }

        res
    }

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }

    fn render_debug(
        &self,
        options: &super::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        match &self.energy_source {
            AnyEnergySource::Fluid { data } => {
                data.fluid_box
                    .render_debug(options, used_mods, render_layers);
            }
            AnyEnergySource::Heat { data } => {
                data.render_debug(options, used_mods, render_layers);
            }
            _ => {}
        }

        self.child.render_debug(options, used_mods, render_layers);
    }
}
