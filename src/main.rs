use std::fs;
use std::path::PathBuf;
use eframe::egui::{self, CentralPanel};
use eframe::{self};
use egui::{ScrollArea, SidePanel, TopBottomPanel};
use icon::get_icon;
use navigation::Navigator;
use file_ops::{get_file_type, list_directory_contents, open_file};

pub mod file_ops;
pub mod navigation;
pub mod icon;

fn main() -> Result<(), eframe::Error> {
    // Define native options for the application
    let native_options = eframe::NativeOptions::default();
    // Run the application
    eframe::run_native(
        "File explorer",
        native_options,
        Box::new(|_cc| Ok(Box::new(FExpApp::default()))),
    )
}

// Define the structure for the application
struct FExpApp {
    navigator: Navigator,
    focussed_file: PathBuf
}

impl Default for FExpApp {
    fn default() -> Self {
        Self {
            navigator: Navigator::new(),
            focussed_file: PathBuf::default()
        }
    }
}

impl eframe::App for FExpApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        let files = list_directory_contents(&self.navigator.current_path());

        TopBottomPanel::top("topbar").exact_height(64.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("back").clicked() {
                    self.navigator.go_back_one();
                }
                if ui.button("forward").clicked() {
                    self.navigator.go_forward_one();
                }
            })
        });

        SidePanel::right("focussed_file_panel").exact_width(256.0).show(ctx, |ui| {
            ui.heading("Details");
            if let Some(filename) = self.focussed_file.file_name() {
                ui.label(format!("name: {}", filename.to_string_lossy()));
            }
            if let Ok(metadata) = fs::metadata(self.focussed_file.clone()) {
                ui.label(format!("size: {} bytes", metadata.len()));
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.navigator.current_path().to_string_lossy());

            ScrollArea::vertical().show(ui, |ui| {
                for (index, file) in files.iter().enumerate() {
                    ui.push_id(index, |ui| {
                        let full_path: PathBuf = self.navigator.current_path().join(file);
                        let file_type = get_file_type(&full_path);
                        let icon = get_icon(file_type);
                        let response = ui.horizontal(|ui| {
                           ui.add(
                                egui::Image::new(icon)
                                    .max_width(16.0)
                                    .rounding(1.0),
                           );

                           ui.label(file.clone());
                        }).response.interact(egui::Sense::click());

                        if response.clicked() {
                            self.focussed_file = self.navigator.current_path().join(file);
                        }

                        if response.double_clicked() {
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
