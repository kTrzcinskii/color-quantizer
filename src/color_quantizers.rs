use std::collections::HashMap;

use egui::{Color32, ColorImage};
use rayon::prelude::*;

use crate::algorithms::{DitheringParameters, PopularityParameters};

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
        let r_levels = Self::generate_color_levels(params.k_r);
        let g_levels = Self::generate_color_levels(params.k_g);
        let b_levels = Self::generate_color_levels(params.k_b);
        let output_pixels: Vec<_> = initial_image
            .pixels
            .par_chunks(256)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .flat_map(|&pixel| {
                        let r = Self::find_closest_level(pixel.r(), &r_levels);
                        let g = Self::find_closest_level(pixel.g(), &g_levels);
                        let b = Self::find_closest_level(pixel.b(), &b_levels);
                        egui::Color32::from_rgb(r, g, b).to_array()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let size = initial_image.size;
        ColorImage::from_rgba_unmultiplied(size, output_pixels.as_slice())
    }
}

pub struct PopularityAlgorithmColorQuantizer;

impl PopularityAlgorithmColorQuantizer {
    fn find_most_popular_k_colors(initial_image: &ColorImage, k: usize) -> Vec<Color32> {
        let mut colors_count = HashMap::<Color32, usize>::new();
        for pixel in &initial_image.pixels {
            *colors_count.entry(*pixel).or_insert(0) += 1;
        }
        let mut colors_vec: Vec<(Color32, usize)> = colors_count.into_iter().collect();
        colors_vec.sort_by(|&lhs, &rhs| rhs.1.cmp(&lhs.1));
        colors_vec.into_iter().take(k).map(|c| c.0).collect()
    }

    fn find_closest_color(pixel: Color32, colors: &[Color32]) -> Color32 {
        colors
            .iter()
            .min_by(|&lhs, &rhs| {
                let lhs_dist = Self::colors_distance(pixel, *lhs);
                let rhs_dist = Self::colors_distance(pixel, *rhs);
                lhs_dist
                    .partial_cmp(&rhs_dist)
                    .expect("Colors distances should always be comparable")
            })
            .copied()
            .expect("Color should never be empty")
    }

    fn colors_distance(lhs: Color32, rhs: Color32) -> f32 {
        let r_diff = lhs.r() as f32 - rhs.r() as f32;
        let g_diff = lhs.g() as f32 - rhs.g() as f32;
        let b_diff = lhs.b() as f32 - rhs.b() as f32;

        r_diff * r_diff + g_diff * g_diff + b_diff * b_diff
    }
}

impl ColorQuantizer for PopularityAlgorithmColorQuantizer {
    type Params = PopularityParameters;

    fn generate_output_image(params: Self::Params, initial_image: &ColorImage) -> ColorImage {
        let colors = Self::find_most_popular_k_colors(initial_image, params.k);
        let output_pixesl: Vec<_> = initial_image
            .pixels
            .par_chunks(256)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .flat_map(|&pixel| Self::find_closest_color(pixel, &colors).to_array())
                    .collect::<Vec<_>>()
            })
            .collect();
        let size = initial_image.size;
        ColorImage::from_rgba_unmultiplied(size, output_pixesl.as_slice())
    }
}
