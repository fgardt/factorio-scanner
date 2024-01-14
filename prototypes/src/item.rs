use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use crate::PrototypeMap;

mod ammo;
mod capsule;
mod gun;
mod item_with_entity_data;
mod item_with_label;
mod module;
mod rail_planner;
mod tool;

pub use ammo::*;
pub use capsule::*;
pub use gun::*;
pub use item_with_entity_data::*;
pub use item_with_label::*;
pub use module::*;
pub use rail_planner::*;
pub use tool::*;

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
    pub stack_size: types::ItemCountType,

    #[serde(flatten)]
    pub icon: types::Icon,

    #[serde(flatten)]
    pub dark_background_icon: Option<DarkBackgroundIcon>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub place_result: types::EntityID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub placed_as_equipment_result: types::EquipmentID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fuel_category: types::FuelCategoryID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub burnt_result: types::ItemID,

    pub place_as_tile: Option<PlaceAsTile>,

    pub pictures: Option<types::SpriteVariations>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: types::ItemPrototypeFlags,

    pub default_request_amount: Option<types::ItemCountType>,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub wire_count: types::ItemCountType,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_acceleration_multiplier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_top_speed_multiplier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub fuel_emissions_multiplier: f64,

    pub fuel_glow_color: Option<types::Color>,

    #[serde(flatten)]
    pub rocket_launch_product: Option<RocketLaunchProduct>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DarkBackgroundIcon {
    Array {
        dark_background_icons: types::FactorioArray<types::IconData>,
    },
    Single {
        dark_background_icon: types::FileName,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceAsTile {
    pub result: types::TileID,
    pub condition: types::CollisionMask,
    pub condition_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RocketLaunchProduct {
    Multiple(types::FactorioArray<types::ItemProductPrototype>),
    Single(types::ItemProductPrototype),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AllTypes {
    pub item: PrototypeMap<ItemPrototype>,

    pub ammo: PrototypeMap<AmmoItemPrototype>,

    pub capsule: PrototypeMap<CapsulePrototype>,

    pub gun: PrototypeMap<GunPrototype>,

    pub item_with_entity_data: PrototypeMap<ItemWithEntityDataPrototype>,

    pub item_with_label: PrototypeMap<ItemWithLabelPrototype>,
    pub item_with_inventory: PrototypeMap<ItemWithInventoryPrototype>,
    pub blueprint_book: PrototypeMap<BlueprintBookPrototype>,
    pub item_with_tags: PrototypeMap<ItemWithTagsPrototype>,
    pub selection_tool: PrototypeMap<SelectionToolPrototype>,
    pub blueprint: PrototypeMap<BlueprintItemPrototype>,
    pub copy_paste_tool: PrototypeMap<CopyPasteToolPrototype>,
    pub deconstruction_item: PrototypeMap<DeconstructionItemPrototype>,
    pub upgrade_item: PrototypeMap<UpgradeItemPrototype>,

    pub module: PrototypeMap<ModulePrototype>,

    pub rail_planner: PrototypeMap<RailPlannerPrototype>,

    pub tool: PrototypeMap<ToolPrototype>,
    pub armor: PrototypeMap<ArmorPrototype>,
    pub mining_tool: PrototypeMap<MiningToolPrototype>,
    pub repair_tool: PrototypeMap<RepairToolPrototype>,
    // not implemented
    // pub spidertron_remote: PrototypeMap<SpidertronRemotePrototype>,
}

#[cfg(test)]
mod test {
    use super::*;

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
            order: String::new(),
            child: ItemPrototypeData {
                stack_size: 100,
                // icon: types::Icon::Single {
                //     icon: types::FileName::new("__base__/graphics/icons/iron-plate.png".to_owned()),
                //     icon_size: 64,
                //     icon_mipmaps: 4,
                // },
                icon: types::Icon::Array {
                    icons: types::FactorioArray::new(vec![types::IconData::Default {
                        icon: types::FileName::new(
                            "__base__/graphics/icons/iron-plate.png".to_owned(),
                        ),
                        common: types::CommonIconData {
                            icon_size: None,
                            tint: types::Color::white(),
                            shift: types::Vector::new(0.0, 0.0),
                            scale: None,
                            icon_mipmaps: 4,
                        },
                    }]),
                    icon_size: Some(32),
                    icon_mipmaps: 0,
                },
                dark_background_icon: None,
                place_result: String::new(),
                place_as_tile: None,
                placed_as_equipment_result: String::new(),
                fuel_category: String::new(),
                burnt_result: String::new(),
                pictures: None,
                flags: types::FactorioArray::default(),
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
