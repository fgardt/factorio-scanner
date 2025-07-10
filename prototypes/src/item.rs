use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{
    CollisionMaskConnector, Color, Energy, EntityID, EquipmentID, FactorioArray, FileName,
    FuelCategoryID, Icon, IconData, ItemCountType, ItemID, ItemProductPrototype,
    ItemPrototypeFlags, RenderableGraphics, SpaceLocationID, SpriteVariations, TileID, Weight,
};

mod ammo;
mod capsule;
mod gun;
mod item_with_entity_data;
mod item_with_label;
mod module;
mod rail_planner;
mod space_platform_starter_pack;
mod tool;

pub use ammo::*;
pub use capsule::*;
pub use gun::*;
pub use item_with_entity_data::*;
pub use item_with_label::*;
pub use module::*;
pub use rail_planner::*;
pub use space_platform_starter_pack::*;
pub use tool::*;

use crate::helper_macro::namespace_struct;

/// [`Prototypes/ItemPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPrototype(super::BasePrototype<ItemPrototypeData>);

impl std::ops::Deref for ItemPrototype {
    type Target = super::BasePrototype<ItemPrototypeData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`Prototypes/ItemPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPrototypeData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub stack_size: ItemCountType,

    #[serde(flatten)]
    pub icon: Icon,

    #[serde(flatten)]
    pub dark_background_icon: Option<DarkBackgroundIcon>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub place_result: EntityID,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub place_as_equipment_result: EquipmentID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fuel_category: FuelCategoryID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub burnt_result: ItemID,
    pub spoil_result: Option<ItemID>,
    pub plant_result: Option<EntityID>,
    pub place_as_tile: Option<PlaceAsTile>,

    pub pictures: Option<SpriteVariations>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: ItemPrototypeFlags,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub spoil_ticks: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fuel_value: Energy,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_acceleration_multiplier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_top_speed_multiplier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_emissions_multiplier: f64,

    pub fuel_acceleration_multiplier_quality_bonus: Option<f64>,
    pub fuel_top_speed_multiplier_quality_bonus: Option<f64>,

    pub weight: Option<Weight>,

    #[serde(default = "helper::f64_05", skip_serializing_if = "helper::is_05_f64")]
    pub ingredient_to_weight_coefficient: f64,

    pub fuel_glow_color: Option<Color>,

    // pub open_sound: Option<Sound>,
    // pub close_sound: Option<Sound>,
    // pub pick_sound: Option<Sound>,
    // pub drop_sound: Option<Sound>,
    // pub inventory_move_sound: Option<Sound>,
    pub default_import_location: Option<SpaceLocationID>,

    // pub color_hint: Option<ColorHintSpecification>,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub has_random_tint: bool,

    // pub spoil_to_trigger_result: Option<SpoilToTriggerResult>,
    // pub destroyed_by_dropping_trigger: Option<Trigger>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rocket_launch_products: FactorioArray<ItemProductPrototype>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub send_to_orbit_mode: SendToOrbitMode,

    pub moved_to_hub_when_building: Option<bool>,

    pub random_tint_color: Option<Color>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub spoil_level: u8,
}

impl ItemPrototypeData {
    pub fn get_icon(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.icon.render(scale, used_mods, image_cache, &())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DarkBackgroundIcon {
    Array {
        dark_background_icons: FactorioArray<IconData>,
    },
    Single {
        dark_background_icon: FileName,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceAsTile {
    pub result: TileID,
    pub condition: CollisionMaskConnector,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub invert: bool,

    pub condition_size: u32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tile_condition: FactorioArray<TileID>,
}

/// [`Types/SendToOrbitMode`](https://lua-api.factorio.com/latest/types/SendToOrbitMode.html)
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SendToOrbitMode {
    #[default]
    NotSendable,
    Manual,
    Automated,
}

// workaround for prototype type string not matching the actual type name
type AmmoPrototype = AmmoItemPrototype;
type BlueprintPrototype = BlueprintItemPrototype;

namespace_struct! {
    AllTypes,
    ItemID,
    "item",
    "ammo",
    "capsule",
    "gun",
    "item-with-entity-data",
    "item-with-label",
    "item-with-inventory",
    "blueprint-book",
    "item-with-tags",
    "selection-tool",
    "blueprint",
    "copy-paste-tool",
    "deconstruction-item",
    "spidertron-remote",
    "upgrade-item",
    "module",
    "rail-planner",
    "space-platform-starter-pack",
    "tool",
    "armor",
    "repair-tool"
}

impl AllTypes {
    pub fn get_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        let id = &ItemID::new(name);

        if self.item.contains_key(id) {
            return self.item[id].get_icon(scale, used_mods, image_cache);
        }

        if self.ammo.contains_key(id) {
            return self.ammo[id].get_icon(scale, used_mods, image_cache);
        }

        if self.capsule.contains_key(id) {
            return self.capsule[id].get_icon(scale, used_mods, image_cache);
        }

        if self.gun.contains_key(id) {
            return self.gun[id].get_icon(scale, used_mods, image_cache);
        }

        if self.item_with_entity_data.contains_key(id) {
            return self.item_with_entity_data[id].get_icon(scale, used_mods, image_cache);
        }

        if self.item_with_label.contains_key(id) {
            return self.item_with_label[id].get_icon(scale, used_mods, image_cache);
        }

        if self.item_with_inventory.contains_key(id) {
            return self.item_with_inventory[id].get_icon(scale, used_mods, image_cache);
        }

        if self.blueprint_book.contains_key(id) {
            return self.blueprint_book[id].get_icon(scale, used_mods, image_cache);
        }

        if self.item_with_tags.contains_key(id) {
            return self.item_with_tags[id].get_icon(scale, used_mods, image_cache);
        }

        if self.selection_tool.contains_key(id) {
            return self.selection_tool[id].get_icon(scale, used_mods, image_cache);
        }

        if self.blueprint.contains_key(id) {
            return self.blueprint[id].get_icon(scale, used_mods, image_cache);
        }

        if self.copy_paste_tool.contains_key(id) {
            return self.copy_paste_tool[id].get_icon(scale, used_mods, image_cache);
        }

        if self.deconstruction_item.contains_key(id) {
            return self.deconstruction_item[id].get_icon(scale, used_mods, image_cache);
        }

        if self.spidertron_remote.contains_key(id) {
            return self.spidertron_remote[id].get_icon(scale, used_mods, image_cache);
        }

        if self.upgrade_item.contains_key(id) {
            return self.upgrade_item[id].get_icon(scale, used_mods, image_cache);
        }

        if self.module.contains_key(id) {
            return self.module[id].get_icon(scale, used_mods, image_cache);
        }

        if self.rail_planner.contains_key(id) {
            return self.rail_planner[id].get_icon(scale, used_mods, image_cache);
        }

        if self.space_platform_starter_pack.contains_key(id) {
            return self.space_platform_starter_pack[id].get_icon(scale, used_mods, image_cache);
        }

        if self.tool.contains_key(id) {
            return self.tool[id].get_icon(scale, used_mods, image_cache);
        }

        if self.armor.contains_key(id) {
            return self.armor[id].get_icon(scale, used_mods, image_cache);
        }

        if self.repair_tool.contains_key(id) {
            return self.repair_tool[id].get_icon(scale, used_mods, image_cache);
        }

        None
    }
}
