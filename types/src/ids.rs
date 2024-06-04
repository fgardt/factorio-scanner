use paste::paste;
use serde::{Deserialize, Serialize};

macro_rules! ids {
    ( $( $name:ident ),* ) => {
        $(
            paste!{
                #[doc="[`Types/" $name "`](https://lua-api.factorio.com/latest/types/" $name ".html)"]
                #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
                pub struct $name(String);
            }

            impl $name {
                pub fn new(id: impl Into<String>) -> Self {
                    Self(id.into())
                }
            }

            impl std::ops::Deref for $name {
                type Target = String;

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
    AmmoCategoryID,
    DamageTypeID,
    EntityID,
    EquipmentGridID,
    EquipmentID,
    FluidID,
    FuelCategoryID,
    ItemGroupID,
    ItemID,
    ItemSubGroupID,
    MouseCursorID,
    RecipeCategoryID,
    RecipeID,
    ResourceCategoryID,
    TileID,
    VirtualSignalID
);
