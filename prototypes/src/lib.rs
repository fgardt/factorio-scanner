#![allow(
    unused_variables,
    clippy::wildcard_imports,
    clippy::struct_excessive_bools,
    clippy::module_name_repetitions
)]

use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;
use std::{collections::HashMap, ops::Rem};

use entity::RenderableEntity;
use image::{imageops, GenericImageView};
use mod_util::mod_info::Version;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use mod_util::UsedMods;
use types::*;

pub mod entity;
pub mod item;

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

impl<T> Deref for BasePrototype<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

/// [`Prototypes/UtilitySprites/CursorBoxSpecification`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#cursor_box)
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorBoxSpecification {
    pub regular: FactorioArray<BoxSpecification>,
    pub not_allowed: FactorioArray<BoxSpecification>,
    pub copy: FactorioArray<BoxSpecification>,
    pub electricity: FactorioArray<BoxSpecification>,
    pub logistics: FactorioArray<BoxSpecification>,
    pub pair: FactorioArray<BoxSpecification>,
    pub train_visualization: FactorioArray<BoxSpecification>,
    pub blueprint_snap_rectangle: FactorioArray<BoxSpecification>,
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
    #[serde(flatten)]
    pub entity: entity::AllTypes,

    #[serde(flatten)]
    pub item: item::AllTypes,

    pub utility_sprites: PrototypeMap<UtilitySprites>,
}

impl DataRaw {
    #[must_use]
    pub fn load(dump_path: &Path) -> Option<Self> {
        let mut bytes = Vec::new();
        File::open(dump_path).ok()?.read_to_end(&mut bytes).ok()?;
        Some(serde_json::from_slice(&bytes).unwrap())
    }
}

pub struct DataUtil {
    data: DataRaw,

    entities: HashMap<String, entity::Type>,
}

impl DataUtil {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn new(data: DataRaw) -> Self {
        let mut entities: HashMap<String, entity::Type> = HashMap::new();

        {
            (*data.entity.accumulator).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Accumulator);
            });

            (*data.entity.artillery_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ArtilleryTurret);
            });

            (*data.entity.beacon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Beacon);
            });

            (*data.entity.boiler).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Boiler);
            });

            (*data.entity.burner_generator).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::BurnerGenerator);
            });

            (*data.entity.arithmetic_combinator)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ArithmeticCombinator);
                });

            (*data.entity.decider_combinator)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::DeciderCombinator);
                });

            (*data.entity.constant_combinator)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ConstantCombinator);
                });

            (*data.entity.programmable_speaker)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ProgrammableSpeaker);
                });

            (*data.entity.container).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Container);
            });

            (*data.entity.logistic_container)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::LogisticContainer);
                });

            (*data.entity.infinity_container)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::InfinityContainer);
                });

            (*data.entity.linked_container).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LinkedContainer);
            });

            (*data.entity.assembling_machine)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::AssemblingMachine);
                });

            (*data.entity.rocket_silo).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::RocketSilo);
            });

            (*data.entity.furnace).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Furnace);
            });

            (*data.entity.electric_energy_interface)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ElectricEnergyInterface);
                });

            (*data.entity.electric_pole).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ElectricPole);
            });

            (*data.entity.power_switch).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::PowerSwitch);
            });

            (*data.entity.combat_robot).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::CombatRobot);
            });

            (*data.entity.construction_robot)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ConstructionRobot);
                });

            (*data.entity.logistic_robot).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LogisticRobot);
            });

            (*data.entity.roboport).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Roboport);
            });

            (*data.entity.gate).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Gate);
            });

            (*data.entity.gate).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Gate);
            });

            (*data.entity.wall).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Wall);
            });

            (*data.entity.generator).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Generator);
            });

            (*data.entity.reactor).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Reactor);
            });

            (*data.entity.heat_interface).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::HeatInterface);
            });

            (*data.entity.heat_pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::HeatPipe);
            });

            (*data.entity.inserter).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Inserter);
            });

            (*data.entity.lab).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Lab);
            });

            (*data.entity.lamp).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Lamp);
            });

            (*data.entity.land_mine).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LandMine);
            });

            (*data.entity.market).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Market);
            });

            (*data.entity.mining_drill).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::MiningDrill);
            });

            (*data.entity.offshore_pump).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::OffshorePump);
            });

            (*data.entity.pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Pipe);
            });

            (*data.entity.infinity_pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::InfinityPipe);
            });

            (*data.entity.pipe_to_ground).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::PipeToGround);
            });

            (*data.entity.pump).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Pump);
            });

            (*data.entity.simple_entity_with_owner)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::SimpleEntityWithOwner);
                });

            (*data.entity.simple_entity_with_force)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::SimpleEntityWithForce);
                });

            (*data.entity.solar_panel).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::SolarPanel);
            });

            (*data.entity.storage_tank).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::StorageTank);
            });

            (*data.entity.linked_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LinkedBelt);
            });

            (*data.entity.loader_1x1).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Loader1x1);
            });

            (*data.entity.loader).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Loader);
            });

            (*data.entity.splitter).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Splitter);
            });

            (*data.entity.transport_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::TransportBelt);
            });

            (*data.entity.underground_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::UndergroundBelt);
            });

            (*data.entity.radar).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Radar);
            });

            (*data.entity.turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Turret);
            });

            (*data.entity.ammo_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::AmmoTurret);
            });

            (*data.entity.electric_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ElectricTurret);
            });

            (*data.entity.fluid_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::FluidTurret);
            });

            (*data.entity.car).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Car);
            });

            (*data.entity.curved_rail).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::CurvedRail);
            });

            (*data.entity.straight_rail).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::StraightRail);
            });

            (*data.entity.rail_signal).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::RailSignal);
            });

            (*data.entity.rail_chain_signal)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::RailChainSignal);
                });

            (*data.entity.train_stop).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::TrainStop);
            });

            (*data.entity.locomotive).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Locomotive);
            });

            (*data.entity.cargo_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::CargoWagon);
            });

            (*data.entity.fluid_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::FluidWagon);
            });

            (*data.entity.artillery_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ArtilleryWagon);
            });
        }

        Self { data, entities }
    }

    #[must_use]
    pub fn get_type(&self, name: &str) -> Option<&entity::Type> {
        self.entities.get(name)
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn get_entity(&self, name: &str) -> Option<&dyn RenderableEntity> {
        let entity_type = self.get_type(name)?;

        match entity_type {
            entity::Type::Accumulator => self
                .data
                .entity
                .accumulator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ArtilleryTurret => self
                .data
                .entity
                .artillery_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Beacon => self
                .data
                .entity
                .beacon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Boiler => self
                .data
                .entity
                .boiler
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::BurnerGenerator => self
                .data
                .entity
                .burner_generator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ArithmeticCombinator => self
                .data
                .entity
                .arithmetic_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::DeciderCombinator => self
                .data
                .entity
                .decider_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ConstantCombinator => self
                .data
                .entity
                .constant_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ProgrammableSpeaker => self
                .data
                .entity
                .programmable_speaker
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Container => self
                .data
                .entity
                .container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LogisticContainer => self
                .data
                .entity
                .logistic_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::InfinityContainer => self
                .data
                .entity
                .infinity_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LinkedContainer => self
                .data
                .entity
                .linked_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::AssemblingMachine => self
                .data
                .entity
                .assembling_machine
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::RocketSilo => self
                .data
                .entity
                .rocket_silo
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Furnace => self
                .data
                .entity
                .furnace
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ElectricEnergyInterface => self
                .data
                .entity
                .electric_energy_interface
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ElectricPole => self
                .data
                .entity
                .electric_pole
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::PowerSwitch => self
                .data
                .entity
                .power_switch
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::CombatRobot => self
                .data
                .entity
                .combat_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ConstructionRobot => self
                .data
                .entity
                .construction_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LogisticRobot => self
                .data
                .entity
                .logistic_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Roboport => self
                .data
                .entity
                .roboport
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Gate => self
                .data
                .entity
                .gate
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Wall => self
                .data
                .entity
                .wall
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Generator => self
                .data
                .entity
                .generator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Reactor => self
                .data
                .entity
                .reactor
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::HeatInterface => self
                .data
                .entity
                .heat_interface
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::HeatPipe => self
                .data
                .entity
                .heat_pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Inserter => self
                .data
                .entity
                .inserter
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Lab => self
                .data
                .entity
                .lab
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Lamp => self
                .data
                .entity
                .lamp
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LandMine => self
                .data
                .entity
                .land_mine
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Market => self
                .data
                .entity
                .market
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::MiningDrill => self
                .data
                .entity
                .mining_drill
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::OffshorePump => self
                .data
                .entity
                .offshore_pump
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Pipe => self
                .data
                .entity
                .pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::InfinityPipe => self
                .data
                .entity
                .infinity_pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::PipeToGround => self
                .data
                .entity
                .pipe_to_ground
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Pump => self
                .data
                .entity
                .pump
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::SimpleEntityWithOwner => self
                .data
                .entity
                .simple_entity_with_owner
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::SimpleEntityWithForce => self
                .data
                .entity
                .simple_entity_with_force
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::SolarPanel => self
                .data
                .entity
                .solar_panel
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::StorageTank => self
                .data
                .entity
                .storage_tank
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LinkedBelt => self
                .data
                .entity
                .linked_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Loader1x1 => self
                .data
                .entity
                .loader_1x1
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Loader => self
                .data
                .entity
                .loader
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Splitter => self
                .data
                .entity
                .splitter
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::TransportBelt => self
                .data
                .entity
                .transport_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::UndergroundBelt => self
                .data
                .entity
                .underground_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Radar => self
                .data
                .entity
                .radar
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Turret => self
                .data
                .entity
                .turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::AmmoTurret => self
                .data
                .entity
                .ammo_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ElectricTurret => self
                .data
                .entity
                .electric_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::FluidTurret => self
                .data
                .entity
                .fluid_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Car => self
                .data
                .entity
                .car
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::CurvedRail => self
                .data
                .entity
                .curved_rail
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::StraightRail => self
                .data
                .entity
                .straight_rail
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::RailSignal => self
                .data
                .entity
                .rail_signal
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::RailChainSignal => self
                .data
                .entity
                .rail_chain_signal
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::TrainStop => self
                .data
                .entity
                .train_stop
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Locomotive => self
                .data
                .entity
                .locomotive
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::CargoWagon => self
                .data
                .entity
                .cargo_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::FluidWagon => self
                .data
                .entity
                .fluid_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ArtilleryWagon => self
                .data
                .entity
                .artillery_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
        }
    }

    #[must_use]
    pub fn entities(&self) -> std::collections::HashSet<&String> {
        self.entities.keys().collect()
    }

    #[must_use]
    pub fn render_entity(
        &self,
        entity_name: &str,
        render_opts: &entity::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> entity::RenderOutput {
        self.get_entity(entity_name)?
            .render(render_opts, used_mods, render_layers, image_cache)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InternalRenderLayer {
    Background,

    Ground,
    GroundPatch,

    RailStonePathBackground,
    RailStonePath,
    RailTies,
    RailBackplate,
    RailMetal,

    Shadow,
    Entity,
    EntityHigh,
    EntityHigher,
    InserterHand,
    AboveEntity,

    Wire,

    DirectionOverlay,
    RecipeOverlay,
}

impl InternalRenderLayer {
    #[must_use]
    pub const fn all() -> [Self; 17] {
        [
            Self::Background,
            Self::Ground,
            Self::GroundPatch,
            Self::RailStonePathBackground,
            Self::RailStonePath,
            Self::RailTies,
            Self::RailBackplate,
            Self::RailMetal,
            Self::Shadow,
            Self::Entity,
            Self::EntityHigh,
            Self::EntityHigher,
            Self::InserterHand,
            Self::AboveEntity,
            Self::Wire,
            Self::DirectionOverlay,
            Self::RecipeOverlay,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TargetSize {
    width: u32,
    height: u32,
    scale: f64,
    top_left: crate::MapPosition,
    bottom_right: crate::MapPosition,

    tile_res: f64,
}

impl TargetSize {
    #[must_use]
    pub fn new(
        width: u32,
        height: u32,
        scale: f64,
        top_left: crate::MapPosition,
        bottom_right: crate::MapPosition,
    ) -> Self {
        const TILE_RES: f64 = 32.0;
        let tile_res = TILE_RES / scale;

        Self {
            width,
            height,
            scale,
            top_left,
            bottom_right,
            tile_res,
        }
    }

    #[must_use]
    fn get_pixel_pos(
        &self,
        (width, height): (u32, u32),
        shift: &Vector,
        position: &MapPosition,
    ) -> (i64, i64) {
        let (x, y) = position.as_tuple();
        let (shift_x, shift_y) = shift.as_tuple();
        let (tl_x, tl_y) = self.top_left.as_tuple();

        let px = f64::from(width).mul_add(-0.5, (x + shift_x - tl_x) * self.tile_res);
        let py = f64::from(height).mul_add(-0.5, (y + shift_y - tl_y) * self.tile_res);

        (px.round() as i64, py.round() as i64)
    }
}

#[derive(Debug, Clone)]
pub struct RenderLayerBuffer {
    target_size: TargetSize,
    layers: HashMap<InternalRenderLayer, image::DynamicImage>,
}

impl RenderLayerBuffer {
    #[must_use]
    pub fn new(target_size: TargetSize) -> Self {
        Self {
            target_size,
            layers: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        (img, shift): (image::DynamicImage, Vector),
        position: &MapPosition,
        layer: InternalRenderLayer,
    ) {
        let layer = self.layers.entry(layer).or_insert_with(|| {
            image::DynamicImage::new_rgba8(self.target_size.width, self.target_size.height)
        });

        let (x, y) = self
            .target_size
            .get_pixel_pos(img.dimensions(), &shift, position);

        imageops::overlay(layer, &img, x, y);
    }

    pub fn add_entity(&mut self, input: (image::DynamicImage, Vector), position: &MapPosition) {
        self.add(input, position, InternalRenderLayer::Entity);
    }

    pub fn add_shadow(&mut self, input: (image::DynamicImage, Vector), position: &MapPosition) {
        self.add(input, position, InternalRenderLayer::Shadow);
    }

    #[must_use]
    pub const fn scale(&self) -> f64 {
        self.target_size.scale
    }

    pub fn generate_background(&mut self) {
        let back_pxl = image::Luma([0x22u8]);
        let line_pxl = image::Luma([0x33u8]);

        let (tl_x, tl_y) = self.target_size.top_left.as_tuple();
        let tile_res = self.target_size.tile_res;
        let tile_p = 0.05;

        let background =
            image::ImageBuffer::from_fn(self.target_size.width, self.target_size.height, |x, y| {
                let x = f64::from(x);
                let y = f64::from(y);

                let t_x = (x / tile_res) - tl_x;
                let t_y = (y / tile_res) - tl_y;

                let p_x = t_x.rem(1.0);
                let p_y = t_y.rem(1.0);

                if p_x < tile_p || p_x > (1.0 - tile_p) || p_y < tile_p || p_y > (1.0 - tile_p) {
                    line_pxl
                } else {
                    back_pxl
                }
            });

        self.layers
            .insert(InternalRenderLayer::Background, background.into());
    }

    #[must_use]
    pub fn combine(self) -> image::DynamicImage {
        let mut combined =
            image::DynamicImage::new_rgba8(self.target_size.width, self.target_size.height);

        for layer in InternalRenderLayer::all() {
            if let Some(img) = self.layers.get(&layer) {
                imageops::overlay(&mut combined, img, 0, 0);
            }
        }

        combined
    }
}

use konst::{primitive::parse_u16, result::unwrap_ctx};

#[must_use]
pub const fn targeted_engine_version() -> Version {
    Version::new(
        unwrap_ctx!(parse_u16(env!("CARGO_PKG_VERSION_MAJOR"))),
        unwrap_ctx!(parse_u16(env!("CARGO_PKG_VERSION_MINOR"))),
        unwrap_ctx!(parse_u16(env!("CARGO_PKG_VERSION_PATCH"))),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[must_use]
    fn load_data(name: &str) -> DataRaw {
        let mut bytes = Vec::new();
        File::open(format!(
            "test_dumps/{name}.{}.json",
            targeted_engine_version()
        ))
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[test]
    fn deserialize_vanilla() {
        let _ = load_data("vanilla");
    }

    #[test]
    fn deserialize_k2_se() {
        let _ = load_data("k2-se");
    }
}
