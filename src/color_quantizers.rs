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
            .map(|i| ((i as f32) * 255.0 / (k - 1) as f32).round() as u8)
            .collect()
    }

    fn find_closest_level(value: u8, levels: &[u8]) -> u8 {
        if value <= levels[0] {
            return levels[0];
        }
        if value >= levels[levels.len() - 1] {
            return levels[levels.len() - 1];
        }

        // We can use bin search because we know levels are sorted
        match levels.binary_search(&value) {
            Ok(idx) => levels[idx],
            Err(idx) => {
                // At this point we know that it's not gonna be first nor last, because we check it at the beginning
                let prev = levels[idx - 1];
                let next = levels[idx];
                if (value - prev) <= (next - value) {
                    prev
                } else {
                    next
                }
            }
        }
    }
}

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
