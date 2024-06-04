use std::collections::{HashMap, HashSet};

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

type ItemPrototypeMap<T> = HashMap<ItemID, T>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AllTypes {
    pub item: ItemPrototypeMap<ItemPrototype>,

    pub ammo: ItemPrototypeMap<AmmoItemPrototype>,

    pub capsule: ItemPrototypeMap<CapsulePrototype>,

    pub gun: ItemPrototypeMap<GunPrototype>,

    pub item_with_entity_data: ItemPrototypeMap<ItemWithEntityDataPrototype>,

    pub item_with_label: ItemPrototypeMap<ItemWithLabelPrototype>,
    pub item_with_inventory: ItemPrototypeMap<ItemWithInventoryPrototype>,
    pub blueprint_book: ItemPrototypeMap<BlueprintBookPrototype>,
    pub item_with_tags: ItemPrototypeMap<ItemWithTagsPrototype>,
    pub selection_tool: ItemPrototypeMap<SelectionToolPrototype>,
    pub blueprint: ItemPrototypeMap<BlueprintItemPrototype>,
    pub copy_paste_tool: ItemPrototypeMap<CopyPasteToolPrototype>,
    pub deconstruction_item: ItemPrototypeMap<DeconstructionItemPrototype>,
    pub upgrade_item: ItemPrototypeMap<UpgradeItemPrototype>,

    pub module: ItemPrototypeMap<ModulePrototype>,

    pub rail_planner: ItemPrototypeMap<RailPlannerPrototype>,

    pub spidertron_remote: ItemPrototypeMap<SpidertronRemotePrototype>,

    pub tool: ItemPrototypeMap<ToolPrototype>,
    pub armor: ItemPrototypeMap<ArmorPrototype>,
    pub mining_tool: ItemPrototypeMap<MiningToolPrototype>,
    pub repair_tool: ItemPrototypeMap<RepairToolPrototype>,
}

impl crate::IdNamespace for AllTypes {
    type Id = ItemID;

    fn all_ids(&self) -> HashSet<&Self::Id> {
        let mut res = HashSet::new();

        res.extend(self.item.keys());
        res.extend(self.ammo.keys());
        res.extend(self.capsule.keys());
        res.extend(self.gun.keys());
        res.extend(self.item_with_entity_data.keys());
        res.extend(self.item_with_label.keys());
        res.extend(self.item_with_inventory.keys());
        res.extend(self.blueprint_book.keys());
        res.extend(self.item_with_tags.keys());
        res.extend(self.selection_tool.keys());
        res.extend(self.blueprint.keys());
        res.extend(self.copy_paste_tool.keys());
        res.extend(self.deconstruction_item.keys());
        res.extend(self.upgrade_item.keys());
        res.extend(self.module.keys());
        res.extend(self.rail_planner.keys());
        res.extend(self.spidertron_remote.keys());
        res.extend(self.tool.keys());
        res.extend(self.armor.keys());
        res.extend(self.mining_tool.keys());
        res.extend(self.repair_tool.keys());

        res
    }

    fn contains(&self, id: &Self::Id) -> bool {
        self.item.contains_key(id)
            || self.ammo.contains_key(id)
            || self.capsule.contains_key(id)
            || self.gun.contains_key(id)
            || self.item_with_entity_data.contains_key(id)
            || self.item_with_label.contains_key(id)
            || self.item_with_inventory.contains_key(id)
            || self.blueprint_book.contains_key(id)
            || self.item_with_tags.contains_key(id)
            || self.selection_tool.contains_key(id)
            || self.blueprint.contains_key(id)
            || self.copy_paste_tool.contains_key(id)
            || self.deconstruction_item.contains_key(id)
            || self.upgrade_item.contains_key(id)
            || self.module.contains_key(id)
            || self.rail_planner.contains_key(id)
            || self.spidertron_remote.contains_key(id)
            || self.tool.contains_key(id)
            || self.armor.contains_key(id)
            || self.mining_tool.contains_key(id)
            || self.repair_tool.contains_key(id)
    }
}

impl crate::IdNamespaceAccess<ItemPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ItemPrototype> {
        self.item.get(id)
    }
}

impl crate::IdNamespaceAccess<AmmoItemPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&AmmoItemPrototype> {
        self.ammo.get(id)
    }
}

impl crate::IdNamespaceAccess<CapsulePrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&CapsulePrototype> {
        self.capsule.get(id)
    }
}

impl crate::IdNamespaceAccess<GunPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&GunPrototype> {
        self.gun.get(id)
    }
}

impl crate::IdNamespaceAccess<ItemWithEntityDataPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ItemWithEntityDataPrototype> {
        self.item_with_entity_data.get(id)
    }
}

impl crate::IdNamespaceAccess<ItemWithLabelPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ItemWithLabelPrototype> {
        self.item_with_label.get(id)
    }
}

impl crate::IdNamespaceAccess<ItemWithInventoryPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ItemWithInventoryPrototype> {
        self.item_with_inventory.get(id)
    }
}

impl crate::IdNamespaceAccess<BlueprintBookPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&BlueprintBookPrototype> {
        self.blueprint_book.get(id)
    }
}

impl crate::IdNamespaceAccess<ItemWithTagsPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ItemWithTagsPrototype> {
        self.item_with_tags.get(id)
    }
}

impl crate::IdNamespaceAccess<SelectionToolPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&SelectionToolPrototype> {
        self.selection_tool.get(id)
    }
}

impl crate::IdNamespaceAccess<BlueprintItemPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&BlueprintItemPrototype> {
        self.blueprint.get(id)
    }
}

impl crate::IdNamespaceAccess<CopyPasteToolPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&CopyPasteToolPrototype> {
        self.copy_paste_tool.get(id)
    }
}

impl crate::IdNamespaceAccess<DeconstructionItemPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&DeconstructionItemPrototype> {
        self.deconstruction_item.get(id)
    }
}

impl crate::IdNamespaceAccess<UpgradeItemPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&UpgradeItemPrototype> {
        self.upgrade_item.get(id)
    }
}

impl crate::IdNamespaceAccess<ModulePrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ModulePrototype> {
        self.module.get(id)
    }
}

impl crate::IdNamespaceAccess<RailPlannerPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&RailPlannerPrototype> {
        self.rail_planner.get(id)
    }
}

impl crate::IdNamespaceAccess<SpidertronRemotePrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&SpidertronRemotePrototype> {
        self.spidertron_remote.get(id)
    }
}

impl crate::IdNamespaceAccess<ToolPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ToolPrototype> {
        self.tool.get(id)
    }
}

impl crate::IdNamespaceAccess<ArmorPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&ArmorPrototype> {
        self.armor.get(id)
    }
}

impl crate::IdNamespaceAccess<MiningToolPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&MiningToolPrototype> {
        self.mining_tool.get(id)
    }
}

impl crate::IdNamespaceAccess<RepairToolPrototype> for AllTypes {
    fn get_proto(&self, id: &Self::Id) -> Option<&RepairToolPrototype> {
        self.repair_tool.get(id)
    }
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
            order: String::new(),
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
