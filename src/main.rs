#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::CreationContext;
use evideo_editor::{app::load_texture, VideoEditor};
use std::env;

fn load_video_editor(cc: &CreationContext<'_>) -> Box<VideoEditor> {
    let mut video_editor = VideoEditor::new(cc);
    for texture in &video_editor.project.paths {
        load_texture(&mut video_editor.textures, &texture, &cc.egui_ctx)
    }
    return Box::new(video_editor)
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main_gui() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "evideo_editor",
        native_options,
        Box::new(|cc| load_video_editor(cc)),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main_gui() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| load_video_editor(cc)),
            )
            .await
            .expect("failed to start eframe");
    });
}

fn print_help() -> () {
    println!("Usage: video_editor [file_path] [options]");
    println!("\t file_path \t Path of the script that produces the video");
    println!("");
    println!("Options:");
    println!("\t--gui\t\t Open the application in gui mode");
}

#[derive(Debug, Clone)]
struct Arguments {
    file_path: Option<String>,
    gui_mode: bool,
}

fn parse_arguments() -> Option<Arguments> {
    let mut args: Vec<String> = env::args().rev().collect();

    if args.len() == 1 {
        return None;
    }

    args.pop();


    let mut arguments = Arguments{file_path: None, gui_mode: false};
    let arg = args.pop().unwrap();

    if !arg.starts_with("--") {
        arguments.file_path = Some(arg);
    } else {
        args.push(arg)
    }

    arguments.gui_mode = args.contains(&"--gui".to_string());

    return Some(arguments);
}

fn main() -> Result<(), std::io::Error> {
    let arguments = &parse_arguments();

    if arguments.is_none() {
        print_help();
        return Ok(());
    }

    if (*arguments).clone().unwrap().gui_mode {
         main_gui().unwrap();
    }

    dbg!(arguments.clone());

    return Ok(());
}
