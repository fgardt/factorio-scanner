use serde::{Deserialize, Serialize};

use serde_helper as helper;
use types::{Color, Icon, QualityID, RenderableGraphics};

use crate::helper_macro::namespace_struct;

/// [`Prototypes/QualityPrototype`](https://lua-api.factorio.com/latest/prototypes/QualityPrototype.html)
pub type QualityPrototype = crate::BasePrototype<QualityPrototypeData>;

/// [`Prototypes/QualityPrototype`](https://lua-api.factorio.com/latest/prototypes/QualityPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct QualityPrototypeData {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_sprite_by_default: bool,

    pub color: Color,
    pub level: u32,
    pub next: Option<QualityID>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub next_probability: f64,

    #[serde(flatten)]
    pub icon: Icon,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub beacon_power_usage_multiplier: f32,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub mining_drill_resource_drain_multiplier: f32,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub science_pack_drain_multiplier: f32,
}

namespace_struct! {
    AllTypes,
    QualityID,
    "quality"
}

impl AllTypes {
    pub fn get_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.quality
            .get(&QualityID::new(name))
            .and_then(|proto| proto.icon.render(scale, used_mods, image_cache, &()))
    }
}
