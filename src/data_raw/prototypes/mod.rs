#![allow(clippy::struct_excessive_bools, clippy::module_name_repetitions)]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::helper;
use super::types::GraphicsOutput;
#[allow(clippy::wildcard_imports)]
use super::types::{Animation, BoxSpecification, LocalisedString, Order, Sprite};

mod entity;
pub use entity::RenderOpts as EntityRenderOpts;
pub use entity::Renderable as RenderableEntity;
pub use entity::*;

/// [`Prototypes/PrototypeBase`](https://lua-api.factorio.com/latest/PrototypeBase.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BasePrototype<T> {
    /// type can effectively be ignored, as it should be enforced by the struct/enum types itself
    #[serde(rename = "type")]
    pub type_: String,

    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub order: Order,

    pub localised_name: Option<LocalisedString>,
    pub localised_description: Option<LocalisedString>,

    #[serde(flatten)]
    pub child: T,
}

/// [`Prototypes/UtilitySprites/CursorBoxSpecification`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#cursor_box)
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorBoxSpecification {
    pub regular: Vec<BoxSpecification>,
    pub not_allowed: Vec<BoxSpecification>,
    pub copy: Vec<BoxSpecification>,
    pub electricity: Vec<BoxSpecification>,
    pub logistics: Vec<BoxSpecification>,
    pub pair: Vec<BoxSpecification>,
    pub train_visualization: Vec<BoxSpecification>,
    pub blueprint_snap_rectangle: Vec<BoxSpecification>,
}

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UtilitySpritesData {
    pub cursor_box: CursorBoxSpecification,
    pub clouds: Animation,
    pub arrow_button: Animation,
    pub explosion_chart_visualization: Animation,
    pub refresh_white: Animation,

    #[serde(flatten)]
    pub sprites: HashMap<String, Sprite>,
}

pub type UtilitySprites = BasePrototype<UtilitySpritesData>;

pub type PrototypeMap<T> = HashMap<String, T>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataRaw {
    pub accumulator: EntityPrototypeMap<AccumulatorPrototype>,
    pub artillery_turret: EntityPrototypeMap<ArtilleryTurretPrototype>,
    pub beacon: EntityPrototypeMap<BeaconPrototype>,
    pub boiler: EntityPrototypeMap<BoilerPrototype>,
    pub burner_generator: EntityPrototypeMap<BurnerGeneratorPrototype>,

    pub arithmetic_combinator: EntityPrototypeMap<ArithmeticCombinatorPrototype>,
    pub decider_combinator: EntityPrototypeMap<DeciderCombinatorPrototype>,
    pub constant_combinator: EntityPrototypeMap<ConstantCombinatorPrototype>,
    pub programmable_speaker: EntityPrototypeMap<ProgrammableSpeakerPrototype>,

    pub container: EntityPrototypeMap<ContainerPrototype>,
    pub logistic_container: EntityPrototypeMap<LogisticContainerPrototype>,
    pub infinity_container: EntityPrototypeMap<InfinityContainerPrototype>,
    pub linked_container: EntityPrototypeMap<LinkedContainerPrototype>,

    pub assembling_machine: EntityPrototypeMap<AssemblingMachinePrototype>,
    pub rocket_silo: EntityPrototypeMap<RocketSiloPrototype>,
    pub furnace: EntityPrototypeMap<FurnacePrototype>,

    pub electric_energy_interface: EntityPrototypeMap<ElectricEnergyInterfacePrototype>,
    pub electric_pole: EntityPrototypeMap<ElectricPolePrototype>,
    pub power_switch: EntityPrototypeMap<PowerSwitchPrototype>,

    pub combat_robot: EntityPrototypeMap<CombatRobotPrototype>,
    pub construction_robot: EntityPrototypeMap<ConstructionRobotPrototype>,
    pub logistic_robot: EntityPrototypeMap<LogisticRobotPrototype>,
    pub roboport: EntityPrototypeMap<RoboportPrototype>,

    pub gate: EntityPrototypeMap<GatePrototype>,
    pub wall: EntityPrototypeMap<WallPrototype>,

    pub generator: EntityPrototypeMap<GeneratorPrototype>,

    pub reactor: EntityPrototypeMap<ReactorPrototype>,
    pub heat_interface: EntityPrototypeMap<HeatInterfacePrototype>,
    pub heat_pipe: EntityPrototypeMap<HeatPipePrototype>,

    pub inserter: EntityPrototypeMap<InserterPrototype>,

    pub lab: EntityPrototypeMap<LabPrototype>,

    pub lamp: EntityPrototypeMap<LampPrototype>,

    pub land_mine: EntityPrototypeMap<LandMinePrototype>,

    pub mining_drill: EntityPrototypeMap<MiningDrillPrototype>,
    pub offshore_pump: EntityPrototypeMap<OffshorePumpPrototype>,

    pub pipe: EntityPrototypeMap<PipePrototype>,
    pub infinity_pipe: EntityPrototypeMap<InfinityPipePrototype>,
    pub pipe_to_ground: EntityPrototypeMap<PipeToGroundPrototype>,
    pub pump: EntityPrototypeMap<PumpPrototype>,

    pub simple_entity: EntityPrototypeMap<SimpleEntityPrototype>,
    pub simple_entity_with_owner: EntityPrototypeMap<SimpleEntityWithOwnerPrototype>,
    pub simple_entity_with_force: EntityPrototypeMap<SimpleEntityWithForcePrototype>,

    pub solar_panel: EntityPrototypeMap<SolarPanelPrototype>,

    pub storage_tank: EntityPrototypeMap<StorageTankPrototype>,

    pub linked_belt: EntityPrototypeMap<LinkedBeltPrototype>,
    pub loader_1x1: EntityPrototypeMap<Loader1x1Prototype>,
    pub loader: EntityPrototypeMap<Loader1x2Prototype>,
    pub splitter: EntityPrototypeMap<SplitterPrototype>,
    pub transport_belt: EntityPrototypeMap<TransportBeltPrototype>,
    pub underground_belt: EntityPrototypeMap<UndergroundBeltPrototype>,

    pub radar: EntityPrototypeMap<RadarPrototype>,
    pub turret: EntityPrototypeMap<TurretPrototype>,
    pub ammo_turret: EntityPrototypeMap<AmmoTurretPrototype>,
    pub electric_turret: EntityPrototypeMap<ElectricTurretPrototype>,
    pub fluid_turret: EntityPrototypeMap<FluidTurretPrototype>,

    pub car: EntityPrototypeMap<CarPrototype>,

    pub curved_rail: EntityPrototypeMap<CurvedRailPrototype>,
    pub straight_rail: EntityPrototypeMap<StraightRailPrototype>,
    pub rail_signal: EntityPrototypeMap<RailSignalPrototype>,
    pub rail_chain_signal: EntityPrototypeMap<RailChainSignalPrototype>,
    pub train_stop: EntityPrototypeMap<TrainStopPrototype>,
    pub locomotive: EntityPrototypeMap<LocomotivePrototype>,
    pub cargo_wagon: EntityPrototypeMap<CargoWagonPrototype>,
    pub fluid_wagon: EntityPrototypeMap<FluidWagonPrototype>,
    pub artillery_wagon: EntityPrototypeMap<ArtilleryWagonPrototype>,

    pub utility_sprites: PrototypeMap<UtilitySprites>,
    // not implemented
    // pub character: EntityPrototypeMap<CharacterPrototype>,
    // pub unit_spawner: EntityPrototypeMap<EnemySpawnerPrototype>,
    // pub player_port: EntityPrototypeMap<PlayerPortPrototype>,
    // pub unit: EntityPrototypeMap<UnitPrototype>,
    // pub spider_vehicle: EntityPrototypeMap<SpiderVehiclePrototype>,
}

pub enum EntityType {
    Accumulator,
    ArtilleryTurret,
    Beacon,
    Boiler,
    BurnerGenerator,
    ArithmeticCombinator,
    DeciderCombinator,
    ConstantCombinator,
    ProgrammableSpeaker,
    Container,
    LogisticContainer,
    InfinityContainer,
    LinkedContainer,
    AssemblingMachine,
    RocketSilo,
    Furnace,
    ElectricEnergyInterface,
    ElectricPole,
    PowerSwitch,
    CombatRobot,
    ConstructionRobot,
    LogisticRobot,
    Roboport,
    Gate,
    Wall,
    Generator,
    Reactor,
    HeatInterface,
    HeatPipe,
    Inserter,
    Lab,
    Lamp,
    LandMine,
    MiningDrill,
    OffshorePump,
    Pipe,
    InfinityPipe,
    PipeToGround,
    Pump,
    SimpleEntityWithOwner,
    SimpleEntityWithForce,
    SolarPanel,
    StorageTank,
    LinkedBelt,
    Loader1x1,
    Loader,
    Splitter,
    TransportBelt,
    UndergroundBelt,
    Radar,
    Turret,
    AmmoTurret,
    ElectricTurret,
    FluidTurret,
    Car,
    CurvedRail,
    StraightRail,
    RailSignal,
    RailChainSignal,
    TrainStop,
    Locomotive,
    CargoWagon,
    FluidWagon,
    ArtilleryWagon,
}

pub struct DataUtil {
    data: DataRaw,

    entities: HashMap<String, EntityType>,
}

impl DataUtil {
    #[allow(clippy::too_many_lines)]
    pub fn new(data: DataRaw) -> Self {
        let mut entities: HashMap<String, EntityType> = HashMap::new();

        {
            (*data.accumulator).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Accumulator);
            });

            (*data.artillery_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ArtilleryTurret);
            });

            (*data.beacon).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Beacon);
            });

            (*data.boiler).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Boiler);
            });

            (*data.burner_generator).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::BurnerGenerator);
            });

            (*data.arithmetic_combinator).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ArithmeticCombinator);
            });

            (*data.decider_combinator).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::DeciderCombinator);
            });

            (*data.constant_combinator).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ConstantCombinator);
            });

            (*data.programmable_speaker).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ProgrammableSpeaker);
            });

            (*data.container).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Container);
            });

            (*data.logistic_container).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::LogisticContainer);
            });

            (*data.infinity_container).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::InfinityContainer);
            });

            (*data.linked_container).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::LinkedContainer);
            });

            (*data.assembling_machine).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::AssemblingMachine);
            });

            (*data.rocket_silo).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::RocketSilo);
            });

            (*data.furnace).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Furnace);
            });

            (*data.electric_energy_interface)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), EntityType::ElectricEnergyInterface);
                });

            (*data.electric_pole).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ElectricPole);
            });

            (*data.power_switch).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::PowerSwitch);
            });

            (*data.combat_robot).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::CombatRobot);
            });

            (*data.construction_robot).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ConstructionRobot);
            });

            (*data.logistic_robot).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::LogisticRobot);
            });

            (*data.roboport).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Roboport);
            });

            (*data.gate).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Gate);
            });

            (*data.gate).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Gate);
            });

            (*data.wall).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Wall);
            });

            (*data.generator).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Generator);
            });

            (*data.reactor).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Reactor);
            });

            (*data.heat_interface).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::HeatInterface);
            });

            (*data.heat_pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::HeatPipe);
            });

            (*data.inserter).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Inserter);
            });

            (*data.lab).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Lab);
            });

            (*data.lamp).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Lamp);
            });

            (*data.land_mine).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::LandMine);
            });

            (*data.mining_drill).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::MiningDrill);
            });

            (*data.offshore_pump).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::OffshorePump);
            });

            (*data.pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Pipe);
            });

            (*data.infinity_pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::InfinityPipe);
            });

            (*data.pipe_to_ground).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::PipeToGround);
            });

            (*data.pump).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Pump);
            });

            (*data.simple_entity_with_owner)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), EntityType::SimpleEntityWithOwner);
                });

            (*data.simple_entity_with_force)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), EntityType::SimpleEntityWithForce);
                });

            (*data.solar_panel).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::SolarPanel);
            });

            (*data.storage_tank).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::StorageTank);
            });

            (*data.linked_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::LinkedBelt);
            });

            (*data.loader_1x1).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Loader1x1);
            });

            (*data.loader).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Loader);
            });

            (*data.splitter).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Splitter);
            });

            (*data.transport_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::TransportBelt);
            });

            (*data.underground_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::UndergroundBelt);
            });

            (*data.radar).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Radar);
            });

            (*data.turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Turret);
            });

            (*data.ammo_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::AmmoTurret);
            });

            (*data.electric_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ElectricTurret);
            });

            (*data.fluid_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::FluidTurret);
            });

            (*data.car).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Car);
            });

            (*data.curved_rail).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::CurvedRail);
            });

            (*data.straight_rail).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::StraightRail);
            });

            (*data.rail_signal).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::RailSignal);
            });

            (*data.rail_chain_signal).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::RailChainSignal);
            });

            (*data.train_stop).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::TrainStop);
            });

            (*data.locomotive).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::Locomotive);
            });

            (*data.cargo_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::CargoWagon);
            });

            (*data.fluid_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::FluidWagon);
            });

            (*data.artillery_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), EntityType::ArtilleryWagon);
            });
        }

        Self { data, entities }
    }

    #[allow(clippy::too_many_lines)]
    pub fn get_entity(&self, name: &str) -> Option<&dyn RenderableEntity> {
        let entity_type = self.entities.get(name)?;

        match entity_type {
            EntityType::Accumulator => self
                .data
                .accumulator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ArtilleryTurret => self
                .data
                .artillery_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Beacon => self
                .data
                .beacon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Boiler => self
                .data
                .boiler
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::BurnerGenerator => self
                .data
                .burner_generator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ArithmeticCombinator => self
                .data
                .arithmetic_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::DeciderCombinator => self
                .data
                .decider_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ConstantCombinator => self
                .data
                .constant_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ProgrammableSpeaker => self
                .data
                .programmable_speaker
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Container => self
                .data
                .container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::LogisticContainer => self
                .data
                .logistic_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::InfinityContainer => self
                .data
                .infinity_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::LinkedContainer => self
                .data
                .linked_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::AssemblingMachine => self
                .data
                .assembling_machine
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::RocketSilo => self
                .data
                .rocket_silo
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Furnace => self
                .data
                .furnace
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ElectricEnergyInterface => self
                .data
                .electric_energy_interface
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ElectricPole => self
                .data
                .electric_pole
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::PowerSwitch => self
                .data
                .power_switch
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::CombatRobot => self
                .data
                .combat_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ConstructionRobot => self
                .data
                .construction_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::LogisticRobot => self
                .data
                .logistic_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Roboport => self
                .data
                .roboport
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Gate => self.data.gate.get(name).map(|x| x as &dyn RenderableEntity),
            EntityType::Wall => self.data.wall.get(name).map(|x| x as &dyn RenderableEntity),
            EntityType::Generator => self
                .data
                .generator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Reactor => self
                .data
                .reactor
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::HeatInterface => self
                .data
                .heat_interface
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::HeatPipe => self
                .data
                .heat_pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Inserter => self
                .data
                .inserter
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Lab => self.data.lab.get(name).map(|x| x as &dyn RenderableEntity),
            EntityType::Lamp => self.data.lamp.get(name).map(|x| x as &dyn RenderableEntity),
            EntityType::LandMine => self
                .data
                .land_mine
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::MiningDrill => self
                .data
                .mining_drill
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::OffshorePump => self
                .data
                .offshore_pump
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Pipe => self.data.pipe.get(name).map(|x| x as &dyn RenderableEntity),
            EntityType::InfinityPipe => self
                .data
                .infinity_pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::PipeToGround => self
                .data
                .pipe_to_ground
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Pump => self.data.pump.get(name).map(|x| x as &dyn RenderableEntity),
            EntityType::SimpleEntityWithOwner => self
                .data
                .simple_entity_with_owner
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::SimpleEntityWithForce => self
                .data
                .simple_entity_with_force
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::SolarPanel => self
                .data
                .solar_panel
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::StorageTank => self
                .data
                .storage_tank
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::LinkedBelt => self
                .data
                .linked_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Loader1x1 => self
                .data
                .loader_1x1
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Loader => self
                .data
                .loader
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Splitter => self
                .data
                .splitter
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::TransportBelt => self
                .data
                .transport_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::UndergroundBelt => self
                .data
                .underground_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Radar => self
                .data
                .radar
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Turret => self
                .data
                .turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::AmmoTurret => self
                .data
                .ammo_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ElectricTurret => self
                .data
                .electric_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::FluidTurret => self
                .data
                .fluid_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Car => self.data.car.get(name).map(|x| x as &dyn RenderableEntity),
            EntityType::CurvedRail => self
                .data
                .curved_rail
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::StraightRail => self
                .data
                .straight_rail
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::RailSignal => self
                .data
                .rail_signal
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::RailChainSignal => self
                .data
                .rail_chain_signal
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::TrainStop => self
                .data
                .train_stop
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::Locomotive => self
                .data
                .locomotive
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::CargoWagon => self
                .data
                .cargo_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::FluidWagon => self
                .data
                .fluid_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            EntityType::ArtilleryWagon => self
                .data
                .artillery_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
        }
    }

    pub fn entities(&self) -> std::collections::HashSet<&String> {
        self.entities.keys().collect()
    }

    pub fn render_entity(
        &self,
        entity_name: &str,
        render_opts: &RenderOpts,
    ) -> Option<GraphicsOutput> {
        self.get_entity(entity_name)?.render(render_opts)
    }
}
