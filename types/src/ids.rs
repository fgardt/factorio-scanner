use paste::paste;
use serde::{Deserialize, Serialize};

macro_rules! ids {
    ( $type:ident, $( $name:ident ),* ) => {
        $(
            paste!{
                #[doc="[`Types/" $name "`](https://lua-api.factorio.com/latest/types/" $name ".html)"]
                #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
                pub struct $name($type);
            }

            impl $name {
                #[allow(dead_code)]
                pub fn new(id: impl Into<$type>) -> Self {
                    Self(id.into())
                }
            }

            impl From<$type> for $name {
                fn from(id: $type) -> Self {
                    Self(id)
                }
            }

            impl std::ops::Deref for $name {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        )+
    };
}

ids!(
    String,
    ActiveTriggerID,
    AirbornePollutantID,
    AmmoCategoryID,
    AsteroidChunkID,
    AutoplaceControlID,
    BurnerUsageID,
    CollisionLayerID,
    DamageTypeID,
    DecorativeID,
    EntityID,
    EquipmentCategoryID,
    EquipmentGridID,
    EquipmentID,
    // FluidBoxLinkedConnectionID, // not done here since its a u32
    FluidID,
    FuelCategoryID,
    ItemGroupID,
    ItemID,
    ItemSubGroupID,
    ModuleCategoryID,
    MouseCursorID,
    ParticleID,
    ProcessionID,
    ProcessionLayerInheritanceGroupID,
    QualityID,
    RecipeCategoryID,
    RecipeID,
    ResourceCategoryID,
    SpaceConnectionID,
    SpaceLocationID,
    SurfaceID,
    SurfacePropertyID,
    TechnologyID,
    TileEffectDefinitionID,
    TileID,
    TrivialSmokeID,
    VirtualSignalID
);

ids!(u32, FluidBoxLinkedConnectionID);

impl QualityID {
    #[must_use]
    pub fn normal() -> Self {
        Self("normal".to_string())
    }

    #[must_use]
    pub fn is_normal(&self) -> bool {
        self.0 == "normal"
    }
}
