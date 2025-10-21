#![allow(clippy::struct_excessive_bools)]

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use crate::{CircuitCondition, SignalID, blueprint::logistics::LogisticSections};

mod combinator;
pub use combinator::*;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CommonControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_enabled: bool,
    pub circuit_condition: Option<CircuitCondition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub connect_to_logistic_network: bool,
    pub logistic_condition: Option<CircuitCondition>,
}

impl crate::GetIDs for CommonControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.circuit_condition.get_ids();
        ids.merge(self.logistic_condition.get_ids());
        ids
    }
}

/// [`AccumulatorBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/AccumulatorBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccumulatorControlBehavior {
    pub output_signal: SignalID,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub read_charge: bool,
}

impl crate::GetIDs for AccumulatorControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        self.output_signal.get_ids()
    }
}

/// [`AgriculturalTowerBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/AgriculturalTowerBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AgriculturalTowerControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_contents: bool,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for AgriculturalTowerControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for AgriculturalTowerControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

/// [`TurretBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/TurretBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TurretControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_priority_list: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_ignore_unlisted_targets: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_ammo: bool,
    pub ignore_unlisted_targets_condition: Option<CircuitCondition>,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for TurretControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for TurretControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl crate::GetIDs for TurretControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.common.get_ids();
        ids.merge(self.ignore_unlisted_targets_condition.get_ids());
        ids
    }
}

/// [`ArtilleryTurretBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/ArtilleryTurretBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ArtilleryTurretControlBehavior {
    pub read_ammo: Option<bool>,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for ArtilleryTurretControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for ArtilleryTurretControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

/// [`AssemblingMachineBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/AssemblingMachineBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssemblingMachineControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_recipe: bool,

    #[serde(flatten)]
    pub(super) shared: FurnaceControlBehavior,
}

impl Deref for AssemblingMachineControlBehavior {
    type Target = FurnaceControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}

impl DerefMut for AssemblingMachineControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shared
    }
}

impl crate::GetIDs for AssemblingMachineControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        self.shared.get_ids()
    }
}

/// [`AsteroidCollectorBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/AsteroidCollectorBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AsteroidCollectorControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_set_filters: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_read_contents: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub include_hands: bool,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for AsteroidCollectorControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for AsteroidCollectorControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

/// [`CargoLandingPadBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/CargoLandingPadBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CargoLandingPadControlBehavior {
    /// Defaults to `send_contents` which should be `0`.
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_mode_of_operation: u8,
}

/// [`BlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/BlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CombinatorControlBehavior {
    /// [`ConstantCombinatorBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/ConstantCombinatorBlueprintControlBehavior.html)
    Constant {
        sections: LogisticSections,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        is_on: bool,
    },
    /// [`DeciderCombinatorBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/DeciderCombinatorBlueprintControlBehavior.html)
    Decider {
        decider_conditions: DeciderCombinatorParameters,
    },
    /// [`ArithmeticCombinatorBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/ArithmeticCombinatorBlueprintControlBehavior.html)
    Arithmetic {
        arithmetic_conditions: ArithmeticCombinatorParameters,
    },
    /// [`SelectorCombinatorParameters`](https://lua-api.factorio.com/latest/concepts/SelectorCombinatorParameters.html)
    Selector(SelectorCombinatorParameters),
}

impl crate::GetIDs for CombinatorControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        match self {
            Self::Constant { .. } => crate::UsedIDs::default(),
            Self::Decider { decider_conditions } => decider_conditions.get_ids(),
            Self::Arithmetic {
                arithmetic_conditions,
            } => arithmetic_conditions.get_ids(),
            Self::Selector(selector_data) => selector_data.get_ids(),
        }
    }
}

/// [`ContainerBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/ContainerBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContainerControlBehavior {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub read_contents: bool,
}

/// [`DisplayPanelBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/DisplayPanelBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DisplayPanelControlBehavior {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<DisplayPanelMessageDefinition>,
}

impl crate::GetIDs for DisplayPanelControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        self.parameters.get_ids()
    }
}

/// [`DisplayPanelMessageDefinition`](https://lua-api.factorio.com/latest/concepts/DisplayPanelMessageDefinition.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DisplayPanelMessageDefinition {
    pub text: String,
    pub icon: SignalID,
    pub condition: CircuitCondition,
}

impl crate::GetIDs for DisplayPanelMessageDefinition {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.icon.get_ids();
        ids.merge(self.condition.get_ids());
        ids
    }
}

/// [`FurnaceBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/FurnaceBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FurnaceControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_contents: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub include_in_crafting: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub include_fuel: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_recipe_finished: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_ingredients: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_working: bool,
    pub recipe_finished_signal: Option<SignalID>,
    pub working_signal: Option<SignalID>,

    #[serde(flatten)]
    pub(super) common: CommonControlBehavior,
}

impl Deref for FurnaceControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for FurnaceControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl crate::GetIDs for FurnaceControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.common.get_ids();
        ids.merge(self.recipe_finished_signal.get_ids());
        ids.merge(self.working_signal.get_ids());
        ids
    }
}

/// [`InserterBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/InserterBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InserterControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_set_filters: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_read_hand_contents: bool,

    /// Defaults to `pulse` which should be `0`.
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_read_hand_mode: u8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_set_stack_size: bool,
    pub stack_control_input_signal: Option<SignalID>,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for InserterControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for InserterControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl crate::GetIDs for InserterControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.common.get_ids();
        ids.merge(self.stack_control_input_signal.get_ids());
        ids
    }
}

// not yet publicly documented in the lua-api
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LampControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub use_colors: bool,

    pub red_signal: Option<SignalID>,
    pub green_signal: Option<SignalID>,
    pub blue_signal: Option<SignalID>,
    pub rgb_signal: Option<SignalID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub color_mode: u8,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for LampControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for LampControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl crate::GetIDs for LampControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.common.get_ids();
        ids.merge(self.red_signal.get_ids());
        ids.merge(self.green_signal.get_ids());
        ids.merge(self.blue_signal.get_ids());
        ids.merge(self.rgb_signal.get_ids());
        ids
    }
}

/// [`LogisticContainerBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/LogisticContainerBlueprintControlBehavior.html)
#[allow(clippy::struct_field_names)]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LogisticContainerControlBehavior {
    /// Defaults to `send_contents` which should be `0`.
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_mode_of_operation: u8,

    pub circuit_condition_enabled: bool,
    pub circuit_condition: Option<CircuitCondition>,
}

impl crate::GetIDs for LogisticContainerControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        self.circuit_condition.get_ids()
    }
}

/// [`MiningDrillBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/MiningDrillBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MiningDrillControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_read_resources: bool,
    pub circuit_resource_read_mode: Option<u8>,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for MiningDrillControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for MiningDrillControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

/// [`ProgrammableSpeakerBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/ProgrammableSpeakerBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProgrammableSpeakerControlBehavior {
    pub circuit_condition: CircuitCondition,
    pub circuit_parameters: ProgrammableSpeakerCircuitParameters,
}

impl crate::GetIDs for ProgrammableSpeakerControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        self.circuit_condition.get_ids()
    }
}

/// [`ProgrammableSpeakerCircuitParameters`](https://lua-api.factorio.com/latest/concepts/ProgrammableSpeakerCircuitParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProgrammableSpeakerCircuitParameters {
    pub signal_value_is_pitch: bool,
    pub stop_playing_sounds: bool,
    pub instrument_id: u32,
    pub note_id: u32,
}

/// [`PumpBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/PumpBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PumpControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_filter: bool,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for PumpControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for PumpControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

/// [`RailSignalBaseBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/RailSignalBaseBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RailSignalControlBehavior {
    pub circuit_closes_signal: bool,
    pub circuit_read_signal: bool,

    pub red_output_signal: Option<SignalID>,
    pub orange_output_signal: Option<SignalID>,
    pub green_output_signal: Option<SignalID>,
    pub blue_output_signal: Option<SignalID>,

    pub circuit_condition: CircuitCondition,
}

impl crate::GetIDs for RailSignalControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.circuit_condition.get_ids();
        ids.merge(self.red_output_signal.get_ids());
        ids.merge(self.orange_output_signal.get_ids());
        ids.merge(self.green_output_signal.get_ids());
        ids.merge(self.blue_output_signal.get_ids());
        ids
    }
}

/// [`ReactorBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/ReactorBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReactorControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_burner_fuel: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_temperature: bool,
    pub temperature_signal: SignalID,
}

impl crate::GetIDs for ReactorControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        self.temperature_signal.get_ids()
    }
}

/// [`RoboportBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/RoboportBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RoboportControlBehavior {
    // TODO: figure out defaults here
    pub read_items_mode: Option<bool>,
    pub read_robot_stats: Option<bool>,

    pub available_logistic_output_signal: Option<SignalID>,
    pub total_logistic_output_signal: Option<SignalID>,
    pub available_construction_output_signal: Option<SignalID>,
    pub total_construction_output_signal: Option<SignalID>,
    pub roboport_count_output_signal: Option<SignalID>,
}

impl crate::GetIDs for RoboportControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.available_construction_output_signal.get_ids();
        ids.merge(self.available_logistic_output_signal.get_ids());
        ids.merge(self.total_construction_output_signal.get_ids());
        ids.merge(self.total_logistic_output_signal.get_ids());
        ids.merge(self.roboport_count_output_signal.get_ids());
        ids
    }
}

/// [`RocketSiloBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/RocketSiloBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RocketSiloControlBehavior {
    #[serde(default = "helper::u8_1", skip_serializing_if = "helper::is_1_u8")]
    pub read_items_mode: u8,
}

/// [`SpacePlatformHubBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/SpacePlatformHubBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SpacePlatformHubControlBehavior {
    pub damage_taken_signal: Option<SignalID>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_damage_taken: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub read_contents: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub send_to_platform: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_moving_from: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_moving_to: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_speed: bool,

    pub speed_signal: Option<SignalID>,
}

impl crate::GetIDs for SpacePlatformHubControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.damage_taken_signal.get_ids();
        ids.merge(self.speed_signal.get_ids());

        ids
    }
}

/// [`SplitterBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/SplitterBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SplitterControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_input_side: bool,
    pub input_left_condition: Option<CircuitCondition>,
    pub input_right_condition: Option<CircuitCondition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_output_side: bool,
    pub output_left_condition: Option<CircuitCondition>,
    pub output_right_condition: Option<CircuitCondition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_filter: bool,
}

impl crate::GetIDs for SplitterControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.input_left_condition.get_ids();
        ids.merge(self.input_right_condition.get_ids());
        ids.merge(self.output_left_condition.get_ids());
        ids.merge(self.output_right_condition.get_ids());
        ids
    }
}

// TODO: check if `read_contents` CBs can be merged
/// [`StorageTankBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/StorageTankBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StorageTankControlBehavior {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub read_contents: bool,
}

/// [`TrainStopBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/TrainStopBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TrainStopControlBehavior {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub send_to_train: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_from_train: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_stopped_train: bool,
    pub train_stopped_signal: Option<SignalID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_trains_limit: bool,
    pub trains_limit_signal: Option<SignalID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub read_trains_count: bool,
    pub trains_count_signal: Option<SignalID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub set_priority: bool,
    pub priority_signal: Option<SignalID>,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for TrainStopControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for TrainStopControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl crate::GetIDs for TrainStopControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.common.get_ids();
        ids.merge(self.train_stopped_signal.get_ids());
        ids.merge(self.trains_limit_signal.get_ids());
        ids.merge(self.trains_count_signal.get_ids());
        ids.merge(self.priority_signal.get_ids());
        ids
    }
}

/// [`TransportBeltBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/TransportBeltBlueprintControlBehavior.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TransportBeltControlBehavior {
    pub circuit_read_hand_contents: bool,
    pub circuit_contents_read_mode: u8,

    #[serde(flatten)]
    common: CommonControlBehavior,
}

impl Deref for TransportBeltControlBehavior {
    type Target = CommonControlBehavior;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for TransportBeltControlBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

/// [`WallBlueprintControlBehavior`](https://lua-api.factorio.com/latest/concepts/WallBlueprintControlBehavior.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WallControlBehavior {
    pub circuit_open_gate: bool,
    pub circuit_read_sensor: bool,

    pub output_signal: Option<SignalID>,
    pub circuit_condition: CircuitCondition,
}

impl crate::GetIDs for WallControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.output_signal.get_ids();
        ids.merge(self.circuit_condition.get_ids());
        ids
    }
}
