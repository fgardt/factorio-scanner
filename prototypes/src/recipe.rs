use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{Color, FactorioArray, FluidID, Icon, ItemID, ItemSubGroupID, RecipeCategoryID};

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

fn crafting_category() -> RecipeCategoryID {
    "crafting".to_owned()
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

    #[serde(default = "helper::u32_30", skip_serializing_if = "helper::is_30_u32")]
    pub requester_paste_multiplier: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub overload_multiplier: u32,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_inserter_overload: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
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

        #[serde(default = "helper::u16_1", skip_serializing_if = "helper::is_1_u16")]
        result_count: u16,
    },
}

/// [`Types/IngredientPrototype`](https://lua-api.factorio.com/latest/types/IngredientPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IngredientPrototype {
    SimpleItem(ItemID, u16),

    /// [`Types/FluidIngredientPrototype`](https://lua-api.factorio.com/latest/types/FluidIngredientPrototype.html)
    Fluid(FluidIngredientPrototype),

    /// [`Types/ItemIngredientPrototype`](https://lua-api.factorio.com/latest/types/ItemIngredientPrototype.html)
    Item(ItemIngredientPrototype),
}

/// [`Types/FluidIngredientPrototype`](https://lua-api.factorio.com/latest/types/FluidIngredientPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "fluid", tag = "type")]
pub struct FluidIngredientPrototype {
    pub name: FluidID,
    pub amount: f64,

    #[serde(flatten)]
    pub temperature: Option<IngredientTemperature>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub catalyst_amount: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fluidbox_index: u32,
}

/// [`Types/ItemIngredientPrototype`](https://lua-api.factorio.com/latest/types/ItemIngredientPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemIngredientPrototype {
    pub name: ItemID,
    pub amount: u16,

    #[serde(default, skip_serializing_if = "helper::is_default")]
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
    SimpleItem(ItemID, u16),

    /// [`Types/FluidProductPrototype`](https://lua-api.factorio.com/latest/types/FluidProductPrototype.html)
    Fluid(FluidProductPrototype),

    /// [`Types/ItemProductPrototype`](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html)
    Item(ItemProductPrototype),
}

/// [`Types/FluidProductPrototype`](https://lua-api.factorio.com/latest/types/FluidProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "fluid", tag = "type")]
pub struct FluidProductPrototype {
    pub name: FluidID,

    #[serde(flatten)]
    pub amount: ProductAmount<f64>,

    pub temperature: Option<f64>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub catalyst_amount: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fluidbox_index: u32,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_details_in_recipe_tooltip: bool,
}

/// [`Types/ItemProductPrototype`](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemProductPrototype {
    pub name: ItemID,

    #[serde(flatten)]
    pub amount: ProductAmount<u16>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub catalyst_amount: u16,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_details_in_recipe_tooltip: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProductAmount<T> {
    Static { amount: T },
    Range { amount_min: T, amount_max: T },
}
