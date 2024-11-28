use std::num::NonZero;

use rfd::FileDialog;
use strum::IntoEnumIterator;

use crate::{
    algorithms::{
        Algorithm, AlgorithmCacheKey, AlgorithmParameters, AlgorithmType, DitheringParameters,
        PopularityParameters,
    },
    image_loader,
    processed_images_cache::ProcessedImagesCache,
};

const CACHE_SIZE: usize = 512;

pub struct App {
    algorithm: Algorithm,
    dithering_parameters: DitheringParameters,
    popularity_algorithm_parameters: PopularityParameters,
    initial_image: Option<egui::ColorImage>,
    processed_images_cache: ProcessedImagesCache,
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
                    AlgorithmType::Dithering => self.show_dithering_parameters(ui),
                    AlgorithmType::Popularity => self.show_popularity_parameters(ctx),
                }
                if self.initial_image.is_some() {
                    self.show_change_image_button(ui);
                }
            });
    }

    fn show_dithering_parameters(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add(egui::Slider::new(&mut self.dithering_parameters.k_r, 2..=255).text("Kr"));
            ui.add(egui::Slider::new(&mut self.dithering_parameters.k_g, 2..=255).text("Kg"));
            ui.add(egui::Slider::new(&mut self.dithering_parameters.k_b, 2..=255).text("Kb"));
        });
    }

    fn show_popularity_parameters(&mut self, ctx: &egui::Context) {
        // TODO:
    }

    fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| match &self.initial_image {
            Some(initial_image) => {
                self.show_images(ctx, ui, initial_image.to_owned());
            }
            None => {
                self.show_load_initial_image_button(ui);
            }
        });
    }

    fn show_images(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        initial_image: egui::ColorImage,
    ) {
        let available_rect = ui.available_rect_before_wrap();
        let max_width = available_rect.width() / 3.0;
        let alg_cache_key = self.current_algorithm_cache_key();
        let processed_image = self
            .processed_images_cache
            .get(alg_cache_key, &initial_image);

        let image_texture = ctx.load_texture("INITIAL_IMAGE", initial_image, Default::default());
        let processed_image_texture = ctx.load_texture(
            "PROCESSED_IMAGE",
            processed_image.to_owned(),
            Default::default(),
        );

        let normal_image = egui::Image::new(&image_texture).max_width(max_width);
        let processed_image = egui::Image::new(&processed_image_texture).max_width(max_width);

        let img_width = normal_image
            .size()
            .map(|v| v[0])
            .unwrap_or(max_width)
            .min(max_width);
        let space_width =
            (1.0 - img_width * 2.0 / available_rect.width()) / 3.0 * available_rect.width();

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(space_width);
            ui.add(normal_image);
            ui.add_space(space_width);
            ui.add(processed_image);
        });
    }

    fn show_load_initial_image_button(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            let button_size = ui.spacing().interact_size * 2.0;
            let available_rect = ui.available_rect_before_wrap();
            ui.add_space(available_rect.height() / 2.0 - button_size.y);
            if ui
                .add_sized(button_size, egui::Button::new("Load image"))
                .clicked()
            {
                self.file_dialog_change_image();
            }
        });
    }

    fn show_change_image_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Change image").clicked() {
            self.file_dialog_change_image();
            self.processed_images_cache.clear();
        }
    }

    fn file_dialog_change_image(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg"])
            .pick_file()
        {
            self.initial_image = Some(image_loader::load_image_from_path(path).unwrap());
        }
    }

    fn current_algorithm_cache_key(&self) -> AlgorithmCacheKey {
        let algorithm = self.algorithm;
        let params = match AlgorithmType::from(algorithm) {
            AlgorithmType::Dithering => AlgorithmParameters::Dithering(self.dithering_parameters),
            AlgorithmType::Popularity => {
                AlgorithmParameters::Popularity(self.popularity_algorithm_parameters)
            }
        };
        AlgorithmCacheKey { algorithm, params }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::AverageDithering,
            dithering_parameters: DitheringParameters::default(),
            popularity_algorithm_parameters: PopularityParameters {},
            initial_image: None,
            processed_images_cache: ProcessedImagesCache::new(NonZero::new(CACHE_SIZE).unwrap()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_controls_panel(ctx);
        self.show_central_panel(ctx);
    }
}
