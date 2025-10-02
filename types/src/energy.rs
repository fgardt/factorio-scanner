use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    AirbornePollutantID, BurnerUsageID, FactorioArray, FluidAmount, FuelCategoryID, HeatBuffer,
    ItemID, ItemStackIndex,
};

use super::{Direction, FluidBox, MapPosition, helper};

/// [`Types/Energy`](https://lua-api.factorio.com/latest/types/Energy.html)
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Energy(String);

impl Energy {
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(value.into())
    }
}

impl std::ops::Deref for Energy {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::default::Default for Energy {
    fn default() -> Self {
        Self("0J".to_string())
    }
}

/// [`Types/BaseEnergySource`](https://lua-api.factorio.com/latest/types/BaseEnergySource.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseEnergySource<T> {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub emissions_per_minute: HashMap<AirbornePollutantID, f64>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub render_no_power_icon: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub render_no_network_icon: bool,

    #[serde(flatten)]
    child: T,
}

impl<T> std::ops::Deref for BaseEnergySource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

/// [`Types/BurnerEnergySource`](https://lua-api.factorio.com/latest/types/BurnerEnergySource.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BurnerEnergySourceData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub fuel_inventory_size: ItemStackIndex,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub burnt_inventory_size: ItemStackIndex,

    // #[serde(default, skip_serializing_if = "Vec::is_empty"))]
    // smoke: FactorioArray<SmokeSource>,
    // light_flicker: Option<LightFlickeringDefinition>,
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub effectivity: f64,

    #[serde(default = "fuel_usage_id", skip_serializing_if = "is_fuel_usage_id")]
    pub burner_usage: BurnerUsageID,

    #[serde(default = "fuel_category", skip_serializing_if = "is_fuel_category")]
    pub fuel_categories: FactorioArray<FuelCategoryID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub initial_fuel: ItemID,
    #[serde(
        default = "helper::f64_025",
        skip_serializing_if = "helper::is_025_f64"
    )]
    pub initial_fuel_percent: f64,
}

fn fuel_usage_id() -> BurnerUsageID {
    BurnerUsageID::new("burner")
}

fn is_fuel_usage_id(value: &BurnerUsageID) -> bool {
    value == &fuel_usage_id()
}

fn fuel_category() -> FactorioArray<FuelCategoryID> {
    FactorioArray::new(vec![FuelCategoryID::new("chemical")])
}

fn is_fuel_category(value: &FactorioArray<FuelCategoryID>) -> bool {
    value.len() == 1 && value.first() == Some(&FuelCategoryID::new("chemical"))
}

/// [`Types/BurnerEnergySource`](https://lua-api.factorio.com/latest/types/BurnerEnergySource.html)
pub type BurnerEnergySource = BaseEnergySource<BurnerEnergySourceData>;

/// [`Types/ElectricEnergySource`](https://lua-api.factorio.com/latest/types/ElectricEnergySource.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricEnergySourceData {
    pub buffer_capacity: Option<Energy>,
    pub usage_priority: ElectricUsagePriority,
    pub input_flow_limit: Option<Energy>,
    pub output_flow_limit: Option<Energy>,
    pub drain: Option<Energy>,
}

/// [`Types/ElectricEnergySource`](https://lua-api.factorio.com/latest/types/ElectricEnergySource.html)
pub type ElectricEnergySource = BaseEnergySource<ElectricEnergySourceData>;

/// [`Types/FluidEnergySource`](https://lua-api.factorio.com/latest/types/FluidEnergySource.html)
//#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FluidEnergySourceData {
    pub fluid_box: FluidBox,

    // #[serde(default, skip_serializing_if = "Vec::is_empty"))]
    // pub smoke: FactorioArray<SmokeSource>,
    // pub light_flicker: Option<LightFlickeringDefinition>,
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub effectivity: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub burns_fluid: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub scale_fluid_usage: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub destroy_non_fuel_fluid: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fluid_usage_per_tick: FluidAmount,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub maximum_temperature: f32,
}

/// [`Types/FluidEnergySource`](https://lua-api.factorio.com/latest/types/FluidEnergySource.html)
pub type FluidEnergySource = BaseEnergySource<FluidEnergySourceData>;

/// [`Types/HeatEnergySource`](https://lua-api.factorio.com/latest/types/HeatEnergySource.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatEnergySourceData(HeatBuffer);

impl std::ops::Deref for HeatEnergySourceData {
    type Target = HeatBuffer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`Types/HeatEnergySource`](https://lua-api.factorio.com/latest/types/HeatEnergySource.html)
pub type HeatEnergySource = BaseEnergySource<HeatEnergySourceData>;

/// Enumeration of all `EnergySource` types.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AnyEnergySource {
    /// [`Types/BurnerEnergySource`](https://lua-api.factorio.com/latest/types/BurnerEnergySource.html)
    Burner {
        #[serde(flatten)]
        data: Box<BurnerEnergySource>,
    },

    /// [`Types/ElectricEnergySource`](https://lua-api.factorio.com/latest/types/ElectricEnergySource.html)
    Electric {
        #[serde(flatten)]
        data: Box<ElectricEnergySource>,
    },

    /// [`Types/FluidEnergySource`](https://lua-api.factorio.com/latest/types/FluidEnergySource.html)
    Fluid {
        #[serde(flatten)]
        data: Box<FluidEnergySource>,
    },

    /// [`Types/HeatEnergySource`](https://lua-api.factorio.com/latest/types/HeatEnergySource.html)
    Heat {
        #[serde(flatten)]
        data: Box<HeatEnergySource>,
    },

    /// [`Types/VoidEnergySource`](https://lua-api.factorio.com/latest/types/VoidEnergySource.html)
    Void,
}

/// [`Types/ElectricUsagePriority`](https://lua-api.factorio.com/latest/types/ElectricUsagePriority.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ElectricUsagePriority {
    PrimaryInput,
    PrimaryOutput,
    SecondaryInput,
    SecondaryOutput,
    Tertiary,
    Solar,
    Lamp,
}

/// [`Types/HeatConnection`](https://lua-api.factorio.com/latest/types/HeatConnection.html)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeatConnection {
    pub position: MapPosition,
    pub direction: Direction,
}
