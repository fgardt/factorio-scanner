use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use types::{ArithmeticOperation, Comparator, QualityID, SelectorOperation};

use crate::{CompareType, NameString, QualityCondition, SignalID};

/// [`CircuitNetworkSelection`](https://lua-api.factorio.com/latest/concepts/CircuitNetworkSelection.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CircuitNetworkSelection {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub red: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub green: bool,
}

/// [`ArithmeticCombinatorParameters`](https://lua-api.factorio.com/latest/concepts/ArithmeticCombinatorParameters.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum ArithmeticCombinatorParameters {
    SignalSignal {
        first_signal: Option<SignalID>,
        first_signal_networks: Option<CircuitNetworkSelection>,
        #[serde(default)]
        operation: ArithmeticOperation,
        second_signal: Option<SignalID>,
        second_signal_networks: Option<CircuitNetworkSelection>,
        output_signal: Option<SignalID>,
    },
    SignalConstant {
        first_signal: Option<SignalID>,
        first_signal_networks: Option<CircuitNetworkSelection>,
        #[serde(default)]
        operation: ArithmeticOperation,
        #[serde(default)]
        second_constant: i32,
        output_signal: Option<SignalID>,
    },
    ConstantSignal {
        #[serde(default)]
        first_constant: i32,
        #[serde(default)]
        operation: ArithmeticOperation,
        second_signal: Option<SignalID>,
        second_signal_networks: Option<CircuitNetworkSelection>,
        output_signal: Option<SignalID>,
    },
    ConstantConstant {
        #[serde(default)]
        first_constant: i32,
        #[serde(default)]
        operation: ArithmeticOperation,
        #[serde(default)]
        second_constant: i32,
        output_signal: Option<SignalID>,
    },
}

impl ArithmeticCombinatorParameters {
    #[must_use]
    pub const fn operation(&self) -> ArithmeticOperation {
        match self {
            Self::SignalSignal { operation, .. }
            | Self::SignalConstant { operation, .. }
            | Self::ConstantSignal { operation, .. }
            | Self::ConstantConstant { operation, .. } => *operation,
        }
    }
}

impl crate::GetIDs for ArithmeticCombinatorParameters {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            Self::SignalSignal {
                first_signal,
                second_signal,
                output_signal,
                ..
            } => {
                ids.merge(first_signal.get_ids());
                ids.merge(second_signal.get_ids());
                ids.merge(output_signal.get_ids());
            }
            Self::SignalConstant {
                first_signal,
                output_signal,
                ..
            } => {
                ids.merge(first_signal.get_ids());
                ids.merge(output_signal.get_ids());
            }
            Self::ConstantSignal {
                second_signal,
                output_signal,
                ..
            } => {
                ids.merge(second_signal.get_ids());
                ids.merge(output_signal.get_ids());
            }
            Self::ConstantConstant { output_signal, .. } => {
                ids.merge(output_signal.get_ids());
            }
        }

        ids
    }
}

/// [`DeciderCombinatorParameters`](https://lua-api.factorio.com/latest/concepts/DeciderCombinatorParameters.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeciderCombinatorParameters {
    pub conditions: Vec<DeciderCombinatorCondition>,
    pub outputs: Vec<DeciderCombinatorOutput>,
}

impl DeciderCombinatorParameters {
    #[must_use]
    pub fn operation(&self) -> Comparator {
        self.conditions
            .first()
            .map_or(Comparator::Unknown, DeciderCombinatorCondition::comparator)
    }
}

impl crate::GetIDs for DeciderCombinatorParameters {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.conditions.get_ids();
        ids.merge(self.outputs.get_ids());

        ids
    }
}

/// [`DeciderCombinatorCondition`](https://lua-api.factorio.com/latest/concepts/DeciderCombinatorCondition.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum DeciderCombinatorCondition {
    Signal {
        first_signal: Option<SignalID>,
        first_signal_networks: Option<CircuitNetworkSelection>,
        second_signal: SignalID,
        second_signal_networks: Option<CircuitNetworkSelection>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        comparator: Comparator,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        compare_type: CompareType,
    },
    Constant {
        first_signal: Option<SignalID>,
        first_signal_networks: Option<CircuitNetworkSelection>,

        #[serde(default)]
        constant: i32,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        comparator: Comparator,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        compare_type: CompareType,
    },
}

impl DeciderCombinatorCondition {
    #[must_use]
    pub const fn comparator(&self) -> Comparator {
        match self {
            Self::Signal { comparator, .. } | Self::Constant { comparator, .. } => *comparator,
        }
    }
}

impl crate::GetIDs for DeciderCombinatorCondition {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            Self::Signal {
                first_signal,
                second_signal,
                ..
            } => {
                ids.merge(first_signal.get_ids());
                ids.merge(second_signal.get_ids());
            }
            Self::Constant { first_signal, .. } => {
                ids.merge(first_signal.get_ids());
            }
        }

        ids
    }
}

/// [`DeciderCombinatorOutput`](https://lua-api.factorio.com/latest/concepts/DeciderCombinatorOutput.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeciderCombinatorOutput {
    pub signal: Option<SignalID>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub copy_count_from_input: bool,

    #[serde(
        default = "serde_helper::i32_1",
        skip_serializing_if = "serde_helper::is_1_i32"
    )]
    pub constant: i32,

    pub networks: Option<CircuitNetworkSelection>,
}

impl crate::GetIDs for DeciderCombinatorOutput {
    fn get_ids(&self) -> crate::UsedIDs {
        self.signal.get_ids()
    }
}

/// [`SelectorCombinatorParameters`](https://lua-api.factorio.com/latest/concepts/SelectorCombinatorParameters.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "operation", rename_all = "kebab-case")]
pub enum SelectorCombinatorParameters {
    Select {
        #[serde(
            default = "serde_helper::bool_true",
            skip_serializing_if = "Clone::clone"
        )]
        select_max: bool,
        index_signal: Option<SignalID>,
        #[serde(default, skip_serializing_if = "serde_helper::is_default")]
        index_constant: i32,
    },
    Count {
        count_signal: Option<SignalID>,
    },
    Random {
        #[serde(default, skip_serializing_if = "serde_helper::is_default")]
        random_update_interval: u32,
    },
    StackSize,
    RocketCapacity,
    QualityFilter {
        quality_filter: Option<QualityCondition>,
    },
    QualityTransfer {
        #[serde(default, skip_serializing_if = "serde_helper::is_default")]
        select_quality_from_signal: bool,
        quality_source_signal: Option<SignalID>,
        quality_source_static: Option<NameString<QualityID>>,
        quality_destination_signal: Option<SignalID>,
    },
}

impl SelectorCombinatorParameters {
    #[must_use]
    pub const fn operation(&self) -> SelectorOperation {
        match self {
            Self::Select { select_max, .. } => SelectorOperation::Select {
                select_max: *select_max,
            },
            Self::Count { .. } => SelectorOperation::Count,
            Self::Random { .. } => SelectorOperation::Random,
            Self::StackSize => SelectorOperation::StackSize,
            Self::RocketCapacity => SelectorOperation::RocketCapacity,
            Self::QualityFilter { .. } => SelectorOperation::QualityFilter,
            Self::QualityTransfer { .. } => SelectorOperation::QualityTransfer,
        }
    }
}

impl crate::GetIDs for SelectorCombinatorParameters {
    fn get_ids(&self) -> crate::UsedIDs {
        match self {
            Self::Select { index_signal, .. } => index_signal.get_ids(),
            Self::Count { count_signal, .. } => count_signal.get_ids(),
            Self::Random { .. } | Self::StackSize | Self::RocketCapacity => {
                crate::UsedIDs::default()
            }
            Self::QualityFilter { quality_filter, .. } => quality_filter
                .as_ref()
                .map(|f| f.quality.get_ids())
                .unwrap_or_default(),
            Self::QualityTransfer {
                quality_source_signal,
                quality_source_static,
                quality_destination_signal,
                ..
            } => {
                let mut ids = crate::UsedIDs::default();

                ids.merge(quality_source_signal.get_ids());
                ids.merge(quality_destination_signal.get_ids());
                ids.merge(quality_source_static.get_ids());

                ids
            }
        }
    }
}
