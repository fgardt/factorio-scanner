use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use types::SpaceLocationID;

use crate::{CircuitCondition, CompareType, EntityNumber, RequestCondition};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StockConnection {
    pub stock: EntityNumber,
    pub front: Option<EntityNumber>,
    pub back: Option<EntityNumber>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schedule {
    pub locomotives: Vec<EntityNumber>,
    pub schedule: ScheduleData,
}

impl crate::GetIDs for Schedule {
    fn get_ids(&self) -> crate::UsedIDs {
        self.schedule.get_ids()
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub records: Vec<ScheduleRecord>,

    pub group: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interrupts: Vec<ScheduleInterrupt>,
}

impl crate::GetIDs for ScheduleData {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.records.get_ids();
        ids.merge(self.interrupts.get_ids());

        ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleRecord {
    pub station: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wait_conditions: Vec<WaitCondition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub temporary: bool,

    #[serde(
        default = "serde_helper::bool_true",
        skip_serializing_if = "Clone::clone"
    )]
    pub allows_unloading: bool,
}

impl crate::GetIDs for ScheduleRecord {
    fn get_ids(&self) -> crate::UsedIDs {
        self.wait_conditions.get_ids()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleInterrupt {
    pub name: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<WaitCondition>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<ScheduleRecord>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub inside_interrupt: bool,
}

impl crate::GetIDs for ScheduleInterrupt {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.conditions.get_ids();
        ids.merge(self.targets.get_ids());

        ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
//#[serde(deny_unknown_fields)] // causes deserialization issues (https://github.com/serde-rs/serde/issues/1358)
pub struct WaitCondition {
    pub compare_type: CompareType,

    #[serde(flatten)]
    pub condition: WaitConditionType,
}

impl crate::GetIDs for WaitCondition {
    fn get_ids(&self) -> crate::UsedIDs {
        match &self.condition {
            WaitConditionType::Circuit { condition }
            | WaitConditionType::ItemCount { condition }
            | WaitConditionType::FluidCount { condition } => condition.get_ids(),
            WaitConditionType::RequestSatisfied { condition }
            | WaitConditionType::RequestNotSatisfied { condition } => condition.get_ids(),
            _ => crate::UsedIDs::default(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WaitConditionType {
    Full,
    Empty,
    #[serde(alias = "not-empty")]
    NotEmpty,
    FuelFull,
    AtStation {
        station: Option<String>,
    },
    NotAtStation {
        station: Option<String>,
    },
    RobotsInactive,
    PassengerPresent,
    PassengerNotPresent,
    AllRequestsSatisfied,
    AnyRequestZero,
    AnyRequestNotSatisfied,
    AnyPlanetImportZero {
        planet: Option<PlanetImportTarget>,
    },
    DestinationFullOrNoPath,
    Time {
        ticks: u32,
    },
    Inactivity {
        ticks: u32,
    },
    DamageTaken {
        damage: u32,
    },
    Circuit {
        condition: Option<CircuitCondition>,
    },
    ItemCount {
        condition: Option<CircuitCondition>,
    },
    FluidCount {
        condition: Option<CircuitCondition>,
    },
    FuelItemCountAll {
        condition: Option<CircuitCondition>,
    },
    FuelItemCountAny {
        condition: Option<CircuitCondition>,
    },
    RequestSatisfied {
        condition: Option<RequestCondition>,
    },
    RequestNotSatisfied {
        condition: Option<RequestCondition>,
    },
    SpecificDestinationFull {
        station: Option<String>,
    },
    SpecificDestinationNotFull {
        station: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlanetImportTarget {
    pub name: SpaceLocationID,
}
