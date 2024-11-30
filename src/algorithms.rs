use std::fmt::Display;

use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Algorithm {
    AverageDithering,
    ErrorDiffusionDithering,
    OrderedDitheringRandom,
    OrderedDitheringRelative,
    PopularityAlgorithm,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::AverageDithering => write!(f, "Average Dithering"),
            Algorithm::ErrorDiffusionDithering => write!(f, "Error Diffusion Dithering"),
            Algorithm::OrderedDitheringRandom => write!(f, "Ordered Dithering Random"),
            Algorithm::OrderedDitheringRelative => write!(f, "Ordered Dithering Relative"),
            Algorithm::PopularityAlgorithm => write!(f, "Popularity Algorithm"),
        }
    }
}

pub enum AlgorithmType {
    Dithering,
    Popularity,
}

impl From<Algorithm> for AlgorithmType {
    fn from(value: Algorithm) -> Self {
        match value {
            Algorithm::AverageDithering => AlgorithmType::Dithering,
            Algorithm::ErrorDiffusionDithering => AlgorithmType::Dithering,
            Algorithm::OrderedDitheringRandom => AlgorithmType::Dithering,
            Algorithm::OrderedDitheringRelative => AlgorithmType::Dithering,
            Algorithm::PopularityAlgorithm => AlgorithmType::Popularity,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DitheringParameters {
    pub k_r: u8,
    pub k_g: u8,
    pub k_b: u8,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PopularityParameters {
    pub k: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AlgorithmParameters {
    Dithering(DitheringParameters),
    Popularity(PopularityParameters),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AlgorithmCacheKey {
    pub algorithm: Algorithm,
    pub params: AlgorithmParameters,
}
