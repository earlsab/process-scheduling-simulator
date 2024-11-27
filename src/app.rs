use egui_dropdown::DropDownBox;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    jobcount: u16,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    items: Vec<String>,
    buf: String,
    viewport_open: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            jobcount: 2,
            value: 2.7,
            items: vec![
                "First Come First Serve (FCFS)".into(),
                "Shortest Job Next (SJN)".into(),
                "Shortest Remaining Time (SRN)".into(),
                "Round Robin".into(),
            ],
            buf: String::new(),
            viewport_open: false,
        }
    }
}



impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Process Scheduling Simulator");

            ui.horizontal(|ui| {
                ui.label("Number of Jobs: ");
                ui.add(egui::DragValue::new(&mut self.jobcount).range(0..=u16::MAX));
            });

            ui.horizontal(|ui| {
                ui.label("Process Flags");
                // ui.text_edit_singleline(&mut self.label);
                ui.add(
                    DropDownBox::from_iter(
                        &self.items,
                        "test_dropbox",
                        &mut self.buf,
                        |ui, text| ui.selectable_label(false, text),
                    )
                    // choose whether to filter the box items based on what is in the text edit already
                    // default is true when this is not used
                    .filter_by_input(false)
                    // choose whether to select all text in the text edit when it gets focused
                    // default is false when this is not used
                    .select_on_focus(true)
                    // passes through the desired width to the text edit
                    // default is None internally, so TextEdit does whatever its default implements
                    .desired_width(250.0),
                );

                if ui.button("OK").clicked() {
                    self.viewport_open = true;
                }

                // TODO: Allow for User Closing
                if self.viewport_open {
                    let ctx_clone = ctx.clone();
                    ctx.show_viewport_deferred(
                        egui::ViewportId::from_hash_of(1),
                        egui::ViewportBuilder::default(),
                        move |_, _| {
                            // Define the UI for the new viewport here
                            egui::CentralPanel::default().show(&ctx_clone, |ui| {
                                job_builder_screen(ui);
                            });
                        },
                    );
                }

            });
        });
    }
}

fn job_builder_screen(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        powered_by_egui_and_eframe(ui);
        egui::warn_if_debug_build(ui);
    });
}

// fn process_scheduler() {

// }


fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
