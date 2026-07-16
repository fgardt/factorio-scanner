use serde::{Deserialize, Serialize};
use serde_helper as helper;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct FeatureFlags {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub expansion: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub quality: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub rail_bridges: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub space_travel: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub spoiling: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub freezing: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub segmented_units: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub expansion_shaders: bool,
}

impl std::ops::BitOr for FeatureFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            expansion: self.expansion || rhs.expansion,
            quality: self.quality || rhs.quality,
            rail_bridges: self.rail_bridges || rhs.rail_bridges,
            space_travel: self.space_travel || rhs.space_travel,
            spoiling: self.spoiling || rhs.spoiling,
            freezing: self.freezing || rhs.freezing,
            segmented_units: self.segmented_units || rhs.segmented_units,
            expansion_shaders: self.expansion_shaders || rhs.expansion_shaders,
        }
    }
}

impl std::ops::BitOrAssign for FeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.expansion |= rhs.expansion;
        self.quality |= rhs.quality;
        self.rail_bridges |= rhs.rail_bridges;
        self.space_travel |= rhs.space_travel;
        self.spoiling |= rhs.spoiling;
        self.freezing |= rhs.freezing;
        self.segmented_units |= rhs.segmented_units;
        self.expansion_shaders |= rhs.expansion_shaders;
    }
}

impl std::ops::BitAnd for FeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            expansion: self.expansion && rhs.expansion,
            quality: self.quality && rhs.quality,
            rail_bridges: self.rail_bridges && rhs.rail_bridges,
            space_travel: self.space_travel && rhs.space_travel,
            spoiling: self.spoiling && rhs.spoiling,
            freezing: self.freezing && rhs.freezing,
            segmented_units: self.segmented_units && rhs.segmented_units,
            expansion_shaders: self.expansion_shaders && rhs.expansion_shaders,
        }
    }
}

impl std::ops::BitAndAssign for FeatureFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.expansion &= rhs.expansion;
        self.quality &= rhs.quality;
        self.rail_bridges &= rhs.rail_bridges;
        self.space_travel &= rhs.space_travel;
        self.spoiling &= rhs.spoiling;
        self.freezing &= rhs.freezing;
        self.segmented_units &= rhs.segmented_units;
        self.expansion_shaders &= rhs.expansion_shaders;
    }
}

impl std::ops::BitXor for FeatureFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            expansion: self.expansion ^ rhs.expansion,
            quality: self.quality ^ rhs.quality,
            rail_bridges: self.rail_bridges ^ rhs.rail_bridges,
            space_travel: self.space_travel ^ rhs.space_travel,
            spoiling: self.spoiling ^ rhs.spoiling,
            freezing: self.freezing ^ rhs.freezing,
            segmented_units: self.segmented_units ^ rhs.segmented_units,
            expansion_shaders: self.expansion_shaders ^ rhs.expansion_shaders,
        }
    }
}

impl std::ops::BitXorAssign for FeatureFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.expansion ^= rhs.expansion;
        self.quality ^= rhs.quality;
        self.rail_bridges ^= rhs.rail_bridges;
        self.space_travel ^= rhs.space_travel;
        self.spoiling ^= rhs.spoiling;
        self.freezing ^= rhs.freezing;
        self.segmented_units ^= rhs.segmented_units;
        self.expansion_shaders ^= rhs.expansion_shaders;
    }
}
