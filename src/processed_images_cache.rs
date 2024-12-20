use std::num::NonZero;

use egui::ColorImage;
use lru::LruCache;

use crate::{
    algorithms::{Algorithm, AlgorithmCacheKey, AlgorithmParameters},
    color_quantizers::{
        AverageDitheringColorQuantizer, ColorQuantizer, ErrorDiffusionDitheringColorQuantizer,
        OrderedDitheringRandomColorQuantizer, OrderedDitheringRelativeColorQuantizer,
        PopularityAlgorithmColorQuantizer,
    },
};

pub struct ProcessedImagesCache {
    cache: LruCache<AlgorithmCacheKey, ColorImage>,
}

impl ProcessedImagesCache {
    pub fn new(size: NonZero<usize>) -> ProcessedImagesCache {
        let cache = LruCache::new(size);
        ProcessedImagesCache { cache }
    }

    // Returns image for given algorithm and parameters
    // If no image match provided criteria, new image is created with proper algorithm
    pub fn get(&mut self, key: AlgorithmCacheKey, initial_image: &ColorImage) -> &ColorImage {
        self.cache
            .get_or_insert(key, || Self::create_new_image(&key, initial_image))
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    fn create_new_image(key: &AlgorithmCacheKey, initial_image: &ColorImage) -> ColorImage {
        match key.algorithm {
            Algorithm::AverageDithering => {
                let params = match key.params {
                    AlgorithmParameters::Dithering(dithering_parameters) => dithering_parameters,
                    AlgorithmParameters::Popularity(_) => panic!("UNREACHABLE"),
                };
                AverageDitheringColorQuantizer::generate_output_image(params, initial_image)
            }
            Algorithm::ErrorDiffusionDithering => {
                let params = match key.params {
                    AlgorithmParameters::Dithering(dithering_parameters) => dithering_parameters,
                    AlgorithmParameters::Popularity(_) => panic!("UNREACHABLE"),
                };
                ErrorDiffusionDitheringColorQuantizer::generate_output_image(params, initial_image)
            }
            Algorithm::OrderedDitheringRandom => {
                let params = match key.params {
                    AlgorithmParameters::Dithering(dithering_parameters) => dithering_parameters,
                    AlgorithmParameters::Popularity(_) => panic!("UNREACHABLE"),
                };
                OrderedDitheringRandomColorQuantizer::generate_output_image(params, initial_image)
            }
            Algorithm::OrderedDitheringRelative => {
                let params = match key.params {
                    AlgorithmParameters::Dithering(dithering_parameters) => dithering_parameters,
                    AlgorithmParameters::Popularity(_) => panic!("UNREACHABLE"),
                };
                OrderedDitheringRelativeColorQuantizer::generate_output_image(params, initial_image)
            }
            Algorithm::PopularityAlgorithm => {
                let params = match key.params {
                    AlgorithmParameters::Dithering(_) => panic!("UNREACHABLE"),
                    AlgorithmParameters::Popularity(popularity_parameters) => popularity_parameters,
                };
                PopularityAlgorithmColorQuantizer::generate_output_image(params, initial_image)
            }
        }
    }
}
