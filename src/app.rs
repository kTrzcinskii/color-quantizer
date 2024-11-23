use strum::IntoEnumIterator;

use crate::algorithms::{Algorithm, AlgorithmType, DitheringParameters, PopularityParameters};

pub struct App {
    algorithm: Algorithm,
    dithering_parameters: DitheringParameters,
    popularity_algorithm_parameters: PopularityParameters,
}

impl App {
    fn show_controls_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("controls_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Choose algorithm");
                ui.separator();
                for alg in Algorithm::iter() {
                    ui.radio_value(&mut self.algorithm, alg, format!("{}", alg));
                    ui.add_space(8.0);
                }
                match AlgorithmType::from(self.algorithm) {
                    AlgorithmType::Dithering => self.show_dithering_parameters(ctx),
                    AlgorithmType::Popularity => self.show_popularity_parameters(ctx),
                }
            });
    }

    fn show_dithering_parameters(&mut self, ctx: &egui::Context) {
        // TODO:
    }

    fn show_popularity_parameters(&mut self, ctx: &egui::Context) {
        // TODO:
    }

    fn show_central_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Central panel");
        });
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::AverageDithering,
            dithering_parameters: DitheringParameters {},
            popularity_algorithm_parameters: PopularityParameters {},
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_controls_panel(ctx);
        self.show_central_panel(ctx);
    }
}
