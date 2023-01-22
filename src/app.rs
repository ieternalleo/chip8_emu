use chip8::Chip8;
// We derive Deserialize/Serialize so we can persist app state on shutdown
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Chip8App {
    label: String,

    // Opt out of serialization of a member
    #[serde(skip)]
    value: f32,
    chip8: Chip8,
}

impl Default for Chip8App {
    fn default() -> Self {
        Self {
            label: "Chip8 Emulator".to_owned(),
            value: 0.0f32,
            chip8: Chip8::new(),
        }
    }
}

impl Chip8App {
    // Called once before the first frame
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // egui look & feel customization
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`

        // Load previous app state (if any)
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for Chip8App {
    // Called by the frame work to save state before shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            chip8,
        } = self;

        //  Examples of how to create different panels and windows
        //  Pick whichever suits you
        // Tip: a good default choice is just to keep the `CentralPanel`
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("( Dissassembly ) Side Panel");

            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let total_rows = 10_000;
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    for row in row_range {
                        let text = format!("Row {}/{}", row + 1, total_rows);
                        ui.label(text);
                    }
                });
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));

            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }
            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     ui.horizontal(|ui| {
            //         ui.spacing_mut().item_spacing.x = 0.0;
            //         ui.label("powered by ");
            //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            //         ui.label(" and ");
            //         ui.hyperlink_to(
            //             "eframe",
            //             "https://github.com/emilk/egui/tree/master/crates/eframe",
            //         );
            //         ui.label(".");
            //     });
            // });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Main Screen");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            egui::warn_if_debug_build(ui);
        });

        egui::SidePanel::right("side_panel_right")
            .show(ctx, |ui| ui.heading("Right Side Panel (CPU Information)"));
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
