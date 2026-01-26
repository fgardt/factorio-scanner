#![allow(
    unused_variables,
    clippy::wildcard_imports,
    clippy::struct_excessive_bools,
    clippy::module_name_repetitions
)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use entity::EntityInfo;
use mod_util::mod_info::Version;

use mod_util::UsedMods;
use tracing::instrument;
use types::*;

pub mod entity;
#[cfg(feature = "graphics")]
mod entity_graphics;
pub mod fluid;
pub mod item;
pub mod quality;
pub mod recipe;
pub mod signal;
pub mod space_location;
pub mod tile;
pub mod utility_sprites;

#[cfg(feature = "graphics")]
mod rendering;
#[cfg(feature = "graphics")]
pub use rendering::*;

// `Prototype` not implemented since it only holds the `factoriopedia_alternative` field

/// [`Prototypes/PrototypeBase`](https://lua-api.factorio.com/latest/PrototypeBase.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BasePrototype<T> {
    /// type can effectively be ignored, as it should be enforced by the struct/enum types itself
    #[serde(rename = "type")]
    pub type_: String,

    pub name: String,

    #[serde(default, skip_serializing_if = "serde_helper::is_default")]
    pub order: Order,

    pub localised_name: Option<LocalisedString>,
    pub localised_description: Option<LocalisedString>,
    pub factoriopedia_description: Option<LocalisedString>,

    #[serde(default, skip_serializing_if = "serde_helper::is_default")]
    pub hidden: bool,
    pub hidden_in_factoriopedia: Option<bool>,

    #[serde(default, skip_serializing_if = "serde_helper::is_default")]
    pub parameter: bool,

    // pub factoriopedia_simulation: Option<SimulationDefinition>,
    #[serde(flatten)]
    child: T,
}

impl<T> std::ops::Deref for BasePrototype<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

pub trait IdNamespace {
    type Id;
    type Type;

    #[must_use]
    fn all_ids(&self) -> HashSet<&Self::Id>;

    #[must_use]
    fn contains(&self, id: &Self::Id) -> bool;

    #[must_use]
    fn build_mapping(&self) -> HashMap<Self::Id, Self::Type>;
}

pub trait IdNamespaceAccess<T>: IdNamespace {
    #[must_use]
    fn get_proto(&self, id: &Self::Id) -> Option<&T>;
}

mod helper_macro {
    macro_rules! namespace_struct {
        ( $name:ident, $id:ty, $( $member:literal ),+ ) => {
            paste::paste! {
                #[derive(Debug, Default, Deserialize, Serialize)]
                #[serde(rename_all = "kebab-case")]
                pub struct $name {
                    $(
                        #[serde(default)]
                        pub [< $member:snake >]: std::collections::HashMap<$id, [< $member:camel Prototype >]>,
                    )+
                }

                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum Type {
                    $(
                        [< $member:camel >],
                    )+
                }

                impl crate::IdNamespace for $name {
                    type Id = $id;
                    type Type = Type;

                    fn all_ids(&self) -> std::collections::HashSet<&Self::Id> {
                        let mut res = std::collections::HashSet::new();
                        $(
                            res.extend(self.[< $member:snake >].keys());
                        )+
                        res
                    }

                    fn contains(&self, id: &Self::Id) -> bool {
                        $(
                            if self.[< $member:snake >].contains_key(id) {
                                return true;
                            }
                        )+
                        false
                    }

                    fn build_mapping(&self) -> std::collections::HashMap<Self::Id, Self::Type> {
                        let mut res = std::collections::HashMap::new();

                        $(
                            self.[< $member:snake >].keys().for_each(|id| {
                                res.insert(id.clone(), Type::[< $member:camel >]);
                            });
                        )+

                        res
                    }
                }

                $(
                    impl crate::IdNamespaceAccess<[< $member:camel Prototype >]> for $name {
                        fn get_proto(&self, id: &Self::Id) -> Option<&[< $member:camel Prototype >]> {
                            self.[< $member:snake >].get(id)
                        }
                    }
                )+
            }
        };
        ( $name:ident, $id:ty, $map:ty, $( $member:literal ),+ ) => {
            namespace_struct!($name, $id, $( $member ),+);
            paste::paste! {
                impl $name {
                    #[must_use]
                    pub fn get(&self, id: &$id, map: &std::collections::HashMap<$id, Type>) -> Option<$map> {
                        match map.get(id) {
                            $(
                                Some(Type::[< $member:camel >]) => self.[< $member:snake >].get(id).map(|x| x as $map),
                            )+
                            None => None,
                        }
                    }
                }
            }
        }
    }

    pub(crate) use namespace_struct;
}

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
    pub virtual_signal: signal::AllTypes,

    #[serde(flatten)]
    pub recipe: recipe::AllTypes,
    pub recipe_category: HashMap<RecipeCategoryID, recipe::RecipeCategory>,

    #[serde(flatten)]
    pub tile: tile::AllTypes,

    #[serde(flatten)]
    pub quality: quality::AllTypes,

    #[serde(flatten)]
    pub space_location: space_location::AllTypes,

    pub utility_sprites: HashMap<String, utility_sprites::UtilitySprites>,
}

impl DataRaw {
    pub fn load(dump_path: &Path) -> Result<Self, Error> {
        let mut bytes = Vec::new();
        File::open(dump_path)?.read_to_end(&mut bytes)?;
        Self::load_from_bytes(&bytes)
    }

    #[instrument(skip_all)]
    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

pub struct DataUtil {
    raw: DataRaw,

    entities: HashMap<EntityID, entity::Type>,
}

impl DataUtil {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn new(raw: DataRaw) -> Self {
        let entities = raw.entity.build_mapping();

        Self { raw, entities }
    }

    #[must_use]
    pub const fn entities(&self) -> &HashMap<EntityID, entity::Type> {
        &self.entities
    }

    #[must_use]
    pub fn get_entity_type(&self, name: &str) -> Option<&entity::Type> {
        self.entities.get(&EntityID::new(name))
    }

    #[must_use]
    pub fn contains_entity(&self, name: &str) -> bool {
        self.entities.contains_key(&EntityID::new(name))
    }

    #[must_use]
    pub fn contains_recipe(&self, name: &str) -> bool {
        self.raw.recipe.recipe.contains_key(&RecipeID::new(name))
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn get_entity(&self, name: &str) -> Option<&dyn EntityInfo> {
        self.raw.entity.get(&EntityID::new(name), &self.entities)
    }

    #[must_use]
    pub fn recipe_has_fluid(&self, name: &RecipeID) -> (bool, bool) {
        self.raw.recipe.uses_fluid(name)
    }

    #[must_use]
    pub fn util_sprites(&self) -> Option<&utility_sprites::UtilitySprites> {
        let key = self.raw.utility_sprites.keys().next()?;
        self.raw.utility_sprites.get(key)
    }
}

#[cfg(feature = "graphics")]
impl DataUtil {
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

    pub fn get_item_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.raw.item.get_icon(name, scale, used_mods, image_cache)
    }

    pub fn get_fluid_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.raw.fluid.get_icon(name, scale, used_mods, image_cache)
    }

    pub fn get_signal_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.raw
            .virtual_signal
            .virtual_signal
            .get(&VirtualSignalID::new(name))
            .and_then(|x| x.get_icon(scale, used_mods, image_cache))
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

pub trait DataUtilAccess<I, S>
where
    S: IdNamespace,
{
    fn get_proto<T>(&self, id: &I) -> Option<&T>
    where
        S: IdNamespaceAccess<T>;
}

impl DataUtilAccess<EntityID, entity::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &EntityID) -> Option<&T>
    where
        entity::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.entity.get_proto(id)
    }
}

impl DataUtilAccess<ItemID, item::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &ItemID) -> Option<&T>
    where
        item::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.item.get_proto(id)
    }
}

impl DataUtilAccess<FluidID, fluid::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &FluidID) -> Option<&T>
    where
        fluid::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.fluid.get_proto(id)
    }
}

impl DataUtilAccess<VirtualSignalID, signal::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &VirtualSignalID) -> Option<&T>
    where
        signal::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.virtual_signal.get_proto(id)
    }
}

impl DataUtilAccess<RecipeID, recipe::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &RecipeID) -> Option<&T>
    where
        recipe::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.recipe.get_proto(id)
    }
}

impl DataUtilAccess<TileID, tile::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &TileID) -> Option<&T>
    where
        tile::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.tile.get_proto(id)
    }
}

impl DataUtilAccess<QualityID, quality::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &QualityID) -> Option<&T>
    where
        quality::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.quality.get_proto(id)
    }
}

impl DataUtilAccess<SpaceLocationID, space_location::AllTypes> for DataUtil {
    fn get_proto<T>(&self, id: &SpaceLocationID) -> Option<&T>
    where
        space_location::AllTypes: IdNamespaceAccess<T>,
    {
        self.raw.space_location.get_proto(id)
    }
}

use konst::{iter::collect_const, result::unwrap, string::split as konst_split};

#[must_use]
pub const fn targeted_engine_version() -> Version {
    const V: [&str; 3] = collect_const!(&str => konst_split(env!("CARGO_PKG_VERSION_PRE"), '.'));
    Version::new(
        unwrap!(u16::from_str_radix(V[0], 10)),
        unwrap!(u16::from_str_radix(V[1], 10)),
        unwrap!(u16::from_str_radix(V[2], 10)),
    )
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
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

    mod deserialize {
        use super::load_data;

        macro_rules! deserialize_tests {
            ($($name:ident),+) => {
                $(
                    #[test]
                    fn $name() {
                        let _ = load_data(stringify!($name));
                    }
                )+
            };
        }

        deserialize_tests!(base, space_age, py, pm);
    }
}
