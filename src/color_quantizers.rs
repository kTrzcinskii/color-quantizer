use egui::ColorImage;
use rayon::prelude::*;

use crate::algorithms::DitheringParameters;

pub trait ColorQuantizer {
    type Params;

    fn generate_output_image(params: Self::Params, initial_image: &ColorImage) -> ColorImage;
}

pub struct AverageDitheringColorQuantizer;

impl AverageDitheringColorQuantizer {
    fn generate_color_levels(k: u8) -> Vec<u8> {
        (0..k)
            .map(|i| ((i as f64) * 255.0 / (k - 1) as f64).round() as u8)
            .collect()
    }

    fn find_closest_level(value: u8, levels: &[u8]) -> u8 {
        // We can use bin search because we know levels are sorted
        match levels.binary_search_by(|x| x.cmp(&value)) {
            Ok(id) => levels[id],
            Err(id) => {
                let length = levels.len();
                if id == 0 {
                    levels[0]
                } else if id == length {
                    levels[length - 1]
                } else {
                    let prev = levels[id - 1];
                    let next = levels[id];
                    if (value - prev) < (next - value) {
                        prev
                    } else {
                        next
                    }
                }
            }
        }
    }
}

// FIXME: colors seem a little off
impl ColorQuantizer for AverageDitheringColorQuantizer {
    type Params = DitheringParameters;

    fn generate_output_image(params: Self::Params, initial_image: &ColorImage) -> ColorImage {
        let r_levels = AverageDitheringColorQuantizer::generate_color_levels(params.k_r);
        let g_levels = AverageDitheringColorQuantizer::generate_color_levels(params.k_g);
        let b_levels = AverageDitheringColorQuantizer::generate_color_levels(params.k_b);
        let output_pixels: Vec<_> = initial_image
            .pixels
            .par_iter()
            .map(|pixel| {
                let r = AverageDitheringColorQuantizer::find_closest_level(pixel.r(), &r_levels);
                let g = AverageDitheringColorQuantizer::find_closest_level(pixel.g(), &g_levels);
                let b = AverageDitheringColorQuantizer::find_closest_level(pixel.b(), &b_levels);
                egui::Color32::from_rgb(r, g, b).to_array()
            })
            .flatten()
            .collect();
        let size = initial_image.size;
        ColorImage::from_rgba_unmultiplied(size, output_pixels.as_slice())
    }
}
