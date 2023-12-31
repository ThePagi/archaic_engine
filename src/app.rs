pub mod logwidget;
mod style;

use egui::{Layout, Pos2, Color32, Stroke};
use log::{debug, error, info, warn};

use std::sync::mpsc;
use style::*;

pub const LOREM_IPSUM: &str = "Lorem 😏😏😏😏ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

type LoadedFile = (String, Vec<u8>);

fn open_file(sender: mpsc::Sender<LoadedFile>) {
    let future = async move {
        let file = rfd::AsyncFileDialog::new().pick_file().await;
        if let Some(file) = file {
            let data = file.read().await;
            sender
                .send((file.file_name(), data))
                .expect("File loading channel unexpectedly closed.");
        }
    };
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::spawn_local;
        spawn_local(future);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use futures::executor::block_on;
        block_on(future);
    }
}

pub struct App {
    // Example stuff:
    label: String,
    value: f32,
    show_inspector: bool,
    show_console: bool,
    avg_frametime: f32,
    file_load_rx: mpsc::Receiver<LoadedFile>,
    file_load_tx: mpsc::Sender<LoadedFile>,
    log_widget: logwidget::MyLogger,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, log_widget: logwidget::MyLogger) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        load_fonts(cc);
        cc.egui_ctx.set_style(style::style_dark());

        let (tx, rx) = mpsc::channel();
        let s = Self {
            avg_frametime: 0.016666,
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            show_inspector: false,
            show_console: false,
            file_load_rx: rx,
            file_load_tx: tx,
            log_widget,
        };
        debug!("This is a debug message.");
        info!("This is an info message.");
        warn!("This is a warning message!");
        error!("This is an error message!");
        s
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok((name, _data)) = self.file_load_rx.try_recv() {
            log::info!("{name}");
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open file…").clicked() {
                            open_file(self.file_load_tx.clone());
                        };

                        ui.hyperlink_to(
                            "Open Source Code",
                            "https://github.com/ThePagi/archaic_engine",
                        );

                        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.separator();
                    ui.toggle_value(&mut self.show_inspector, "🪛 Inspector");
                    ui.toggle_value(&mut self.show_console, "🖹 Console");
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::warn_if_debug_build(ui);
                    ui.label(build_time::build_time_local!("Built on %d.%m.%Y, %H:%M."));
                    ui.separator();
                    if let Some(usage) = frame.info().cpu_usage{
                        self.avg_frametime = self.avg_frametime*0.9+usage*0.1;
                        ui.label(format!("Frame time: {:.2}ms", self.avg_frametime*1000.0));
                    }
                });
            });
        });

        if self.show_inspector {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Side Panel");

                ui.horizontal(|ui| {
                    ui.label("Write something: ");
                    ui.text_edit_singleline(&mut self.label);
                });

                ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    self.value += 1.0;
                }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(LOREM_IPSUM.repeat(1));
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("powered by ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and ");
                        ui.hyperlink_to(
                            "eframe",
                            "https://github.com/emilk/egui/tree/master/crates/eframe",
                        );
                        ui.label(".");
                    });
                });
            });
        }
        if self.show_console {
            egui::TopBottomPanel::top("log console")
                .resizable(true)
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                        self.log_widget.show_log(ui)
                    });
                });
        }


        egui::CentralPanel::default().show(ctx, |ui| {
        egui::Frame::canvas(&ctx.style()).show(ui, |ui|{
            ui.painter().circle(Pos2::ZERO, 256.0, Color32::YELLOW, Stroke{ width: 2.0, color: Color32::BLUE });
        });

        ui.heading("Welcome to my life!");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.heading("Top Heading");
            ui.label(LOREM_IPSUM);
            ui.monospace(LOREM_IPSUM);
            ui.small(LOREM_IPSUM);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
