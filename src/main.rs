#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use simplelog::*;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let (log_widget, log_writer) = archaic_engine::app::logwidget::new_logger();
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![
        WriteLogger::new(LevelFilter::Debug, Config::default(), log_writer)
    ];
    #[cfg(debug_assertions)]
    loggers.push(TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto));
    let _ = CombinedLogger::init(
        loggers
    ).unwrap();
   
    let mut native_options = eframe::NativeOptions::default();
    native_options.maximized = true;
    native_options.drag_and_drop_support = true;
    eframe::run_native(
        "Archaic Engine",
        native_options,
        Box::new(|cc| Box::new(archaic_engine::App::new(cc, log_widget))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    //eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let (log_widget, log_writer) = archaic_engine::app::logwidget::new_logger();
    let _ = WriteLogger::new(LevelFilter::Debug, Config::default(), log_writer);
    
    let mut web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(archaic_engine::App::new(cc, log_widget))),
            )
            .await
            .expect("failed to start eframe");
    });
}
