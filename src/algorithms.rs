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
