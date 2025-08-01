use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/ContainerPrototype.html)
pub type ContainerPrototype = EntityWithOwnerPrototype<WireEntityData<ContainerData>>;

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

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub quality_affects_inventory_size: bool,

    pub picture: Option<Sprite>, // overridden in `LogisticContainerPrototype`

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub inventory_type: InventoryType,

    pub inventory_properties: Option<InventoryWithCustomStackSizeSpecification>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub inventory_weight_limit: Weight,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub default_status: EntityStatus,
}

impl super::Renderable for ContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture.as_ref().and_then(|picture| {
            picture.render(
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InventoryType {
    Normal,
    #[default]
    WithBar,
    WithFiltersAndBar,
    WithCustomStackSize,
    WithWeightLimit,
}

/// [`Types/InventoryWithCustomStackSizeSpecification`](https://lua-api.factorio.com/latest/types/InventoryWithCustomStackSizeSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct InventoryWithCustomStackSizeSpecification {
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub stack_size_multiplier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub stack_size_min: f64,

    pub stack_size_max: Option<ItemCountType>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub stack_size_override: HashMap<ItemID, ItemCountType>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub with_bar: bool,
}

/// [`Prototypes/LogisticContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/LogisticContainerPrototype.html)
pub type LogisticContainerPrototype =
    EntityWithOwnerPrototype<WireEntityData<LogisticContainerData>>;

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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub trash_inventory_size: ItemStackIndex,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub render_not_in_network_icon: bool, // overridden in `InfinityContainerPrototype`

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub opened_duration: u8,

    pub animation: Option<Animation>,
    pub landing_location_offset: Option<Vector>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub use_exact_mode: bool,

    #[serde(flatten)]
    parent: ContainerData,
    // not implemented
    // pub animation_sound: Option<Sound>,
}

impl Deref for LogisticContainerData {
    type Target = ContainerData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for LogisticContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let Some(res) = self.animation.as_ref().and_then(|a| {
            a.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) else {
            return self
                .parent
                .render(options, used_mods, render_layers, image_cache);
        };

        render_layers.add_entity(res, &options.position);

        Some(())
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
pub type InfinityContainerPrototype =
    EntityWithOwnerPrototype<WireEntityData<InfinityContainerData>>;

/// [`Prototypes/InfinityContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityContainerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct InfinityContainerData {
    pub erase_contents_when_mined: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub preserve_contents_when_created: bool,

    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,

    #[serde(flatten)]
    parent: LogisticContainerData,
}

impl Deref for InfinityContainerData {
    type Target = LogisticContainerData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for InfinityContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }
}

/// [`Prototypes/LinkedContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedContainerPrototype.html)
pub type LinkedContainerPrototype = EntityWithOwnerPrototype<WireEntityData<LinkedContainerData>>;

/// [`Prototypes/LinkedContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedContainerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LinkedContainerData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    pub picture: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub inventory_type: InventoryType,

    pub inventory_properties: Option<InventoryWithCustomStackSizeSpecification>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub inventory_weight_limit: Weight,

    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,
}

impl super::Renderable for LinkedContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture.as_ref().and_then(|picture| {
            picture.render(
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
