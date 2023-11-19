use egui::{Widget, Vec2, Response, Rect, Label};

pub struct BottomBar {
}

impl Widget for BottomBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.set_max_size(Vec2::new(f32::MAX, 48.0));
        let label = Label::new("114514");
        label.ui(ui)
    }
}