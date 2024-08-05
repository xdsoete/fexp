use std::fs;
use std::path::PathBuf;
use eframe::egui::{self, CentralPanel};
use eframe::{self};
use egui::{Area, ScrollArea, TopBottomPanel};
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
        Box::new(|_cc| Ok(Box::new(FExpApp::default()))),
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

        TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("back").clicked() {
                    self.navigator.go_back_one();
                }
                if ui.button("forward").clicked() {
                    self.navigator.go_forward_one();
                }
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Files");
            ScrollArea::vertical().show(ui, |ui| {
                for (index, file) in files.iter().enumerate() {
                    ui.push_id(index, |ui| {
                        let response = ui.horizontal(|ui| {
                            ui.add(
                                egui::Image::new(egui::include_image!("./default-file.svg"))
                                    .max_width(16.0)
                                    .rounding(1.0),
                            );
                            ui.label(file.clone());
                        }).response.interact(egui::Sense::click());

                        if response.clicked() {
                            let full_path: PathBuf = self.navigator.current_path().join(file);
                            if let Ok(metadata) = fs::metadata(full_path.clone()) {
                                if metadata.is_file() {
                                    open_file(&full_path);
                                } else if metadata.is_dir() {
                                    self.navigator.navigate_to(&full_path);
                                }
                            }
                        }
                    });   
                }
            });
        });
    }
}
