use std::collections::{HashMap, HashSet};
use std::ops::Rem;

use image::{ConvertColorOptions, DynamicImage, GenericImageView, GrayAlphaImage, imageops};
use imageproc::geometric_transformations;
use signed_distance_field::prelude::*;

use tracing::instrument;
use types::*;

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
    fn pos_to_pixel_f64(&self, position: &MapPosition) -> (f64, f64) {
        let (x, y) = position.as_tuple();
        let (tl_x, tl_y) = self.top_left.as_tuple();

        let px = (x - tl_x) * self.tile_res;
        let py = (y - tl_y) * self.tile_res;

        (px, py)
    }

    #[must_use]
    fn pos_to_pixel(&self, position: &MapPosition) -> (i64, i64) {
        let (px, py) = self.pos_to_pixel_f64(position);
        (px.round() as i64, py.round() as i64)
    }

    #[must_use]
    fn len_to_pixel(&self, len: f64) -> i64 {
        (len * self.tile_res).round() as i64
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
    layers: HashMap<RenderLayer, image::DynamicImage>,

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

    fn get_layer(&mut self, layer: RenderLayer) -> &mut image::DynamicImage {
        self.layers.entry(layer).or_insert_with(|| {
            image::DynamicImage::new_rgba8(self.target_size.width, self.target_size.height)
        })
    }

    pub fn add(
        &mut self,
        (img, shift): (image::DynamicImage, Vector),
        position: &MapPosition,
        layer: RenderLayer,
    ) {
        let (x, y) = self
            .target_size
            .get_pixel_pos(img.dimensions(), &shift, position);

        let layer = self.get_layer(layer);
        imageops::overlay(layer, &img, x, y);
    }

    pub fn add_entity(&mut self, input: (image::DynamicImage, Vector), position: &MapPosition) {
        self.add(input, position, RenderLayer::Object);
    }

    pub fn add_shadow(&mut self, input: (image::DynamicImage, Vector), position: &MapPosition) {
        self.add(input, position, RenderLayer::Floor);
    }

    pub fn draw_box(&mut self, bbox: &BoundingBox, position: &MapPosition, color: [u8; 4]) {
        let tl = bbox.top_left() + position;
        let (tl_x, tl_y) = self.target_size.pos_to_pixel(&tl);
        let width = self.target_size.len_to_pixel(bbox.width()) as u32;
        let height = self.target_size.len_to_pixel(bbox.height()) as u32;

        let rect = imageproc::rect::Rect::at(tl_x as i32, tl_y as i32).of_size(width, height);
        imageproc::drawing::draw_hollow_rect_mut(
            self.get_layer(RenderLayer::Cursor),
            rect,
            color.into(),
        );
    }

    pub fn draw_dot(&mut self, position: &MapPosition, color: [u8; 4]) {
        let (x, y) = self.target_size.pos_to_pixel(position);
        let scale = self.scale();

        imageproc::drawing::draw_filled_circle_mut(
            self.get_layer(RenderLayer::Cursor),
            (x as i32, y as i32),
            ((32.0 / scale) * 0.1).ceil() as i32,
            color.into(),
        );
    }

    pub fn draw_direction(&mut self, position: &MapPosition, direction: Direction, color: [u8; 4]) {
        let (s_x, s_y) = self.target_size.pos_to_pixel_f64(position);

        let end = position + &MapPosition::from(direction.get_offset() * 0.5);
        let (e_x, e_y) = self.target_size.pos_to_pixel_f64(&end);

        imageproc::drawing::draw_line_segment_mut(
            self.get_layer(RenderLayer::Cursor),
            (s_x as f32, s_y as f32),
            (e_x as f32, e_y as f32),
            color.into(),
        );
    }

    #[must_use]
    pub const fn scale(&self) -> f64 {
        self.target_size.scale
    }

    pub(crate) fn store_wire_connection_points(
        &mut self,
        bp_entity_id: u64,
        wire_connection_points: &GenericWireConnectionPoint,
    ) {
        self.wire_connection_points
            .insert(bp_entity_id, *wire_connection_points);
    }

    #[instrument(skip_all)]
    fn generate_wire_draw_data<'a>(
        &self,
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
                                    .filter(|((w, s), t)| {
                                        **s && **t || *s_is_switch && **s && *w == 0
                                    })
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

    #[instrument(skip_all)]
    pub fn draw_wires(
        &mut self,
        wire_data: &EntityWireConnections,
        util_sprites: &crate::utility_sprites::UtilitySprites,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) {
        let dd = self.generate_wire_draw_data(wire_data);
        let count = dd.iter().map(std::vec::Vec::len).sum::<usize>();

        if count > 10_000 {
            tracing::warn!("too many wires to draw ({count})");
            return;
        }

        tracing::info!("drawing wires");

        let target_size = self.target_size.clone();
        let layer = self.get_layer(RenderLayer::Wires);

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
                &TintableRenderOpts::default(),
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
                    f64::from(base_wire_width - offset).mul_add(horiz_crop_fac, f64::from(offset));

                // magic numbers :)
                let vert_crop_fac = 5.6f64.mul_add(
                    (horiz_crop_fac / 2.0).powi(4),
                    2.6 * (horiz_crop_fac / 2.0).powi(2),
                );
                let cropped_height =
                    f64::from(base_wire_height - offset).mul_add(vert_crop_fac, f64::from(offset));

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

                if w == 0 || h == 0 {
                    continue;
                }

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
                    RenderLayer::Wires,
                );
            }
        }
    }

    #[instrument(skip_all)]
    pub fn generate_background(&mut self) {
        let lab_tile_dark = image::Luma([0x1bu8]);
        let lab_tile_light = image::Luma([0x31u8]);

        let (tl_x, tl_y) = self.target_size.top_left.as_tuple();
        let tile_res = self.target_size.tile_res;

        let background =
            image::ImageBuffer::from_fn(self.target_size.width, self.target_size.height, |x, y| {
                let x = (f64::from(x) / tile_res) - tl_x;
                let y = (f64::from(y) / tile_res) - tl_y;

                let p_x = x.rem(2.0);
                let p_y = y.rem(2.0);

                if p_x < 1.0 && p_y < 1.0 || p_x >= 1.0 && p_y >= 1.0 {
                    lab_tile_dark
                } else {
                    lab_tile_light
                }
            });

        self.layers.insert(RenderLayer::Zero, background.into());
    }

    #[must_use]
    #[instrument(skip_all)]
    pub fn combine(&mut self) -> image::DynamicImage {
        'sdf_outline: {
            let Some(icons) = self.layers.get(&RenderLayer::EntityInfoIconAbove) else {
                break 'sdf_outline;
            };

            let (width, height) = icons.dimensions();
            let mask = image::ImageBuffer::from_fn(width, height, |x, y| {
                let alpha = icons.get_pixel(x, y).0[3];
                image::Luma([alpha])
            });

            let Some(normalized_sdf) = compute_f32_distance_field(
                &binary_image::of_byte_slice_with_threshold(&mask, width as u16, height as u16, 1),
            )
            .normalize_clamped_distances(0.0, (6.0 / self.scale()) as f32) else {
                break 'sdf_outline;
            };

            let normalized = normalized_sdf
                .to_u8()
                .iter()
                .flat_map(|&x| {
                    if x == 255 {
                        [0, 0]
                    } else {
                        [0, 255 - (f64::from(x).powi(2) / 255f64).round() as u8]
                    }
                })
                .collect::<Vec<_>>();

            let Some(outline_img) = GrayAlphaImage::from_vec(width, height, normalized) else {
                break 'sdf_outline;
            };

            let Ok(mut icons_with_sdf) =
                outline_img.to_color_space(icons.color_space(), ConvertColorOptions::default())
            else {
                break 'sdf_outline;
            };

            imageops::overlay(&mut icons_with_sdf, icons, 0, 0);

            let icons = self.get_layer(RenderLayer::EntityInfoIconAbove);
            icons.clone_from(&icons_with_sdf.into());
        }

        let mut combined =
            image::DynamicImage::new_rgba8(self.target_size.width, self.target_size.height);

        let mut layer_keys = self.layers.keys().collect::<Vec<_>>();
        layer_keys.sort_unstable();

        for layer in layer_keys {
            if let Some(img) = self.layers.get(layer) {
                imageops::overlay(&mut combined, img, 0, 0);
            }
        }

        combined
    }
}
