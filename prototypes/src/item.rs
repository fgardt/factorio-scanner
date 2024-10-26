use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{
    CollisionMask, Color, EntityID, EquipmentID, FactorioArray, FileName, FuelCategoryID, Icon,
    IconData, ItemCountType, ItemID, ItemProductPrototype, ItemPrototypeFlags, RenderableGraphics,
    SpriteVariations, TileID,
};

mod ammo;
mod capsule;
mod gun;
mod item_with_entity_data;
mod item_with_label;
mod module;
mod rail_planner;
mod spidertron_remote;
mod tool;

pub use ammo::*;
pub use capsule::*;
pub use gun::*;
pub use item_with_entity_data::*;
pub use item_with_label::*;
pub use module::*;
pub use rail_planner::*;
pub use spidertron_remote::*;
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
    pub placed_as_equipment_result: EquipmentID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fuel_category: FuelCategoryID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub burnt_result: ItemID,

    pub place_as_tile: Option<PlaceAsTile>,

    pub pictures: Option<SpriteVariations>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: ItemPrototypeFlags,

    pub default_request_amount: Option<ItemCountType>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub wire_count: ItemCountType,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_acceleration_multiplier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_top_speed_multiplier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_emissions_multiplier: f64,

    pub fuel_glow_color: Option<Color>,

    #[serde(flatten)]
    pub rocket_launch_product: Option<RocketLaunchProduct>,
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
    pub condition: CollisionMask,
    pub condition_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RocketLaunchProduct {
    Multiple(FactorioArray<ItemProductPrototype>),
    Single(ItemProductPrototype),
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
    "upgrade-item",
    "module",
    "rail-planner",
    "spidertron-remote",
    "tool",
    "armor",
    "mining-tool",
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

        if self.upgrade_item.contains_key(id) {
            return self.upgrade_item[id].get_icon(scale, used_mods, image_cache);
        }

        if self.module.contains_key(id) {
            return self.module[id].get_icon(scale, used_mods, image_cache);
        }

        if self.rail_planner.contains_key(id) {
            return self.rail_planner[id].get_icon(scale, used_mods, image_cache);
        }

        if self.spidertron_remote.contains_key(id) {
            return self.spidertron_remote[id].get_icon(scale, used_mods, image_cache);
        }

        if self.tool.contains_key(id) {
            return self.tool[id].get_icon(scale, used_mods, image_cache);
        }

        if self.armor.contains_key(id) {
            return self.armor[id].get_icon(scale, used_mods, image_cache);
        }

        if self.mining_tool.contains_key(id) {
            return self.mining_tool[id].get_icon(scale, used_mods, image_cache);
        }

        if self.repair_tool.contains_key(id) {
            return self.repair_tool[id].get_icon(scale, used_mods, image_cache);
        }

        None
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use types::{CommonIconData, Vector};

    #[test]
    fn deserialize() {
        let json = r#"{
            "type": "item",
            "name": "iron-plate",
            "localised_name": [
                "item-name.iron-plate"
            ],
            "icons": 
            [
                {
                    "icon": "__base__/graphics/icons/linked-belt.png",
                    "icon_size": 64,
                    "icon_mipmaps": 4,
                    "tint": 
                    {
                        "r": 1.0,
                        "g": 0.5,
                        "b": 1.0,
                        "a": 1.0
                    }
                }
            ],
            "stack_size": 100,
            "flags": {},
            "fuel_category": "chemical",
            "fuel_value": "2MJ",
            "fuel_acceleration_multiplier": 1.2,
            "fuel_top_speed_multiplier": 1.05,
            "fuel_emissions_multiplier": 1.2,
            "subgroup": "raw-material",
            "order": "a[iron-plate]"
        }"#;

        let _ = serde_json::from_str::<ItemPrototype>(json).unwrap();
    }

    #[test]
    fn deserialize_ee_infinity_pipe() {
        let json = r#"{
            "type": "item",
            "name": "ee-infinity-pipe",
            "icons": [
                {
                    "icon": "__base__/graphics/icons/pipe.png",
                    "tint": 
                    {
                    "r": 1,
                    "g": 0.5,
                    "b": 1,
                    "a": 1
                    }
                }
            ],
            "icon_size": 64,
            "icon_mipmaps": 4,
            "flags": {},
            "subgroup": "ee-misc",
            "order": "ba",
            "stack_size": 50,
            "place_result": "ee-infinity-pipe-100"
        }"#;

        let _ = serde_json::from_str::<ItemPrototype>(json).unwrap();
    }

    #[test]
    fn serialize() {
        let item = ItemPrototype(super::super::BasePrototype {
            type_: "item".to_owned(),
            name: "iron-plate".to_owned(),
            localised_name: None,
            localised_description: None,
            factoriopedia_description: None,
            order: String::new(),
            parameter: false,
            hidden: false,
            hidden_in_factoriopedia: None,
            child: ItemPrototypeData {
                stack_size: 100,
                // icon: Icon::Single {
                //     icon: FileName::new("__base__/graphics/icons/iron-plate.png".to_owned()),
                //     icon_size: 64,
                //     icon_mipmaps: 4,
                // },
                icon: Icon::Array {
                    icons: FactorioArray::new(vec![IconData::Default {
                        icon: FileName::new("__base__/graphics/icons/iron-plate.png".to_owned()),
                        common: CommonIconData {
                            icon_size: None,
                            tint: Color::white(),
                            shift: Vector::new(0.0, 0.0),
                            scale: None,
                            icon_mipmaps: 4,
                        },
                    }]),
                    icon_size: Some(32),
                    icon_mipmaps: 0,
                },
                dark_background_icon: None,
                place_result: EntityID::new(""),
                place_as_tile: None,
                placed_as_equipment_result: EquipmentID::new(""),
                fuel_category: FuelCategoryID::new(""),
                burnt_result: ItemID::new(""),
                pictures: None,
                flags: FactorioArray::default(),
                default_request_amount: None,
                wire_count: 0,
                fuel_acceleration_multiplier: 1.0,
                fuel_top_speed_multiplier: 1.0,
                fuel_emissions_multiplier: 1.0,
                fuel_glow_color: None,
                rocket_launch_product: None,
            },
        });

        let serialized = serde_json::to_string_pretty(&item).unwrap();

        //println!("{serialized}");
    }
}
