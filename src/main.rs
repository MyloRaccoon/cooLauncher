use coolauncher::launcher::Launcher;
use coolauncher::saver::Saver;
use coolauncher::tools::create_main_dir;
use eframe::egui;
use egui::ViewportBuilder;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {

    let _ = create_main_dir();

    let mut launcher = Launcher::new();
    launcher.load(Saver::load());
    let options = eframe::NativeOptions{
        viewport: ViewportBuilder::default()
            .with_inner_size([1200., 720.])
            .with_position([0., 0.]),
        ..Default::default()
    };
    eframe::run_native(
        "cooLauncher",
        options,
        Box::new(|cc| {
            Ok(Box::new(launcher.set_visuals_dark(cc)))
        }),
    )
}
