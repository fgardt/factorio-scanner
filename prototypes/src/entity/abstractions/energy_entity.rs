use serde::{Deserialize, Serialize};
use types::{AnyEnergySource, Vector};

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
        options: &crate::entity::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut types::ImageCache,
    ) -> crate::entity::RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(
        &self,
        options: &crate::entity::RenderOpts,
    ) -> Vec<types::MapPosition> {
        let mut child = self.child.fluid_box_connections(options);

        if let AnyEnergySource::Fluid { data } = &self.energy_source {
            child.append(
                &mut data
                    .fluid_box
                    .connection_points(options.direction, options.mirrored),
            );
        };

        child
    }

    fn heat_buffer_connections(
        &self,
        options: &crate::entity::RenderOpts,
    ) -> Vec<types::MapPosition> {
        let mut child = self.child.heat_buffer_connections(options);

        if let AnyEnergySource::Heat { data } = &self.energy_source {
            // identical code as the HeatBuffer::connection_points method
            // maybe this can be deduplicated somehow?
            child.append(
                &mut data
                    .connections
                    .iter()
                    .map(|c| {
                        let offset = c.direction.get_offset();
                        let pos: Vector = c.position.into();

                        options.direction.rotate_vector(pos + offset).into()
                    })
                    .collect(),
            );
        };

        child
    }

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }
}
