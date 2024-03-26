use egui::{Color32, TextStyle};
use egui_extras::{Size, StripBuilder};

use crate::parser::expression;

use super::parser::tokens::TokenStream;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    input: String,
    output: String, //#[serde(skip)] // This how you opt-out of serialization of a field
                    //value: f32,
}

impl TemplateApp {
    pub fn compute(&mut self) {
        let mut ts = match TokenStream::new(self.input.clone()) {
            Some(v) => v,
            None => {
                return;
            }
        };

        match expression(&mut ts) {
            Ok(ans) => self.output = format!("{}", ans),
            Err(e) => self.output = format!("{}", e),
        }
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            input: String::new(), //"write some expression here".to_owned(),
            output: "output is empty".to_owned(), //value: 2.7,
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

                ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let is_web = cfg!(target_arch = "wasm32");
            if is_web {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Simple Calculator");
                });
            }

            let dark_mode = ui.visuals().dark_mode;
            let faded_color = ui.visuals().window_fill();
            let faded_color = |color: Color32| -> Color32 {
                use egui::Rgba;
                let t = if dark_mode { 0.95 } else { 0.8 };
                egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
            };
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;

            StripBuilder::new(ui)
                .size(Size::exact(3.0 * body_text_size))
                .size(Size::remainder())
                .size(Size::exact(1.5 * body_text_size))
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder
                            .size(Size::relative(0.1))
                            .size(Size::remainder())
                            .size(Size::exact(30.0))
                            .size(Size::relative(0.1))
                            .horizontal(|mut strip| {
                                strip.empty();
                                strip.cell(|ui| {
                                    ui.horizontal_centered(|ui| {
                                        let response = ui.add(
                                            egui::TextEdit::singleline(&mut self.input)
                                                .desired_width(f32::INFINITY),
                                        );
                                        if response.changed() {
                                            self.output = String::from("response changed");
                                        }
                                        if response.lost_focus()
                                            && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                        {
                                            //self.output = String::from("enter key pressed");
                                            self.compute();
                                            response.request_focus();
                                        }
                                    });
                                });
                                strip.cell(|ui| {
                                    ui.horizontal_centered(|ui| {
                                        let btn_response = ui.add(egui::Button::new(" = "));
                                        if btn_response.clicked() {
                                            //self.output = String::from("click");
                                            self.compute();
                                        }
                                    });
                                });
                                strip.empty();
                            });
                    });
                    strip.cell(|ui| {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            0.0,
                            faded_color(Color32::BLUE),
                        );
                        ui.label(&self.output);
                    });
                    strip.cell(|ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                            ui.separator();
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                                egui::warn_if_debug_build(ui);
                                // <a target="_blank" href="https://icons8.com/icon/12780/calculator">Calculator</a> icon by <a target="_blank" href="https://icons8.com">Icons8</a>
                                ui.add(egui::Hyperlink::from_label_and_url(
                                    "Calculator",
                                    "https://icons8.com/icon/12780/calculator",
                                ));
                                ui.label("icon by");
                                ui.add(egui::Hyperlink::from_label_and_url(
                                    "Icons8",
                                    "https://icons8.com",
                                ));
                            });
                        });
                    })
                });
        });
    }
}
