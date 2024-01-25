#![allow(
    unused_variables,
    clippy::wildcard_imports,
    clippy::struct_excessive_bools,
    clippy::module_name_repetitions
)]

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{collections::HashMap, ops::Rem};

use imageproc::geometric_transformations;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use entity::RenderableEntity;
use image::{imageops, DynamicImage, GenericImageView};
use mod_util::mod_info::Version;

use mod_util::UsedMods;
use types::*;

pub mod entity;
pub mod fluid;
pub mod item;
pub mod recipe;
pub mod utility_sprites;

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

impl<T> std::ops::Deref for BasePrototype<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

pub type PrototypeMap<T> = HashMap<String, T>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("data.raw io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("data.raw JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

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

    pub utility_sprites: PrototypeMap<utility_sprites::UtilitySprites>,
}

impl DataRaw {
    pub fn load(dump_path: &Path) -> Result<Self, Error> {
        let mut bytes = Vec::new();
        File::open(dump_path)?.read_to_end(&mut bytes)?;
        Self::load_from_bytes(&bytes)
    }

    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(bytes)?)
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

    #[must_use]
    pub fn contains_entity(&self, name: &str) -> bool {
        self.entities.contains_key(name)
    }

    #[must_use]
    pub fn contains_recipe(&self, name: &str) -> bool {
        self.raw.recipe.recipe.contains_key(name)
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

    #[must_use]
    pub fn util_sprites(&self) -> Option<&utility_sprites::UtilitySprites> {
        let key = self.raw.utility_sprites.keys().next()?;
        self.raw.utility_sprites.get(key)
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

impl std::fmt::Display for TargetSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{} @ {} ({} - {} @ {} px/tile)",
            self.width, self.height, self.scale, self.top_left, self.bottom_right, self.tile_res
        )
    }
}

#[derive(Debug, Clone)]
pub struct RenderLayerBuffer {
    target_size: TargetSize,
    layers: HashMap<InternalRenderLayer, image::DynamicImage>,

    wire_connection_points: HashMap<u64, GenericWireConnectionPoint>,
}

pub type ConnectedEntities = HashMap<u64, [bool; 3]>;
pub type EntityWireConnections = HashMap<u64, (MapPosition, ([ConnectedEntities; 3], bool))>;

impl RenderLayerBuffer {
    #[must_use]
    pub fn new(target_size: TargetSize) -> Self {
        Self {
            target_size,
            layers: HashMap::new(),
            wire_connection_points: HashMap::new(),
        }
    }

    fn get_layer(&mut self, layer: InternalRenderLayer) -> &mut image::DynamicImage {
        self.layers.entry(layer).or_insert_with(|| {
            image::DynamicImage::new_rgba8(self.target_size.width, self.target_size.height)
        })
    }

    pub fn add(
        &mut self,
        (img, shift): (image::DynamicImage, Vector),
        position: &MapPosition,
        layer: InternalRenderLayer,
    ) {
        let (x, y) = self
            .target_size
            .get_pixel_pos(img.dimensions(), &shift, position);

        let layer = self.get_layer(layer);
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

    fn store_wire_connection_points(
        &mut self,
        bp_entity_id: u64,
        wire_connection_points: GenericWireConnectionPoint,
    ) {
        self.wire_connection_points
            .insert(bp_entity_id, wire_connection_points);
    }

    fn generate_wire_draw_data<'a>(
        &mut self,
        wire_data: &'a EntityWireConnections,
    ) -> [Vec<[(&'a MapPosition, Vector); 2]>; 3] {
        let mut already_drawn = HashSet::<((u64, usize), (u64, usize), usize)>::new();
        let mut draw_data: [Vec<[(&MapPosition, Vector); 2]>; 3] = Default::default();

        for (source, (s_pos, (s_wcps_cons, s_is_switch))) in wire_data {
            let Some(s_wcps) = self.wire_connection_points.get(source) else {
                continue;
            };

            s_wcps_cons
                .iter()
                .enumerate()
                .for_each(|(s_cons_id, s_cons)| {
                    let Some(s_wcp) = &s_wcps[s_cons_id] else {
                        return;
                    };

                    for (target, s_con) in s_cons {
                        // if already_drawn.contains(&(*source, *target)) {
                        //     // println!("skipping {source}: {s_cons_id} -> {target}");
                        //     return;
                        // }

                        let Some(t_wcps) = self.wire_connection_points.get(target) else {
                            return;
                        };

                        let Some((t_pos, (t_wcps_cons, _))) = wire_data.get(target) else {
                            return;
                        };

                        t_wcps_cons
                            .iter()
                            .enumerate()
                            .for_each(|(t_cons_id, t_cons)| {
                                let Some(t_con) = t_cons.get(source) else {
                                    return;
                                };

                                let Some(t_wcp) = &t_wcps[t_cons_id] else {
                                    return;
                                };

                                s_con
                                    .iter()
                                    .enumerate()
                                    .zip(t_con.iter())
                                    .filter(|((w, &s), &t)| s && t || *s_is_switch && s && *w == 0)
                                    .for_each(|((wire_id, _), _)| {
                                        let (s_offset, t_offset) = match wire_id {
                                            0 => (s_wcp.wire.copper, t_wcp.wire.copper),
                                            1 => (s_wcp.wire.red, t_wcp.wire.red),
                                            2 => (s_wcp.wire.green, t_wcp.wire.green),
                                            _ => unreachable!(),
                                        };

                                        let Some(s_offset) = s_offset else {
                                            return;
                                        };

                                        let Some(t_offset) = t_offset else {
                                            return;
                                        };

                                        // println!("drawing {source}: {s_cons_id} -> {target}: {t_cons_id} @ {wire_id}");

                                        // skip if already drawn
                                        if already_drawn.contains(&(
                                            (*source, s_cons_id),
                                            (*target, t_cons_id),
                                            wire_id,
                                        )) {
                                            return;
                                        }

                                        let dd = &mut draw_data[wire_id];
                                        dd.push([(s_pos, s_offset), (t_pos, t_offset)]);

                                        already_drawn.insert((
                                            (*source, s_cons_id),
                                            (*target, t_cons_id),
                                            wire_id,
                                        ));

                                        already_drawn.insert((
                                            (*target, t_cons_id),
                                            (*source, s_cons_id),
                                            wire_id,
                                        ));
                                    });
                            });
                    }
                });
        }

        draw_data
    }

    pub fn draw_wires(
        &mut self,
        wire_data: &EntityWireConnections,
        util_sprites: &utility_sprites::UtilitySprites,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) {
        let dd = self.generate_wire_draw_data(wire_data);

        let target_size = self.target_size.clone();
        let layer = self.get_layer(InternalRenderLayer::Wire);

        for i in 0..3u8 {
            let d = &dd[usize::from(i)];

            if d.is_empty() {
                continue;
            }

            let Some((base_wire, _)) = match i {
                0 => &util_sprites.wires.copper_wire,
                1 => &util_sprites.wires.red_wire,
                2 => &util_sprites.wires.green_wire,
                _ => unreachable!(),
            }
            .render(
                self.scale(),
                used_mods,
                image_cache,
                &SimpleGraphicsRenderOpts::default(),
            ) else {
                continue;
            };

            let (base_wire_width, base_wire_height) = base_wire.dimensions();
            let base_length = (f64::from(base_wire_width) / 32.0) * self.scale();

            for [(s_pos, s_offset), (t_pos, t_offset)] in d {
                let start = *s_pos + &MapPosition::from(*s_offset);
                let end = *t_pos + &MapPosition::from(*t_offset);
                let length = start.distance_to(&end);

                let mut orientation = start.rad_orientation_to(&end);
                if orientation > std::f64::consts::FRAC_PI_2 {
                    orientation -= std::f64::consts::PI;
                } else if orientation < -std::f64::consts::FRAC_PI_2 {
                    orientation += std::f64::consts::PI;
                }

                let offset = 3;
                let horiz_crop_fac = orientation.cos() * (length / 3.0).min(1.0);
                let cropped_width =
                    f64::from(base_wire_width - offset) * horiz_crop_fac + f64::from(offset);

                // magic numbers :)
                let vert_crop_fac = 5.6f64.mul_add(
                    (horiz_crop_fac / 2.0).powi(4),
                    2.6 * (horiz_crop_fac / 2.0).powi(2),
                );
                let cropped_height =
                    f64::from(base_wire_height - offset) * vert_crop_fac + f64::from(offset);

                let base_wire = base_wire.crop_imm(
                    ((f64::from(base_wire_width) - cropped_width) / 2.0).floor() as u32,
                    (f64::from(base_wire_height) - cropped_height).floor() as u32,
                    cropped_width.ceil() as u32,
                    cropped_height.ceil() as u32,
                );

                let wire = base_wire.resize_exact(
                    (f64::from(base_wire_width) * (length / base_length)).ceil() as u32,
                    cropped_height.ceil() as u32,
                    image::imageops::FilterType::CatmullRom,
                );

                let (w, h) = wire.dimensions();
                let mut wire_square = DynamicImage::new_rgba8(w, w);
                image::imageops::overlay(&mut wire_square, &wire, 0, i64::from((w - h) / 2));

                let rotated = geometric_transformations::rotate_about_center(
                    &wire_square.to_rgba8(),
                    orientation as f32,
                    geometric_transformations::Interpolation::Bicubic,
                    image::Rgba([0, 0, 0, 0]),
                );

                self.add(
                    (rotated.into(), Vector::default()),
                    &start.center_to(&end),
                    InternalRenderLayer::Wire,
                );
            }
        }
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
