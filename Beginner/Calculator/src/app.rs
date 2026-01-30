use eframe::{NativeOptions, egui};

use std::convert::TryFrom;

use crate::utilites::CalcKey;

enum Theme {
    Light,
    Dark,
}

// Our application state struct - NO LONGER deriving Default
pub struct MyApp {
    theme: Theme,
    input: String,
    result: String,
}

// Manually implement the Default trait for MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            input: "".into(),
            result: "".into(),
        }
    }
}

// Replace the impl eframe::App for MyApp block with this one demonstrating layout
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
            ui.collapsing("heading", |ui| {
                ui.label("hi");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = &[
                &[
                    CalcKey::Delete,
                    CalcKey::Clear,
                    CalcKey::Percentage,
                    CalcKey::Divide,
                ],
                &[
                    CalcKey::Number(7),
                    CalcKey::Number(8),
                    CalcKey::Number(9),
                    CalcKey::Multiply,
                ],
                &[
                    CalcKey::Number(4),
                    CalcKey::Number(5),
                    CalcKey::Number(6),
                    CalcKey::Subtract,
                ],
                &[
                    CalcKey::Number(1),
                    CalcKey::Number(2),
                    CalcKey::Number(3),
                    CalcKey::Add,
                ],
                &[
                    CalcKey::Reverse,
                    CalcKey::Number(0),
                    CalcKey::Dot,
                    CalcKey::Equal,
                ],
            ];

            ui.label(&self.input);
            egui::Grid::new("buttons")
                .num_columns(4)
                .spacing([10.0, 4.0])
                .show(ui, |ui| {
                    for row in layout {
                        for key in *row {
                            if draw_calc_button(ui, *key).clicked() {
                                match key {
                                    CalcKey::Number(n) => self.result += &n.to_string(),
                                    CalcKey::Delete => todo!(),
                                    CalcKey::Clear => todo!(),
                                    CalcKey::Percentage => todo!(),
                                    CalcKey::Dot => todo!(),
                                    CalcKey::Reverse => todo!(),
                                    CalcKey::Add => todo!(),
                                    CalcKey::Subtract => todo!(),
                                    CalcKey::Multiply => todo!(),
                                    CalcKey::Divide => todo!(),
                                    CalcKey::Equal => todo!(),
                                }
                            }
                        }

                        ui.end_row();
                    }
                })
        });
    } // End update
} // End impl

impl MyApp {
    pub fn run(options: NativeOptions) -> eframe::Result {
        eframe::run_native(
            "My gui app",
            options,
            Box::new(|_cc| Ok(Box::<Self>::default())),
        )
    }

    fn add(&mut self, n: u8) {
        todo!()
    }

    fn subtract(&mut self, n: u8) {
        todo!()
    }

    fn multiply(&mut self, n: u8) {
        todo!()
    }

    fn divide(&mut self, n: u8) {
        todo!()
    }
}

fn draw_calc_button(ui: &mut egui::Ui, key: CalcKey) -> egui::Response {
    let size = egui::vec2(48.0, 48.0);

    let (label, fill) = match key {
        CalcKey::Number(n) => (n.to_string(), egui::Color32::from_gray(60)),
        CalcKey::Delete => ("Del".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Clear => ("AC".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Percentage => ("%".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Dot => (".".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Reverse => ("±".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Add => ("+".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Subtract => ("−".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Multiply => ("×".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Divide => ("÷".to_string(), egui::Color32::from_gray(90)),
        CalcKey::Equal => ("=".to_string(), egui::Color32::from_gray(90)),
    };

    ui.add(
        egui::Button::new(
            egui::RichText::new(label)
                .color(egui::Color32::WHITE)
                .size(26.0),
        )
        .min_size(size)
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(48)),
    )
}
