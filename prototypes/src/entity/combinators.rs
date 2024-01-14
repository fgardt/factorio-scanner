use std::ops::Deref;

use serde::{Deserialize, Serialize};

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/CombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/CombinatorPrototype.html)
pub type CombinatorPrototype<T> = EntityWithOwnerPrototype<CombinatorData<T>>;

/// [`Prototypes/CombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/CombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct CombinatorData<T: super::Renderable> {
    pub energy_source: AnyEnergySource, // theoretically only electric and void are valid
    pub active_energy_usage: Energy,
    pub sprites: Option<Sprite4Way>,
    pub activity_led_sprites: Option<Sprite4Way>,
    pub input_connection_bounding_box: BoundingBox,
    pub output_connection_bounding_box: BoundingBox,
    pub activity_led_light_offsets: (Vector, Vector, Vector, Vector),
    pub screen_light_offsets: (Vector, Vector, Vector, Vector),
    pub input_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),
    pub output_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),

    pub activity_led_light: Option<LightDefinition>,
    pub screen_light: Option<LightDefinition>,

    #[serde(
        default = "helper::u8_5",
        skip_serializing_if = "helper::is_5_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub activity_led_hold_time: u8,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    #[serde(flatten)]
    child: T,
}

impl<T: super::Renderable> Deref for CombinatorData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: super::Renderable> super::Renderable for CombinatorData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let mut empty = true;

        if let Some(res) = self.sprites.as_ref().and_then(|s| {
            s.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add_entity(res, &options.position);
        }

        let child = self
            .child
            .render(options, used_mods, render_layers, image_cache);

        if empty && child.is_none() {
            None
        } else {
            Some(())
        }
    }
}

/// [`Prototypes/ArithmeticCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ArithmeticCombinatorPrototype.html)
pub type ArithmeticCombinatorPrototype = CombinatorPrototype<ArithmeticCombinatorData>;

/// [`Prototypes/ArithmeticCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ArithmeticCombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ArithmeticCombinatorData {
    pub plus_symbol_sprites: Option<Sprite4Way>,
    pub minus_symbol_sprites: Option<Sprite4Way>,
    pub multiply_symbol_sprites: Option<Sprite4Way>,
    pub divide_symbol_sprites: Option<Sprite4Way>,
    pub modulo_symbol_sprites: Option<Sprite4Way>,
    pub power_symbol_sprites: Option<Sprite4Way>,
    pub left_shift_symbol_sprites: Option<Sprite4Way>,
    pub right_shift_symbol_sprites: Option<Sprite4Way>,
    pub and_symbol_sprites: Option<Sprite4Way>,
    pub or_symbol_sprites: Option<Sprite4Way>,
    pub xor_symbol_sprites: Option<Sprite4Way>,
}

impl super::Renderable for ArithmeticCombinatorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = options.arithmetic_operation.as_ref().and_then(|op| {
            match op {
                ArithmeticOperation::Add => self.plus_symbol_sprites.as_ref(),
                ArithmeticOperation::Subtract => self.minus_symbol_sprites.as_ref(),
                ArithmeticOperation::Multiply => self.multiply_symbol_sprites.as_ref(),
                ArithmeticOperation::Divide => self.divide_symbol_sprites.as_ref(),
                ArithmeticOperation::Modulo => self.modulo_symbol_sprites.as_ref(),
                ArithmeticOperation::Power => self.power_symbol_sprites.as_ref(),
                ArithmeticOperation::LeftShift => self.left_shift_symbol_sprites.as_ref(),
                ArithmeticOperation::RightShift => self.right_shift_symbol_sprites.as_ref(),
                ArithmeticOperation::BitwiseAnd => self.and_symbol_sprites.as_ref(),
                ArithmeticOperation::BitwiseOr => self.or_symbol_sprites.as_ref(),
                ArithmeticOperation::BitwiseXor => self.xor_symbol_sprites.as_ref(),
            }
            .and_then(|s| {
                s.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            })
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

/// [`Prototypes/DeciderCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/DeciderCombinatorPrototype.html)
pub type DeciderCombinatorPrototype = CombinatorPrototype<DeciderCombinatorData>;

/// [`Prototypes/DeciderCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/DeciderCombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct DeciderCombinatorData {
    pub equal_symbol_sprites: Option<Sprite4Way>,
    pub greater_symbol_sprites: Option<Sprite4Way>,
    pub less_symbol_sprites: Option<Sprite4Way>,
    pub not_equal_symbol_sprites: Option<Sprite4Way>,
    pub greater_or_equal_symbol_sprites: Option<Sprite4Way>,
    pub less_or_equal_symbol_sprites: Option<Sprite4Way>,
}

impl super::Renderable for DeciderCombinatorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = options.decider_operation.as_ref().and_then(|op| {
            match op {
                Comparator::Equal => self.equal_symbol_sprites.as_ref(),
                Comparator::Greater => self.greater_symbol_sprites.as_ref(),
                Comparator::Less => self.less_symbol_sprites.as_ref(),
                Comparator::NotEqual => self.not_equal_symbol_sprites.as_ref(),
                Comparator::GreaterOrEqual => self.greater_or_equal_symbol_sprites.as_ref(),
                Comparator::LessOrEqual => self.less_or_equal_symbol_sprites.as_ref(),
            }
            .and_then(|s| {
                s.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            })
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

/// [`Prototypes/ConstantCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstantCombinatorPrototype.html)
pub type ConstantCombinatorPrototype = EntityWithOwnerPrototype<ConstantCombinatorData>;

/// [`Prototypes/ConstantCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstantCombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstantCombinatorData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub item_slot_count: u32,

    pub sprites: Option<Sprite4Way>,
    pub activity_led_sprites: Option<Sprite4Way>,
    pub activity_led_light_offsets: (Vector, Vector, Vector, Vector),
    pub circuit_wire_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),

    pub activity_led_light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default, skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default, skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,
}

impl super::Renderable for ConstantCombinatorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.sprites.as_ref().and_then(|s| {
            s.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}
