mod util;

use eframe::egui;
use tracing::{error, info};

#[derive(Default)]
struct MyApp {
    input: String,
    result: String,
    error: Option<anyhow::Error>,
}

impl MyApp {
    fn run(options: eframe::NativeOptions) {
        let _ = eframe::run_native(
            "Json To Csv Converter",
            options,
            Box::new(|_cc| Ok(Box::<Self>::default())),
        );
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add(egui::Label::new(
                egui::RichText::new("Json To CSV converter")
                    .strong()
                    .size(32.0),
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new("convert").strong().size(26.0))
                    .clicked()
                {
                    match util::Json2CsvConverter::convert(&self.input) {
                        Ok(result) => {
                            self.result = result;
                        }
                        Err(e) => {
                            error!("{:?}", &e);
                            self.error = Some(e);
                        }
                    }
                }

                if ui
                    .button(egui::RichText::new("clear").strong().size(26.0))
                    .clicked()
                {
                    self.input = "".into();
                    self.result = "".into();
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Json Input").strong());
                        ui.add(
                            egui::TextEdit::multiline(&mut self.input)
                                .desired_width(ui.available_width() / 2.0),
                        );
                    });
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("CSV Output").strong());
                        // pass &str to multiline() makes it uneditable
                        ui.add(
                            egui::TextEdit::multiline(&mut self.result.as_str())
                                .desired_width(ui.available_width() / 2.0),
                        );
                    });
                });
            })
        });
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Application starts!");
    let options = eframe::NativeOptions::default();
    MyApp::run(options);
    Ok(())
}
