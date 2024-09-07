use crate::file_ops::FileType;

pub fn get_icon(file_type: FileType) -> egui::ImageSource<'static> {
    match file_type {
        FileType::Rust => egui::include_image!("icons/rust.png"),
        FileType::Python => egui::include_image!("icons/python.png"),
        FileType::Text => egui::include_image!("icons/default-file.png"),
        FileType::Image => egui::include_image!("icons/image.png"),
        FileType::Folder => egui::include_image!("icons/folder.png"),
        FileType::Unknown => egui::include_image!("icons/default-file.png"),
    }
}
