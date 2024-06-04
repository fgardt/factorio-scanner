use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{
    Color, FactorioArray, FluidID, Icon, ItemID, ItemSubGroupID, RecipeCategoryID, RecipeID,
    RenderableGraphics,
};

use crate::helper_macro::namespace_struct;

/// [`Prototypes/RecipeCategory`](https://lua-api.factorio.com/latest/prototypes/RecipeCategory.html)
pub type RecipeCategory = crate::BasePrototype<()>;

/// [`Prototypes/RecipePrototype`](https://lua-api.factorio.com/latest/prototypes/RecipePrototype.html)
pub type RecipePrototype = crate::BasePrototype<RecipePrototypeData>;

/// [`Prototypes/RecipePrototype`](https://lua-api.factorio.com/latest/prototypes/RecipePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RecipePrototypeData {
    #[serde(
        default = "crafting_category",
        skip_serializing_if = "is_crafting_category"
    )]
    pub category: RecipeCategoryID,

    pub subgroup: Option<ItemSubGroupID>,
    pub crafting_machine_tint: Option<CraftingMachineTint>,

    #[serde(flatten)]
    pub icon: Option<Icon>,

    #[serde(flatten)]
    pub recipe: DifficultyRecipeData,
}

impl RecipePrototypeData {
    pub fn get_icon(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
        items: &crate::item::AllTypes,
        fluids: &crate::fluid::AllTypes,
    ) -> Option<types::GraphicsOutput> {
        if let Some(icon) = self.icon.as_ref() {
            return icon.render(scale, used_mods, image_cache, &());
        }

        let recipe = self.recipe.get_data();

        match &recipe.results {
            RecipeDataResult::Multiple { results } => {
                if results.is_empty() {
                    return None;
                }

                if results.len() == 1 {
                    return match results.first() {
                        Some(ProductPrototype::Specific(
                            SpecificProductPrototype::FluidProductPrototype { name, .. },
                        )) => fluids.get_icon(name, scale, used_mods, image_cache),
                        Some(
                            ProductPrototype::SimpleItem(name, _)
                            | ProductPrototype::UntaggedItem(ItemProductPrototype { name, .. })
                            | ProductPrototype::Specific(
                                SpecificProductPrototype::ItemProductPrototype(
                                    ItemProductPrototype { name, .. },
                                ),
                            ),
                        ) => items.get_icon(name, scale, used_mods, image_cache),
                        _ => None,
                    };
                }

                let main_product = recipe.main_product.as_ref()?;

                for product in results {
                    match product {
                        ProductPrototype::Specific(
                            SpecificProductPrototype::FluidProductPrototype { name, .. },
                        ) => {
                            if **name == *main_product {
                                return fluids.get_icon(name, scale, used_mods, image_cache);
                            }
                        }
                        ProductPrototype::SimpleItem(name, _)
                        | ProductPrototype::UntaggedItem(ItemProductPrototype { name, .. })
                        | ProductPrototype::Specific(
                            SpecificProductPrototype::ItemProductPrototype(ItemProductPrototype {
                                name,
                                ..
                            }),
                        ) => {
                            if **name == *main_product {
                                return items.get_icon(name, scale, used_mods, image_cache);
                            }
                        }
                    }
                }

                None
            }
            RecipeDataResult::Single { result, .. } => {
                items.get_icon(result, scale, used_mods, image_cache)
            }
        }
    }

    #[must_use]
    pub fn uses_fluid(&self) -> (bool, bool) {
        self.recipe.uses_fluid()
    }
}

fn crafting_category() -> RecipeCategoryID {
    RecipeCategoryID::new("crafting")
}

fn is_crafting_category(category: &RecipeCategoryID) -> bool {
    *category == crafting_category()
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CraftingMachineTint {
    pub primary: Option<Color>,
    pub secondary: Option<Color>,
    pub tertiary: Option<Color>,
    pub quaternary: Option<Color>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DifficultyRecipeData {
    NormalExpensive {
        normal: RecipeData,
        expensive: RecipeData,
    },
    ExpensiveDisabled {
        normal: RecipeData,
        expensive: bool,
    },
    NormalDisabled {
        normal: bool,
        expensive: RecipeData,
    },
    NormalOnly {
        normal: RecipeData,
    },
    ExpensiveOnly {
        expensive: RecipeData,
    },
    Simple {
        #[serde(flatten)]
        data: RecipeData,
    },
}

impl DifficultyRecipeData {
    #[must_use]
    pub const fn get_data(&self) -> &RecipeData {
        match self {
            Self::NormalExpensive { normal, .. }
            | Self::NormalOnly { normal }
            | Self::ExpensiveDisabled { normal, .. } => normal,
            Self::ExpensiveOnly { expensive } | Self::NormalDisabled { expensive, .. } => expensive,
            Self::Simple { data } => data,
        }
    }

    #[must_use]
    pub fn uses_fluid(&self) -> (bool, bool) {
        let data = self.get_data();

        let input = data.ingredients.iter().any(|ingredient| {
            matches!(
                ingredient,
                IngredientPrototype::Specific(
                    SpecificIngredientPrototype::FluidIngredientPrototype { .. },
                )
            )
        });

        let output = match &data.results {
            RecipeDataResult::Multiple { results } => results.iter().any(|result| {
                matches!(
                    result,
                    ProductPrototype::Specific(
                        SpecificProductPrototype::FluidProductPrototype { .. }
                    )
                )
            }),
            RecipeDataResult::Single { .. } => false,
        };

        (input, output)
    }
}

/// [`Types/RecipeData`](https://lua-api.factorio.com/latest/types/RecipeData.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeData {
    pub ingredients: FactorioArray<IngredientPrototype>,

    #[serde(flatten)]
    pub results: RecipeDataResult,

    pub main_product: Option<String>,

    #[serde(
        default = "helper::f64_half",
        skip_serializing_if = "helper::is_half_f64"
    )]
    pub energy_required: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub emissions_multiplier: f64,

    #[serde(
        default = "helper::u32_30",
        deserialize_with = "helper::truncating_deserializer",
        skip_serializing_if = "helper::is_30_u32"
    )]
    pub requester_paste_multiplier: u32,

    #[serde(
        default,
        deserialize_with = "helper::truncating_deserializer",
        skip_serializing_if = "helper::is_default"
    )]
    pub overload_multiplier: u32,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_inserter_overload: bool,

    #[serde(
        default = "helper::bool_true",
        deserialize_with = "helper::bool_deserializer",
        skip_serializing_if = "Clone::clone"
    )]
    pub enabled: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hidden: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hide_from_stats: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hide_from_player_crafting: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_decomposition: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_as_intermediate: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_intermediates: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub always_show_made_in: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_amount_in_title: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub always_show_products: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub unlock_results: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RecipeDataResult {
    Multiple {
        results: FactorioArray<ProductPrototype>,
    },
    Single {
        result: ItemID,

        #[serde(
            default = "helper::u16_1",
            deserialize_with = "helper::truncating_deserializer",
            skip_serializing_if = "helper::is_1_u16"
        )]
        result_count: u16,
    },
}

/// [`Types/IngredientPrototype`](https://lua-api.factorio.com/latest/types/IngredientPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IngredientPrototype {
    SimpleItem(
        ItemID,
        #[serde(deserialize_with = "helper::truncating_deserializer")] u16,
    ),
    Specific(SpecificIngredientPrototype),
    UntaggedItem(ItemIngredientPrototype),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SpecificIngredientPrototype {
    /// [`Types/ItemIngredientPrototype`](https://lua-api.factorio.com/latest/types/ItemIngredientPrototype.html)
    #[serde(rename = "item")]
    ItemIngredientPrototype(ItemIngredientPrototype),

    /// [`Types/FluidIngredientPrototype`](https://lua-api.factorio.com/latest/types/FluidIngredientPrototype.html)
    #[serde(rename = "fluid")]
    FluidIngredientPrototype {
        name: FluidID,
        amount: f64,

        #[serde(flatten)]
        temperature: Option<IngredientTemperature>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        catalyst_amount: f64,

        #[serde(
            default,
            deserialize_with = "helper::truncating_deserializer",
            skip_serializing_if = "helper::is_default"
        )]
        fluidbox_index: u32,
    },
}

/// [`Types/ItemIngredientPrototype`](https://lua-api.factorio.com/latest/types/ItemIngredientPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemIngredientPrototype {
    pub name: ItemID,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub amount: u16,

    #[serde(
        default,
        deserialize_with = "helper::truncating_deserializer",
        skip_serializing_if = "helper::is_default"
    )]
    pub catalyst_amount: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IngredientTemperature {
    Static {
        temperature: f64,
    },
    Range {
        minimum_temperature: f64,
        maximum_temperature: f64,
    },
}

/// [`Types/ProductPrototype`](https://lua-api.factorio.com/latest/types/ProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProductPrototype {
    SimpleItem(
        ItemID,
        #[serde(deserialize_with = "helper::truncating_deserializer")] u16,
    ),
    Specific(SpecificProductPrototype),
    UntaggedItem(ItemProductPrototype),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SpecificProductPrototype {
    /// [`Types/ItemProductPrototype`](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html)
    #[serde(rename = "item")]
    ItemProductPrototype(ItemProductPrototype),

    /// [`Types/FluidProductPrototype`](https://lua-api.factorio.com/latest/types/FluidProductPrototype.html)
    #[serde(rename = "fluid")]
    FluidProductPrototype {
        name: FluidID,

        #[serde(flatten)]
        amount: ProductFluidAmount,

        temperature: Option<f64>,

        #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
        probability: f64,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        catalyst_amount: f64,

        #[serde(
            default,
            deserialize_with = "helper::truncating_deserializer",
            skip_serializing_if = "helper::is_default"
        )]
        fluidbox_index: u32,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        show_details_in_recipe_tooltip: bool,
    },
}

/// [`Types/ItemProductPrototype`](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemProductPrototype {
    pub name: ItemID,

    #[serde(flatten)]
    pub amount: ProductItemAmount,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(
        default,
        deserialize_with = "helper::truncating_deserializer",
        skip_serializing_if = "helper::is_default"
    )]
    pub catalyst_amount: u16,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_details_in_recipe_tooltip: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProductFluidAmount {
    Static { amount: f64 },
    Range { amount_min: f64, amount_max: f64 },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProductItemAmount {
    Static {
        #[serde(deserialize_with = "helper::truncating_deserializer")]
        amount: u16,
    },
    Range {
        #[serde(deserialize_with = "helper::truncating_deserializer")]
        amount_min: u16,

        #[serde(deserialize_with = "helper::truncating_deserializer")]
        amount_max: u16,
    },
}

namespace_struct! {
    AllTypes,
    RecipeID,
    "recipe"
}

impl AllTypes {
    pub fn get_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
        items: &crate::item::AllTypes,
        fluids: &crate::fluid::AllTypes,
    ) -> Option<types::GraphicsOutput> {
        self.recipe
            .get(&RecipeID::new(name))
            .and_then(|recipe| recipe.get_icon(scale, used_mods, image_cache, items, fluids))
    }

    #[must_use]
    pub fn uses_fluid(&self, name: &str) -> (bool, bool) {
        self.recipe
            .get(&RecipeID::new(name))
            .map_or((false, false), |recipe| recipe.uses_fluid())
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn deserialize_empty_barrel() {
        let recipe = r#"{
          "type": "recipe",
          "name": "empty-barrel",
          "category": "crafting",
          "energy_required": 1,
          "subgroup": "intermediate-product",
          "enabled": false,
          "ingredients": 
          [
            
            {
              "type": "item",
              "name": "steel-plate",
              "amount": 1
            }
          ],
          "results": 
          [
            
            {
              "type": "item",
              "name": "empty-barrel",
              "amount": 1
            }
          ]
        }"#;

        let _ = serde_json::from_str::<RecipePrototype>(recipe).unwrap();
    }

    #[test]
    fn deserialize_uranium_processing() {
        let recipe = r#"{
            "type": "recipe",
            "name": "uranium-processing",
            "energy_required": 12,
            "enabled": false,
            "category": "centrifuging",
            "ingredients": 
            [
              
              [
                "uranium-ore",
                10
              ]
            ],
            "icon": "__base__/graphics/icons/uranium-processing.png",
            "icon_size": 64,
            "icon_mipmaps": 4,
            "subgroup": "raw-material",
            "order": "k[uranium-processing]",
            "results": 
            [
              
              {
                "name": "uranium-235",
                "probability": 0.00700000000000000088817841970012523233890533447265625,
                "amount": 1
              },
              
              {
                "name": "uranium-238",
                "probability": 0.992999999999999971578290569595992565155029296875,
                "amount": 1
              }
            ]
          }"#;

        let _ = serde_json::from_str::<RecipePrototype>(recipe).unwrap();
    }
}
