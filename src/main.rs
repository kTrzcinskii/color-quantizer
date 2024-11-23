use color_quantizer::app::App;
use eframe::NativeOptions;

fn main() {
    let app = App::default();
    let options = NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Color Quantizer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    ) {
        eprintln!("Error occured: {}", e);
    }
}
