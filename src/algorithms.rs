use std::fmt::Display;

use strum_macros::EnumIter;

#[derive(EnumIter, PartialEq, Clone, Copy)]
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

pub struct DitheringParameters {
    // TODO:
}

pub struct PopularityParameters {
    // TODO:
}
