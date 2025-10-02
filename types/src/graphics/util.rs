use image::{DynamicImage, GenericImageView, imageops};
use mod_util::UsedMods;

use super::{GraphicsOutput, RenderableGraphics};
use crate::ImageCache;

pub fn merge_layers<O, T: RenderableGraphics<RenderOpts = O>>(
    layers: &[T],
    scale: f64,
    used_mods: &UsedMods,
    image_cache: &mut ImageCache,
    opts: &O,
) -> Option<GraphicsOutput> {
    let layers = layers
        .iter()
        .map(|layer| layer.render(scale, used_mods, image_cache, opts))
        .collect::<Vec<_>>();

    merge_renders(layers.as_slice(), scale)
}

#[must_use]
pub fn merge_renders(renders: &[Option<GraphicsOutput>], scale: f64) -> Option<GraphicsOutput> {
    const TILE_RES: f64 = 32.0;

    let renders = renders
        .iter()
        .filter_map(|x| x.as_ref())
        .collect::<Vec<_>>();

    if renders.is_empty() {
        return None;
    }

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for (img, shift) in &renders {
        let (shift_x, shift_y) = shift.as_tuple();
        let (width, height) = img.dimensions();
        let width = f64::from(width) * scale / TILE_RES;
        let height = f64::from(height) * scale / TILE_RES;

        let x = shift_x - (width / 2.0);
        let y = shift_y - (height / 2.0);

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }

    let px_per_tile = TILE_RES / scale;
    let width = (max_x - min_x) * px_per_tile;
    let height = (max_y - min_y) * px_per_tile;
    let res_shift = (min_x.midpoint(max_x), min_y.midpoint(max_y));
    let center = (
        res_shift.0.mul_add(-px_per_tile, width / 2.0),
        res_shift.1.mul_add(-px_per_tile, height / 2.0),
    );

    let mut combined = DynamicImage::new_rgba8(width.ceil() as u32, height.ceil() as u32);

    for (img, shift) in &renders {
        let (shift_x, shift_y) = shift.as_tuple();
        let (post_width, post_height) = img.dimensions();
        let x = shift_x.mul_add(px_per_tile, center.0 - (f64::from(post_width) / 2.0));
        let y = shift_y.mul_add(px_per_tile, center.1 - (f64::from(post_height) / 2.0));

        imageops::overlay(&mut combined, img, x.round() as i64, y.round() as i64);
    }

    Some((combined, res_shift.into()))
}
