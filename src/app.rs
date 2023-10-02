/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "OpenDyslexic".to_owned(),
            value: 3.0,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OpenDyslexic");
            ui.horizontal(|ui| {
                ui.label("OpenDyslexic: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/gnostr-org/gnostr-opendyslexic/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "OD-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "opendyslexic/OpenDyslexic-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "OD-Italic".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "opendyslexic/OpenDyslexic-Italic.otf"
        )),
    );
    fonts.font_data.insert(
        "OD-Bold".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "opendyslexic/OpenDyslexic-Bold.otf"
        )),
    );
    fonts.font_data.insert(
        "OD-Bold-Italic".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "opendyslexic/OpenDyslexic-Bold-Italic.otf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "OD-Regular".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "OD-Regular".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Name("OpenDyslexic".into()))
        .or_default()
        .insert(0, "OD-Italic".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Name("OpenDyslexic".into()))
        .or_default()
        .insert(0, "OD-Regular".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Name("OpenDyslexic".into()))
        .or_default()
        .insert(0, "OD-Bold".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Name("OpenDyslexic".into()))
        .or_default()
        .insert(0, "OD-Bold-Italic".to_owned());

    ctx.set_fonts(fonts);
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "OpenDyslexic",
            "https://github.com/antijingoist/opendyslexic",
        );
        ui.label(".");
    });
}
