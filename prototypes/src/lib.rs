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

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use entity::RenderableEntity;
use image::{imageops, GenericImageView};
use mod_util::mod_info::Version;

use mod_util::UsedMods;
use types::*;

pub mod entity;
pub mod fluid;
pub mod item;
pub mod recipe;

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

    #[serde(flatten)]
    pub fluid: fluid::AllTypes,

    #[serde(flatten)]
    pub recipe: recipe::AllTypes,
    pub recipe_category: PrototypeMap<recipe::RecipeCategory>,

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
    raw: DataRaw,

    entities: HashMap<String, entity::Type>,
}

impl DataUtil {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn new(raw: DataRaw) -> Self {
        let mut entities: HashMap<String, entity::Type> = HashMap::new();

        {
            (*raw.entity.accumulator).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Accumulator);
            });

            (*raw.entity.artillery_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ArtilleryTurret);
            });

            (*raw.entity.beacon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Beacon);
            });

            (*raw.entity.boiler).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Boiler);
            });

            (*raw.entity.burner_generator).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::BurnerGenerator);
            });

            (*raw.entity.arithmetic_combinator)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ArithmeticCombinator);
                });

            (*raw.entity.decider_combinator)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::DeciderCombinator);
                });

            (*raw.entity.constant_combinator)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ConstantCombinator);
                });

            (*raw.entity.programmable_speaker)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ProgrammableSpeaker);
                });

            (*raw.entity.container).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Container);
            });

            (*raw.entity.logistic_container)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::LogisticContainer);
                });

            (*raw.entity.infinity_container)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::InfinityContainer);
                });

            (*raw.entity.linked_container).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LinkedContainer);
            });

            (*raw.entity.assembling_machine)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::AssemblingMachine);
                });

            (*raw.entity.rocket_silo).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::RocketSilo);
            });

            (*raw.entity.furnace).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Furnace);
            });

            (*raw.entity.electric_energy_interface)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ElectricEnergyInterface);
                });

            (*raw.entity.electric_pole).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ElectricPole);
            });

            (*raw.entity.power_switch).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::PowerSwitch);
            });

            (*raw.entity.combat_robot).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::CombatRobot);
            });

            (*raw.entity.construction_robot)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::ConstructionRobot);
                });

            (*raw.entity.logistic_robot).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LogisticRobot);
            });

            (*raw.entity.roboport).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Roboport);
            });

            (*raw.entity.gate).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Gate);
            });

            (*raw.entity.gate).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Gate);
            });

            (*raw.entity.wall).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Wall);
            });

            (*raw.entity.generator).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Generator);
            });

            (*raw.entity.reactor).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Reactor);
            });

            (*raw.entity.heat_interface).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::HeatInterface);
            });

            (*raw.entity.heat_pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::HeatPipe);
            });

            (*raw.entity.inserter).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Inserter);
            });

            (*raw.entity.lab).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Lab);
            });

            (*raw.entity.lamp).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Lamp);
            });

            (*raw.entity.land_mine).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LandMine);
            });

            (*raw.entity.market).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Market);
            });

            (*raw.entity.mining_drill).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::MiningDrill);
            });

            (*raw.entity.offshore_pump).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::OffshorePump);
            });

            (*raw.entity.pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Pipe);
            });

            (*raw.entity.infinity_pipe).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::InfinityPipe);
            });

            (*raw.entity.pipe_to_ground).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::PipeToGround);
            });

            (*raw.entity.pump).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Pump);
            });

            (*raw.entity.simple_entity_with_owner)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::SimpleEntityWithOwner);
                });

            (*raw.entity.simple_entity_with_force)
                .keys()
                .fold((), |(), name| {
                    entities.insert(name.clone(), entity::Type::SimpleEntityWithForce);
                });

            (*raw.entity.solar_panel).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::SolarPanel);
            });

            (*raw.entity.storage_tank).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::StorageTank);
            });

            (*raw.entity.linked_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::LinkedBelt);
            });

            (*raw.entity.loader_1x1).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Loader1x1);
            });

            (*raw.entity.loader).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Loader);
            });

            (*raw.entity.splitter).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Splitter);
            });

            (*raw.entity.transport_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::TransportBelt);
            });

            (*raw.entity.underground_belt).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::UndergroundBelt);
            });

            (*raw.entity.radar).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Radar);
            });

            (*raw.entity.turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Turret);
            });

            (*raw.entity.ammo_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::AmmoTurret);
            });

            (*raw.entity.electric_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ElectricTurret);
            });

            (*raw.entity.fluid_turret).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::FluidTurret);
            });

            (*raw.entity.car).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Car);
            });

            (*raw.entity.curved_rail).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::CurvedRail);
            });

            (*raw.entity.straight_rail).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::StraightRail);
            });

            (*raw.entity.rail_signal).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::RailSignal);
            });

            (*raw.entity.rail_chain_signal).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::RailChainSignal);
            });

            (*raw.entity.train_stop).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::TrainStop);
            });

            (*raw.entity.locomotive).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::Locomotive);
            });

            (*raw.entity.cargo_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::CargoWagon);
            });

            (*raw.entity.fluid_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::FluidWagon);
            });

            (*raw.entity.artillery_wagon).keys().fold((), |(), name| {
                entities.insert(name.clone(), entity::Type::ArtilleryWagon);
            });
        }

        Self { raw, entities }
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
                .raw
                .entity
                .accumulator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ArtilleryTurret => self
                .raw
                .entity
                .artillery_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Beacon => self
                .raw
                .entity
                .beacon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Boiler => self
                .raw
                .entity
                .boiler
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::BurnerGenerator => self
                .raw
                .entity
                .burner_generator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ArithmeticCombinator => self
                .raw
                .entity
                .arithmetic_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::DeciderCombinator => self
                .raw
                .entity
                .decider_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ConstantCombinator => self
                .raw
                .entity
                .constant_combinator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ProgrammableSpeaker => self
                .raw
                .entity
                .programmable_speaker
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Container => self
                .raw
                .entity
                .container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LogisticContainer => self
                .raw
                .entity
                .logistic_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::InfinityContainer => self
                .raw
                .entity
                .infinity_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LinkedContainer => self
                .raw
                .entity
                .linked_container
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::AssemblingMachine => self
                .raw
                .entity
                .assembling_machine
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::RocketSilo => self
                .raw
                .entity
                .rocket_silo
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Furnace => self
                .raw
                .entity
                .furnace
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ElectricEnergyInterface => self
                .raw
                .entity
                .electric_energy_interface
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ElectricPole => self
                .raw
                .entity
                .electric_pole
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::PowerSwitch => self
                .raw
                .entity
                .power_switch
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::CombatRobot => self
                .raw
                .entity
                .combat_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ConstructionRobot => self
                .raw
                .entity
                .construction_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LogisticRobot => self
                .raw
                .entity
                .logistic_robot
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Roboport => self
                .raw
                .entity
                .roboport
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Gate => self
                .raw
                .entity
                .gate
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Wall => self
                .raw
                .entity
                .wall
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Generator => self
                .raw
                .entity
                .generator
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Reactor => self
                .raw
                .entity
                .reactor
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::HeatInterface => self
                .raw
                .entity
                .heat_interface
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::HeatPipe => self
                .raw
                .entity
                .heat_pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Inserter => self
                .raw
                .entity
                .inserter
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Lab => self
                .raw
                .entity
                .lab
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Lamp => self
                .raw
                .entity
                .lamp
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LandMine => self
                .raw
                .entity
                .land_mine
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Market => self
                .raw
                .entity
                .market
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::MiningDrill => self
                .raw
                .entity
                .mining_drill
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::OffshorePump => self
                .raw
                .entity
                .offshore_pump
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Pipe => self
                .raw
                .entity
                .pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::InfinityPipe => self
                .raw
                .entity
                .infinity_pipe
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::PipeToGround => self
                .raw
                .entity
                .pipe_to_ground
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Pump => self
                .raw
                .entity
                .pump
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::SimpleEntityWithOwner => self
                .raw
                .entity
                .simple_entity_with_owner
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::SimpleEntityWithForce => self
                .raw
                .entity
                .simple_entity_with_force
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::SolarPanel => self
                .raw
                .entity
                .solar_panel
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::StorageTank => self
                .raw
                .entity
                .storage_tank
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::LinkedBelt => self
                .raw
                .entity
                .linked_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Loader1x1 => self
                .raw
                .entity
                .loader_1x1
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Loader => self
                .raw
                .entity
                .loader
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Splitter => self
                .raw
                .entity
                .splitter
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::TransportBelt => self
                .raw
                .entity
                .transport_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::UndergroundBelt => self
                .raw
                .entity
                .underground_belt
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Radar => self
                .raw
                .entity
                .radar
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Turret => self
                .raw
                .entity
                .turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::AmmoTurret => self
                .raw
                .entity
                .ammo_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ElectricTurret => self
                .raw
                .entity
                .electric_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::FluidTurret => self
                .raw
                .entity
                .fluid_turret
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Car => self
                .raw
                .entity
                .car
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::CurvedRail => self
                .raw
                .entity
                .curved_rail
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::StraightRail => self
                .raw
                .entity
                .straight_rail
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::RailSignal => self
                .raw
                .entity
                .rail_signal
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::RailChainSignal => self
                .raw
                .entity
                .rail_chain_signal
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::TrainStop => self
                .raw
                .entity
                .train_stop
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::Locomotive => self
                .raw
                .entity
                .locomotive
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::CargoWagon => self
                .raw
                .entity
                .cargo_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::FluidWagon => self
                .raw
                .entity
                .fluid_wagon
                .get(name)
                .map(|x| x as &dyn RenderableEntity),
            entity::Type::ArtilleryWagon => self
                .raw
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

    pub fn get_recipe_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.raw.recipe.get_icon(
            name,
            scale,
            used_mods,
            image_cache,
            &self.raw.item,
            &self.raw.fluid,
        )
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

    #[test]
    fn deserialize_seablock() {
        let _ = load_data("seablock");
    }
}
