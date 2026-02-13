use eframe::egui;

pub struct Card<'a> {
    preview_content: Box<dyn FnOnce(&mut egui::Ui) + 'a>,
    details_content: Box<dyn FnOnce(&mut egui::Ui) + 'a>,
}

impl<'a> Card<'a> {
    pub fn new(
        preview_content: impl FnOnce(&mut egui::Ui) + 'a,
        details_content: impl FnOnce(&mut egui::Ui) + 'a,
    ) -> Self {
        Self {
            preview_content: Box::new(preview_content),
            details_content: Box::new(details_content),
        }
    }

    pub fn show(self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    (self.preview_content)(ui);
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        (self.details_content)(ui);
                    });
                });
            });
    }
}
