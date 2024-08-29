use crate::file_ops::FileType;

pub fn get_icon(file_type: FileType) -> egui::ImageSource<'static> {
    match file_type {
        FileType::Rust => egui::include_image!("rust.png"),
        FileType::Python => egui::include_image!("python.png"),
        FileType::Text => egui::include_image!("default-file.png"),
        FileType::Folder => egui::include_image!("folder.png"),
        FileType::Unknown => egui::include_image!("default-file.png"),
    }
}
