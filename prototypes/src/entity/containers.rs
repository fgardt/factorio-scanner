use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/ContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/ContainerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerPrototype(EntityWithOwnerPrototype<ContainerData>);

impl super::Renderable for ContainerPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, image_cache)
    }
}

/// [`Prototypes/ContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/ContainerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerData {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub inventory_size: Option<ItemStackIndex>, // overridden in `InfinityContainerPrototype`

    pub picture: Option<Sprite>, // overridden in `LogisticContainerPrototype`

    #[serde(default)]
    pub inventory_type: InventoryType,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub enable_inventory_bar: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub scale_info_icons: bool,

    pub circuit_wire_connection_point: Option<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
}

impl super::Renderable for ContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.picture.as_ref().and_then(|picture| {
            picture.render(
                options.factorio_dir,
                &options.used_mods,
                image_cache,
                &options.into(),
            )
        })
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InventoryType {
    #[default]
    WithBar,
    WithFiltersAndBar,
}

/// [`Prototypes/LogisticContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/LogisticContainerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct LogisticContainerPrototype(EntityWithOwnerPrototype<LogisticContainerData>);

impl super::Renderable for LogisticContainerPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, image_cache)
    }
}

/// [`Prototypes/LogisticContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/LogisticContainerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LogisticContainerData {
    pub logistic_mode: Option<LogisticMode>, // overridden in `InfinityContainerPrototype`

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub max_logistic_slots: Option<u16>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub render_not_in_network_icon: bool, // overridden in `InfinityContainerPrototype`

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub opened_duration: u8,

    pub animation: Option<Animation>,
    pub landing_location_offset: Option<Vector>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub use_exact_mode: bool,

    #[serde(flatten)]
    pub parent: ContainerData,
    // not implemented
    // pub animation_sound: Option<Sound>,
}

impl super::Renderable for LogisticContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.animation.as_ref()?.render(
            options.factorio_dir,
            &options.used_mods,
            image_cache,
            &options.into(),
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogisticMode {
    PassiveProvider,
    ActiveProvider,
    Requester,
    Storage,
    Buffer,
}

/// [`Prototypes/InfinityContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityContainerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct InfinityContainerPrototype(EntityWithOwnerPrototype<InfinityContainerData>);

impl super::Renderable for InfinityContainerPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, image_cache)
    }
}

/// [`Prototypes/InfinityContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityContainerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct InfinityContainerData {
    pub erase_contents_when_mined: bool,

    // TODO: skip serializing if default
    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,

    #[serde(flatten)]
    pub parent: LogisticContainerData,
}

impl super::Renderable for InfinityContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.parent.parent.render(options, image_cache)
    }
}

/// [`Prototypes/LinkedContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedContainerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct LinkedContainerPrototype(EntityWithOwnerPrototype<LinkedContainerData>);

impl super::Renderable for LinkedContainerPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, image_cache)
    }
}

/// [`Prototypes/LinkedContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedContainerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LinkedContainerData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    pub picture: Option<Sprite>,

    // TODO: skip serializing if default
    #[serde(default)]
    pub inventory_type: InventoryType,

    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub scale_info_icons: bool,

    pub circuit_wire_connection_point: Option<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
}

impl super::Renderable for LinkedContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.picture.as_ref().and_then(|picture| {
            picture.render(
                options.factorio_dir,
                &options.used_mods,
                image_cache,
                &options.into(),
            )
        })
    }
}
