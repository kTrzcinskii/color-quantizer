use std::collections::HashMap;

use egui::{Color32, ColorImage};
use rayon::prelude::*;

use crate::algorithms::{DitheringParameters, PopularityParameters};

pub trait ColorQuantizer {
    type Params;

    fn generate_output_image(params: Self::Params, initial_image: &ColorImage) -> ColorImage;
}

struct DitheringCommon;

impl DitheringCommon {
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

pub struct AverageDitheringColorQuantizer;

impl ColorQuantizer for AverageDitheringColorQuantizer {
    type Params = DitheringParameters;

    fn generate_output_image(params: Self::Params, initial_image: &ColorImage) -> ColorImage {
        let r_levels = DitheringCommon::generate_color_levels(params.k_r);
        let g_levels = DitheringCommon::generate_color_levels(params.k_g);
        let b_levels = DitheringCommon::generate_color_levels(params.k_b);
        let output_pixels: Vec<_> = initial_image
            .pixels
            .par_chunks(256)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .flat_map(|&pixel| {
                        let r = DitheringCommon::find_closest_level(pixel.r(), &r_levels);
                        let g = DitheringCommon::find_closest_level(pixel.g(), &g_levels);
                        let b = DitheringCommon::find_closest_level(pixel.b(), &b_levels);
                        egui::Color32::from_rgb(r, g, b).to_array()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let size = initial_image.size;
        ColorImage::from_rgba_unmultiplied(size, output_pixels.as_slice())
    }
}

// TODO: for bigger Ks it's quite slow, any idea to speed it up?
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

pub struct ErrorDiffusionDitheringColorQuantizer;

impl ErrorDiffusionDitheringColorQuantizer {
    fn find_closest_level_and_diff(value: u8, levels: &[u8]) -> (u8, f32) {
        let level = DitheringCommon::find_closest_level(value, levels);
        let diff = value as f32 - level as f32;
        (level, diff)
    }

    const ERROR_WAGE_MATRIX: [f32; 4] = [0.4375, 0.1875, 0.3125, 0.0625];
    const OFFSET: usize = 5;
}

impl ColorQuantizer for ErrorDiffusionDitheringColorQuantizer {
    type Params = DitheringParameters;

    fn generate_output_image(params: Self::Params, initial_image: &ColorImage) -> ColorImage {
        let r_levels = DitheringCommon::generate_color_levels(params.k_r);
        let g_levels = DitheringCommon::generate_color_levels(params.k_g);
        let b_levels = DitheringCommon::generate_color_levels(params.k_b);

        let mut output_pixels = initial_image.pixels.clone();
        for i in 0..output_pixels.len() {
            let pixel = output_pixels[i];
            let (r, r_diff) = ErrorDiffusionDitheringColorQuantizer::find_closest_level_and_diff(
                pixel.r(),
                &r_levels,
            );
            let (g, g_diff) = ErrorDiffusionDitheringColorQuantizer::find_closest_level_and_diff(
                pixel.g(),
                &g_levels,
            );
            let (b, b_diff) = ErrorDiffusionDitheringColorQuantizer::find_closest_level_and_diff(
                pixel.b(),
                &b_levels,
            );
            output_pixels[i] = Color32::from_rgb(r, g, b);
            for j in 0..ErrorDiffusionDitheringColorQuantizer::ERROR_WAGE_MATRIX.len() {
                let id = i + j + ErrorDiffusionDitheringColorQuantizer::OFFSET;
                if id >= output_pixels.len() {
                    break;
                }
                let to_change = output_pixels[id];
                let new_r = to_change.r() as f32
                    + ErrorDiffusionDitheringColorQuantizer::ERROR_WAGE_MATRIX[j] * r_diff;
                let new_g = to_change.g() as f32
                    + ErrorDiffusionDitheringColorQuantizer::ERROR_WAGE_MATRIX[j] * g_diff;
                let new_b = to_change.b() as f32
                    + ErrorDiffusionDitheringColorQuantizer::ERROR_WAGE_MATRIX[j] * b_diff;
                output_pixels[id] = Color32::from_rgb(new_r as u8, new_g as u8, new_b as u8);
            }
        }
        let size = initial_image.size;
        ColorImage::from_rgba_unmultiplied(
            size,
            output_pixels
                .iter()
                .flat_map(|&p| p.to_array())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}

pub struct OrderedDitheringCommon;

impl OrderedDitheringCommon {
    const POSSIBLE_N: [u8; 7] = [2, 3, 4, 6, 8, 12, 16];

    fn find_n(k: u8) -> u8 {
        *Self::POSSIBLE_N
            .iter()
            .find(|&&n| n as u32 * n as u32 * (k as u32 - 1) >= 256)
            .unwrap_or(Self::POSSIBLE_N.last().unwrap())
    }

    fn generate_matrix(n: u8) -> Vec<Vec<u32>> {
        let mut matrix: Vec<Vec<u32>> = vec![vec![0; n as usize]; n as usize];
        if n == 2 {
            matrix[0][0] = 0;
            matrix[0][1] = 2;
            matrix[1][0] = 3;
            matrix[1][1] = 1;
            return matrix;
        }
        if n == 3 {
            matrix[0][0] = 6;
            matrix[0][1] = 8;
            matrix[0][2] = 4;
            matrix[1][0] = 1;
            matrix[1][1] = 0;
            matrix[1][2] = 3;
            matrix[2][0] = 5;
            matrix[2][1] = 2;
            matrix[2][2] = 7;
            return matrix;
        }

        let half_matrix = Self::generate_matrix(n / 2);
        let half_n = n as usize / 2;
        for i in 0..half_n {
            for j in 0..half_n {
                matrix[i][j] = 4 * half_matrix[i][j];
                matrix[i][j + half_n] = 4 * half_matrix[i][j] + 2;
                matrix[i + half_n][j] = 4 * half_matrix[i][j] + 3;
                matrix[i + half_n][j + half_n] = 4 * half_matrix[i][j] + 1;
            }
        }
        matrix
    }
}

pub struct OrderedDitheringRelativeColorQuantizer;

impl OrderedDitheringRelativeColorQuantizer {
    fn get_color(
        value: u8,
        levels: &[u8],
        matrix: &[Vec<u32>],
        x: usize,
        y: usize,
        n: usize,
    ) -> u8 {
        let n_sq = n * n;
        let scaled_value = value as usize * (levels.len() - 1);
        let col = scaled_value / 255;
        let re = scaled_value % 255;
        let i = x % n;
        let j = y % n;
        let final_col = if re > (matrix[i][j] as usize * 255 / n_sq) {
            col + 1
        } else {
            col
        };
        levels[final_col]
    }
}

impl ColorQuantizer for OrderedDitheringRelativeColorQuantizer {
    type Params = DitheringParameters;

    fn generate_output_image(params: Self::Params, initial_image: &ColorImage) -> ColorImage {
        let r_levels = DitheringCommon::generate_color_levels(params.k_r);
        let g_levels = DitheringCommon::generate_color_levels(params.k_g);
        let b_levels = DitheringCommon::generate_color_levels(params.k_b);

        let n_r = OrderedDitheringCommon::find_n(params.k_r);
        let m_r = OrderedDitheringCommon::generate_matrix(n_r);
        let n_g = OrderedDitheringCommon::find_n(params.k_g);
        let m_g = OrderedDitheringCommon::generate_matrix(n_g);
        let n_b = OrderedDitheringCommon::find_n(params.k_b);
        let m_b = OrderedDitheringCommon::generate_matrix(n_b);

        const CHUNK_SIZE: usize = 512;

        let size = initial_image.size;

        let output_pixesl: Vec<_> = initial_image
            .pixels
            .par_chunks(CHUNK_SIZE)
            .enumerate()
            .flat_map(|(chunk_id, chunk)| {
                chunk
                    .iter()
                    .enumerate()
                    .flat_map(|(pixel_id, &pixel)| {
                        let id = chunk_id * CHUNK_SIZE + pixel_id;
                        let x = id / size[0];
                        let y = id - x * size[0];
                        let new_r = OrderedDitheringRelativeColorQuantizer::get_color(
                            pixel.r(),
                            &r_levels,
                            &m_r,
                            x,
                            y,
                            n_r as usize,
                        );
                        let new_g = OrderedDitheringRelativeColorQuantizer::get_color(
                            pixel.g(),
                            &g_levels,
                            &m_g,
                            x,
                            y,
                            n_g as usize,
                        );
                        let new_b = OrderedDitheringRelativeColorQuantizer::get_color(
                            pixel.b(),
                            &b_levels,
                            &m_b,
                            x,
                            y,
                            n_b as usize,
                        );
                        let new_pixel = Color32::from_rgb(new_r, new_g, new_b);
                        new_pixel.to_array()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        ColorImage::from_rgba_unmultiplied(size, output_pixesl.as_slice())
    }
}
