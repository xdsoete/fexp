use std::fs;
use std::path::PathBuf;

use eframe::egui::{self, CentralPanel};
use eframe::{self};
use navigation::Navigator;
use file_ops::{list_directory_contents, open_file};

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
        let files = list_directory_contents(&self.navigator.current_path());
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Files");
            for file in files {
                if ui.button(file.clone()).clicked() {
                    let full_path: PathBuf = self.navigator.current_path().join(file);
                    if let Ok(metadata) = fs::metadata(full_path.clone()) {
                        if metadata.is_file() {
                            open_file(&full_path);
                        } else if metadata.is_dir() {
                            self.navigator.navigate_to(&full_path);
                        }
                    }
                }
            }
        });
    }
}
