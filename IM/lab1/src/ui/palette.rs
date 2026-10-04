pub struct Palette;

impl Palette {
    pub fn empty() -> egui::Color32 {
        egui::Color32::from_rgb(30, 30, 30)
    }
    pub fn citizen() -> egui::Color32 {
        egui::Color32::BLACK
    }
    pub fn water() -> egui::Color32 {
        egui::Color32::from_rgb(30, 111, 255)
    }
    pub fn food() -> egui::Color32 {
        egui::Color32::from_rgb(46, 204, 64)
    }
    pub fn energy() -> egui::Color32 {
        egui::Color32::from_rgb(255, 212, 0)
    }
    pub fn population() -> egui::Color32 {
        egui::Color32::from_rgb(217, 217, 217)
    }
    pub fn conflict() -> egui::Color32 {
        egui::Color32::from_rgb(138, 43, 226)
    }
    pub fn overlap() -> egui::Color32 {
        egui::Color32::from_rgb(255, 122, 0)
    }
    pub fn anticell() -> egui::Color32 {
        egui::Color32::from_rgb(224, 27, 36)
    }
}