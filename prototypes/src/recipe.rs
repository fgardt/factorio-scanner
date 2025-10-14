use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{
    Color, FactorioArray, FluidID, Icon, ItemID, LocalisedString, ModuleCategoryID,
    RecipeCategoryID, RecipeID, RenderableGraphics, SurfaceCondition, TechnologyID,
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

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_categories: FactorioArray<RecipeCategoryID>,

    pub crafting_machine_tint: Option<RecipeTints>,

    #[serde(flatten)]
    pub icon: Option<Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredients: FactorioArray<IngredientPrototype>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: FactorioArray<ProductPrototype>,

    pub main_product: Option<String>,

    #[serde(default = "helper::f64_05", skip_serializing_if = "helper::is_05_f64")]
    pub energy_required: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub emissions_multiplier: f64,

    #[serde(default = "helper::f64_3", skip_serializing_if = "helper::is_3_f64")]
    pub maximum_productivity: f64,

    #[serde(default = "helper::u32_30", skip_serializing_if = "helper::is_30_u32")]
    pub requester_paste_multiplier: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub overload_multiplier: u32,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_inserter_overload: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub enabled: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hide_from_stats: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hide_from_player_crafting: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hide_from_bonus_gui: bool,

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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub preserve_products_in_machine_output: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub result_is_always_fresh: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub reset_freshness_on_craft: bool,

    pub allow_consumption_message: Option<LocalisedString>,
    pub allow_speed_message: Option<LocalisedString>,
    pub allow_productivity_message: Option<LocalisedString>,
    pub allow_pollution_message: Option<LocalisedString>,
    pub allow_quality_message: Option<LocalisedString>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surface_conditions: FactorioArray<SurfaceCondition>,

    pub hide_from_signal_gui: Option<bool>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_consumption: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_speed: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_productivity: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_pollution: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_quality: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_module_categories: FactorioArray<ModuleCategoryID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternative_unlock_methods: FactorioArray<TechnologyID>,
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

        // TODO: icon from self.results
        None
    }

    #[must_use]
    pub fn uses_fluid(&self) -> (bool, bool) {
        let mut ingredient = false;
        let mut product = false;

        for ing in &self.ingredients {
            if matches!(ing, IngredientPrototype::FluidIngredientPrototype { .. }) {
                ingredient = true;
                break;
            }
        }

        for res in &self.results {
            if matches!(res, ProductPrototype::FluidProductPrototype { .. }) {
                product = true;
                break;
            }
        }

        (ingredient, product)
    }
}

fn crafting_category() -> RecipeCategoryID {
    RecipeCategoryID::new("crafting")
}

fn is_crafting_category(category: &RecipeCategoryID) -> bool {
    *category == crafting_category()
}

/// [`Types/RecipeTints`](https://lua-api.factorio.com/latest/types/RecipeTints.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeTints {
    pub primary: Option<Color>,
    pub secondary: Option<Color>,
    pub tertiary: Option<Color>,
    pub quaternary: Option<Color>,
}

/// [`Types/IngredientPrototype`](https://lua-api.factorio.com/latest/types/IngredientPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum IngredientPrototype {
    #[serde(rename = "item")]
    ItemIngredientPrototype {
        name: ItemID,
        amount: NonZeroU16,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        ignored_by_stats: u16,
    },
    #[serde(rename = "fluid")]
    FluidIngredientPrototype {
        name: FluidID,
        amount: FluidAmount,

        #[serde(flatten)]
        temperature: IngredientTemperature,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        ignored_by_stats: FluidAmount,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        fluidbox_index: u32,

        #[serde(default = "helper::u8_2", skip_serializing_if = "helper::is_2_u8")]
        fluidbox_multiplier: u8, // could be NonZeroU8
    },
}

/// [`Types/FluidAmount`](https://lua-api.factorio.com/latest/types/FluidAmount.html)
pub type FluidAmount = f64;

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
#[serde(tag = "type")]
pub enum ProductPrototype {
    #[serde(rename = "item")]
    ItemProductPrototype(ItemProductPrototype),

    #[serde(rename = "fluid")]
    FluidProductPrototype(FluidProductPrototype),
}

/// [`Types/ItemProductPrototype`](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct ItemProductPrototype {
    pub name: ItemID,

    #[serde(flatten)]
    pub amount: ProductItemAmount,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub ignored_by_stats: u16,
    pub ignored_by_productivity: Option<u16>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_details_in_recipe_tooltip: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub extra_count_fraction: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub percent_spoiled: f32,
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

/// [`Types/FluidProductPrototype`](https://lua-api.factorio.com/latest/types/FluidProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct FluidProductPrototype {
    pub name: FluidID,

    #[serde(flatten)]
    pub amount: ProductFluidAmount,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub ignored_by_stats: FluidAmount,
    pub ignored_by_productivity: Option<FluidAmount>,

    pub temperature: Option<f32>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fluidbox_index: u32,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_details_in_recipe_tooltip: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProductFluidAmount {
    Static { amount: f64 },
    Range { amount_min: f64, amount_max: f64 },
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
    pub fn uses_fluid(&self, name: &RecipeID) -> (bool, bool) {
        self.recipe
            .get(name)
            .map_or((false, false), |recipe| recipe.uses_fluid())
    }
}
