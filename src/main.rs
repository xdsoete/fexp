use eframe::egui::{self, CentralPanel};
use eframe::{self};
use navigation::Navigator;
use std::path::Path;
use file_ops::list_directory_contents;

pub mod file_ops;
pub mod navigation;

fn main() -> Result<(), eframe::Error> {
    // Define native options for the application
    let native_options = eframe::NativeOptions::default();
    // Run the application
    eframe::run_native(
        "Hello World App",
        native_options,
        Box::new(|_cc| Box::new(FExpApp::default())),
    )
}

// Define the structure for the application
struct FExpApp {
    navigator: Navigator
}

impl Default for FExpApp {
    fn default() -> Self {
        Self {
            navigator: Navigator::new()
        }
    }
}

impl eframe::App for FExpApp {
    // The `update` method is called every frame to update the UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let files = list_directory_contents(Path::new(&self.navigator.current_path()));
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Files");
            for file in files {
                ui.label(file);
            }
        });
    }
}
